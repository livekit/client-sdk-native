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

use super::remote_track;
use super::TrackInner;
use crate::prelude::*;
use libwebrtc::prelude::*;
use livekit_protocol as proto;
// use livekit_webrtc as rtc;
// use rtc::rtp_receiver::RtpReceiver;
use livekit_webrtc::prelude::*;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct RemoteVideoTrack {
    inner: Arc<TrackInner>,
}

impl Debug for RemoteVideoTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteVideoTrack")
            .field("sid", &self.sid())
            .field("name", &self.name())
            .field("source", &self.source())
            .finish()
    }
}

impl RemoteVideoTrack {
    pub(crate) fn new(
        sid: TrackSid,
        name: String,
        rtc_track: RtcVideoTrack,
        receiver: RtpReceiver
    ) -> Self {
        Self {
            inner: Arc::new(super::new_inner(
                sid,
                name,
                TrackKind::Video,
                MediaStreamTrack::Video(rtc_track),
                Some(receiver)
            )),
        }
    }

    pub fn sid(&self) -> TrackSid {
        self.inner.info.read().sid.clone()
    }

    pub fn name(&self) -> String {
        self.inner.info.read().name.clone()
    }

    pub fn kind(&self) -> TrackKind {
        self.inner.info.read().kind
    }

    pub fn source(&self) -> TrackSource {
        self.inner.info.read().source
    }

    pub fn stream_state(&self) -> StreamState {
        self.inner.info.read().stream_state
    }

    pub fn enable(&self) {
        self.inner.rtc_track.set_enabled(true);
    }

    pub fn disable(&self) {
        self.inner.rtc_track.set_enabled(false);
    }

    pub fn is_muted(&self) -> bool {
        self.inner.info.read().muted
    }

    pub fn rtc_track(&self) -> RtcVideoTrack {
        if let MediaStreamTrack::Video(video) = self.inner.rtc_track.clone() {
            return video;
        }
        unreachable!();
    }

    pub fn is_remote(&self) -> bool {
        true
    }

    pub fn on_muted(&self, f: impl Fn(Track) + Send + 'static) {
        *self.inner.events.muted.lock() = Some(Box::new(f));
    }

    pub fn on_unmuted(&self, f: impl Fn(Track) + Send + 'static) {
        *self.inner.events.unmuted.lock() = Some(Box::new(f));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn receiver(&self) -> Option<RtpReceiver> {
        self.inner.receiver.clone()
    }

    #[inline]
    pub(crate) fn transceiver(&self) -> Option<RtpTransceiver> {
        self.inner.info.read().transceiver.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn set_transceiver(&self, transceiver: Option<RtpTransceiver>) {
        self.inner.info.write().transceiver = transceiver;
    }

    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        remote_track::update_info(&self.inner, &Track::RemoteVideo(self.clone()), info);
    }
}
