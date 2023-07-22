use super::TrackKind;
use super::{ConnectionQuality, ParticipantInner};
use crate::prelude::*;
use crate::rtc_engine::RtcEngine;
use crate::track::TrackError;
use livekit_protocol as proto;
use livekit_webrtc::prelude::*;
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

const ADD_TRACK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Default)]
struct RemoteEvents {
    track_published: Mutex<Option<Box<dyn Fn(RemoteParticipant, RemoteTrackPublication) + Send>>>,
    track_unpublished: Mutex<Option<Box<dyn Fn(RemoteParticipant, RemoteTrackPublication) + Send>>>,
    track_subscribed:
        Mutex<Option<Box<dyn Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack) + Send>>>,
    track_unsubscribed:
        Mutex<Option<Box<dyn Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack) + Send>>>,
    track_subscription_failed:
        Mutex<Option<Box<dyn Fn(RemoteParticipant, TrackSid, TrackError) + Send>>>,
}

struct RemoteInfo {
    events: Arc<RemoteEvents>,
}

#[derive(Clone)]
pub struct RemoteParticipant {
    inner: Arc<ParticipantInner>,
    remote: Arc<RemoteInfo>,
}

impl Debug for RemoteParticipant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteParticipant")
            .field("sid", &self.sid())
            .field("identity", &self.identity())
            .field("name", &self.name())
            .finish()
    }
}

impl RemoteParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RtcEngine>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
    ) -> Self {
        Self {
            inner: super::new_inner(rtc_engine, sid, identity, name, metadata),
            remote: Arc::new(RemoteInfo {
                events: Default::default(),
            }),
        }
    }

    pub(crate) fn internal_tracks(&self) -> HashMap<TrackSid, TrackPublication> {
        self.inner.tracks.read().clone()
    }

    pub(crate) async fn add_subscribed_media_track(
        &self,
        sid: TrackSid,
        media_track: MediaStreamTrack,
    ) {
        let wait_publication = {
            let participant = self.clone();
            let sid = sid.clone();
            async move {
                loop {
                    let publication = participant.get_track_publication(&sid);
                    if let Some(publication) = publication {
                        return publication;
                    }

                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        };

        if let Ok(remote_publication) = timeout(ADD_TRACK_TIMEOUT, wait_publication).await {
            let track = match remote_publication.kind() {
                TrackKind::Audio => {
                    if let MediaStreamTrack::Audio(rtc_track) = media_track {
                        let audio_track = RemoteAudioTrack::new(
                            remote_publication.sid().into(),
                            remote_publication.name(),
                            rtc_track,
                        );
                        RemoteTrack::Audio(audio_track)
                    } else {
                        unreachable!();
                    }
                }
                TrackKind::Video => {
                    if let MediaStreamTrack::Video(rtc_track) = media_track {
                        let video_track = RemoteVideoTrack::new(
                            remote_publication.sid().into(),
                            remote_publication.name(),
                            rtc_track,
                        );
                        RemoteTrack::Video(video_track)
                    } else {
                        unreachable!()
                    }
                }
            };

            log::debug!("starting track: {:?}", sid);

            //track.set_muted(remote_publication.is_muted());
            track.update_info(proto::TrackInfo {
                sid: remote_publication.sid().to_string(),
                name: remote_publication.name().to_string(),
                r#type: proto::TrackType::from(remote_publication.kind()) as i32,
                source: proto::TrackSource::from(remote_publication.source()) as i32,
                ..Default::default()
            });

            self.add_publication(TrackPublication::Remote(remote_publication.clone()));
            track.enable();

            remote_publication.set_track(Some(track.into())); // This will fire TrackSubscribed on the publication
        } else {
            log::error!("could not find published track with sid: {:?}", sid);

            if let Some(track_subscription_failed) =
                self.remote.events.track_subscription_failed.lock().as_ref()
            {
                track_subscription_failed(
                    self.clone(),
                    sid.clone(),
                    TrackError::TrackNotFound(sid.0),
                );
            }
        }
    }

    pub(crate) fn unpublish_track(&self, sid: &TrackSid) {
        if let Some(publication) = self.get_track_publication(sid) {
            // Unsubscribe to the track if needed
            if let Some(track) = publication.track() {
                track.disable();
                publication.set_track(None); // This will fire TrackUnsubscribed on the publication
            }

            self.remove_publication(sid);

            if let Some(track_unpublished) = self.remote.events.track_unpublished.lock().as_ref() {
                track_unpublished(self.clone(), publication.clone());
            }
        }
    }

    pub(crate) fn update_info(&self, info: proto::ParticipantInfo) {
        super::update_info(
            &self.inner,
            &Participant::Remote(self.clone()),
            info.clone(),
        );

        let mut valid_tracks = HashSet::<TrackSid>::new();
        for track in info.tracks {
            if let Some(publication) = self.get_track_publication(&track.sid.clone().into()) {
                publication.update_info(track.clone());
            } else {
                let publication = RemoteTrackPublication::new(track.clone(), None);

                self.add_publication(TrackPublication::Remote(publication.clone()));

                // This is a new track, dispatch publish event
                if let Some(track_published) = self.remote.events.track_published.lock().as_ref() {
                    track_published(self.clone(), publication);
                }
            }

            valid_tracks.insert(track.sid.into());
        }

        // remove tracks that are no longer valid
        let tracks = self.inner.tracks.read().clone();
        for (sid, _) in &tracks {
            if valid_tracks.contains(sid) {
                continue;
            }

            self.unpublish_track(sid);
        }
    }

    pub(crate) fn on_track_published(
        &self,
        track_published: impl Fn(RemoteParticipant, RemoteTrackPublication) + Send + 'static,
    ) {
        *self.remote.events.track_published.lock() = Some(Box::new(track_published));
    }

    pub(crate) fn on_track_unpublished(
        &self,
        track_unpublished: impl Fn(RemoteParticipant, RemoteTrackPublication) + Send + 'static,
    ) {
        *self.remote.events.track_unpublished.lock() = Some(Box::new(track_unpublished));
    }

    pub(crate) fn on_track_subscribed(
        &self,
        track_subscribed: impl Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack)
            + Send
            + 'static,
    ) {
        *self.remote.events.track_subscribed.lock() = Some(Box::new(track_subscribed));
    }

    pub(crate) fn on_track_unsubscribed(
        &self,
        track_unsubscribed: impl Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack)
            + Send
            + 'static,
    ) {
        *self.remote.events.track_unsubscribed.lock() = Some(Box::new(track_unsubscribed));
    }

    pub(crate) fn on_track_subscription_failed(
        &self,
        track_subscription_failed: impl Fn(RemoteParticipant, TrackSid, TrackError) + Send + 'static,
    ) {
        *self.remote.events.track_subscription_failed.lock() =
            Some(Box::new(track_subscription_failed));
    }

    pub(crate) fn set_speaking(&self, speaking: bool) {
        super::set_speaking(&self.inner, &Participant::Remote(self.clone()), speaking);
    }

    pub(crate) fn set_audio_level(&self, level: f32) {
        super::set_audio_level(&self.inner, &Participant::Remote(self.clone()), level);
    }

    pub(crate) fn set_connection_quality(&self, quality: ConnectionQuality) {
        super::set_connection_quality(&self.inner, &Participant::Remote(self.clone()), quality);
    }

    pub(crate) fn add_publication(&self, publication: TrackPublication) {
        super::add_publication(
            &self.inner,
            &Participant::Remote(self.clone()),
            publication.clone(),
        );

        let TrackPublication::Remote(publication) = publication else {
            panic!("expected remote publication");
        };

        publication.on_subscription_update_needed({
            let rtc_engine = self.inner.rtc_engine.clone();
            let psid = self.sid().0.clone();
            move |publication, subscribed| {
                let rtc_engine = rtc_engine.clone();
                let psid = psid.clone();
                tokio::spawn(async move {
                    let tsid = publication.sid().0.clone();
                    let update_subscription = proto::UpdateSubscription {
                        track_sids: vec![tsid.clone()],
                        subscribe: subscribed,
                        participant_tracks: vec![proto::ParticipantTracks {
                            participant_sid: psid,
                            track_sids: vec![tsid.clone()],
                        }],
                    };

                    let _ = rtc_engine
                        .send_request(proto::signal_request::Message::Subscription(
                            update_subscription,
                        ))
                        .await;
                });
            }
        });

        publication.on_subscribed({
            let events = self.remote.events.clone();
            let participant = self.clone();
            move |publication, track| {
                if let Some(track_subscribed) = events.track_subscribed.lock().as_ref() {
                    track_subscribed(participant.clone(), publication, track);
                }
            }
        });

        publication.on_unsubscribed({
            let events = self.remote.events.clone();
            let participant = self.clone();
            move |publication, track| {
                if let Some(track_unsubscribed) = events.track_unsubscribed.lock().as_ref() {
                    track_unsubscribed(participant.clone(), publication, track);
                }
            }
        });
    }

    pub(crate) fn remove_publication(&self, sid: &TrackSid) {
        let publication =
            super::remove_publication(&self.inner, &Participant::Remote(self.clone()), sid);

        if let Some(publication) = publication {
            let TrackPublication::Remote(publication) = publication else {
                panic!("expected remote publication");
            };

            publication.on_subscription_update_needed(|_, _| {});
            publication.on_subscribed(|_, _| {});
            publication.on_unsubscribed(|_, _| {});
        }
    }

    pub fn get_track_publication(&self, sid: &TrackSid) -> Option<RemoteTrackPublication> {
        self.inner.tracks.read().get(sid).map(|track| {
            if let TrackPublication::Remote(remote) = track {
                return remote.clone();
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

    pub fn tracks(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
        self.inner
            .tracks
            .read()
            .clone()
            .into_iter()
            .map(|(sid, track)| {
                if let TrackPublication::Remote(remote) = track {
                    return (sid, remote);
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
