// Copyright 2023 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::ConnectionQuality;
use super::ParticipantInner;
use crate::options;
use crate::options::compute_video_encodings;
use crate::options::video_layers_from_encodings;
use crate::options::TrackPublishOptions;
use crate::prelude::*;
use crate::rtc_engine::RtcEngine;
use crate::DataPacketKind;
use livekit_protocol as proto;
use livekit_webrtc::rtp_parameters::RtpEncodingParameters;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

type LocalTrackPublishedHandler = Box<dyn Fn(LocalParticipant, LocalTrackPublication) + Send>;
type LocalTrackUnpublishedHandler = Box<dyn Fn(LocalParticipant, LocalTrackPublication) + Send>;

#[derive(Default)]
struct LocalEvents {
    local_track_published: Mutex<Option<LocalTrackPublishedHandler>>,
    local_track_unpublished: Mutex<Option<LocalTrackUnpublishedHandler>>,
}

struct LocalInfo {
    events: LocalEvents,
}

#[derive(Clone)]
pub struct LocalParticipant {
    inner: Arc<ParticipantInner>,
    local: Arc<LocalInfo>,
}

impl Debug for LocalParticipant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalParticipant")
            .field("sid", &self.sid())
            .field("identity", &self.identity())
            .field("name", &self.name())
            .finish()
    }
}

impl LocalParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RtcEngine>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
    ) -> Self {
        Self {
            inner: super::new_inner(rtc_engine, sid, identity, name, metadata),
            local: Arc::new(LocalInfo {
                events: LocalEvents::default(),
            }),
        }
    }

    pub(crate) fn internal_tracks(&self) -> HashMap<TrackSid, TrackPublication> {
        self.inner.tracks.read().clone()
    }

    pub(crate) fn update_info(&self, info: proto::ParticipantInfo) {
        super::update_info(&self.inner, &Participant::Local(self.clone()), info);
    }

    pub(crate) fn set_speaking(&self, speaking: bool) {
        super::set_speaking(&self.inner, &Participant::Local(self.clone()), speaking);
    }

    pub(crate) fn set_audio_level(&self, level: f32) {
        super::set_audio_level(&self.inner, &Participant::Local(self.clone()), level);
    }

    pub(crate) fn set_connection_quality(&self, quality: ConnectionQuality) {
        super::set_connection_quality(&self.inner, &Participant::Local(self.clone()), quality);
    }

    #[allow(dead_code)]
    pub(crate) fn on_local_track_published(
        &self,
        handler: impl Fn(LocalParticipant, LocalTrackPublication) + Send + 'static,
    ) {
        *self.local.events.local_track_published.lock() = Some(Box::new(handler));
    }

    #[allow(dead_code)]
    pub(crate) fn on_local_track_unpublished(
        &self,
        handler: impl Fn(LocalParticipant, LocalTrackPublication) + Send + 'static,
    ) {
        *self.local.events.local_track_unpublished.lock() = Some(Box::new(handler));
    }

    pub(crate) fn add_publication(&self, publication: TrackPublication) {
        super::add_publication(&self.inner, &Participant::Local(self.clone()), publication);
    }

    #[allow(dead_code)]
    pub(crate) fn remove_publication(&self, sid: &TrackSid) {
        super::remove_publication(&self.inner, &Participant::Local(self.clone()), sid);
    }

    pub(crate) fn published_tracks_info(&self) -> Vec<proto::TrackPublishedResponse> {
        let tracks = self.tracks();
        let mut vec = Vec::with_capacity(tracks.len());

        for p in tracks.values() {
            if let Some(track) = p.track() {
                vec.push(proto::TrackPublishedResponse {
                    cid: track.rtc_track().id(),
                    track: Some(p.proto_info()),
                });
            }
        }

        vec
    }

    pub async fn publish_track(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
    ) -> RoomResult<LocalTrackPublication> {
        let mut req = proto::AddTrackRequest {
            cid: track.rtc_track().id(),
            name: track.name(),
            r#type: proto::TrackType::from(track.kind()) as i32,
            muted: track.is_muted(),
            source: proto::TrackSource::from(options.source) as i32,
            disable_dtx: !options.dtx,
            disable_red: !options.red,
            ..Default::default()
        };

        let mut encodings = Vec::default();
        match &track {
            LocalTrack::Video(video_track) => {
                // Get the video dimension
                // TODO(theomonnom): Use MediaStreamTrack::getSettings() on web
                let resolution = video_track.rtc_source().video_resolution();
                req.width = resolution.width;
                req.height = resolution.height;

                encodings = compute_video_encodings(req.width, req.height, &options);
                req.layers = video_layers_from_encodings(req.width, req.height, &encodings);
            }
            LocalTrack::Audio(_audio_track) => {
                // Setup audio encoding
                let audio_encoding = options
                    .audio_encoding
                    .as_ref()
                    .unwrap_or(&options::audio::SPEECH.encoding);

                encodings.push(RtpEncodingParameters {
                    max_bitrate: Some(audio_encoding.max_bitrate),
                    ..Default::default()
                });
            }
        }
        let track_info = self.inner.rtc_engine.add_track(req).await?;
        let publication = LocalTrackPublication::new(track_info.clone(), track.clone());
        track.update_info(track_info); // Update sid + source

        log::debug!("publishing track with cid {:?}", track.rtc_track().id());
        let transceiver = self
            .inner
            .rtc_engine
            .create_sender(track.clone(), options.clone(), encodings)
            .await?;

        track.set_transceiver(Some(transceiver));
        track.enable();

        self.inner.rtc_engine.publisher_negotiation_needed();

        publication.update_publish_options(options);
        self.add_publication(TrackPublication::Local(publication.clone()));

        if let Some(local_track_published) = self.local.events.local_track_published.lock().as_ref()
        {
            local_track_published(self.clone(), publication.clone());
        }

        Ok(publication)
    }

    pub async fn unpublish_track(
        &self,
        track: &TrackSid,
        // _stop_on_unpublish: bool,
    ) -> RoomResult<LocalTrackPublication> {
        let publication = self.inner.tracks.write().remove(track);
        if let Some(TrackPublication::Local(publication)) = publication {
            let track = publication.track().unwrap();
            let sender = track.transceiver().unwrap().sender();

            self.inner.rtc_engine.remove_track(sender).await?;
            track.set_transceiver(None);

            if let Some(local_track_unpublished) =
                self.local.events.local_track_unpublished.lock().as_ref()
            {
                local_track_unpublished(self.clone(), publication.clone());
            }

            publication.set_track(None);
            self.inner.rtc_engine.publisher_negotiation_needed();

            Ok(publication)
        } else {
            Err(RoomError::Internal("track not found".to_string()))
        }
    }

    pub async fn publish_data(
        &self,
        data: Vec<u8>,
        kind: DataPacketKind,
        destination_sids: Vec<String>,
    ) -> RoomResult<()> {
        let data = proto::DataPacket {
            kind: kind as i32,
            value: Some(proto::data_packet::Value::User(proto::UserPacket {
                payload: data,
                destination_sids: destination_sids.to_owned(),
                ..Default::default()
            })),
        };

        self.inner
            .rtc_engine
            .publish_data(&data, kind)
            .await
            .map_err(Into::into)
    }

    pub fn get_track_publication(&self, sid: &TrackSid) -> Option<LocalTrackPublication> {
        self.inner.tracks.read().get(sid).map(|track| {
            if let TrackPublication::Local(local) = track {
                return local.clone();
            }

            unreachable!()
        })
    }

    pub fn sid(&self) -> ParticipantSid {
        self.inner.info.read().sid.clone()
    }

    pub fn identity(&self) -> ParticipantIdentity {
        self.inner.info.read().identity.clone()
    }

    pub fn name(&self) -> String {
        self.inner.info.read().name.clone()
    }

    pub fn metadata(&self) -> String {
        self.inner.info.read().metadata.clone()
    }

    pub fn is_speaking(&self) -> bool {
        self.inner.info.read().speaking
    }

    pub fn tracks(&self) -> HashMap<TrackSid, LocalTrackPublication> {
        self.inner
            .tracks
            .read()
            .clone()
            .into_iter()
            .map(|(sid, track)| {
                if let TrackPublication::Local(local) = track {
                    return (sid, local);
                }

                unreachable!()
            })
            .collect()
    }

    pub fn audio_level(&self) -> f32 {
        self.inner.info.read().audio_level
    }

    pub fn connection_quality(&self) -> ConnectionQuality {
        self.inner.info.read().connection_quality
    }
}
