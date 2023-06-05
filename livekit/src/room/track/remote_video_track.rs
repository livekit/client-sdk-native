use super::TrackInner;
use crate::prelude::*;
use livekit_protocol as proto;
// use livekit_webrtc as rtc;
// use rtc::rtp_receiver::RtpReceiver;
use livekit_webrtc::prelude::*;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct RemoteVideoTrack {
    pub(crate) inner: Arc<TrackInner>,
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
            inner: Arc::new(TrackInner::new(
                sid,
                name,
                TrackKind::Video,
                MediaStreamTrack::Video(rtc_track),
                Some(receiver)
            )),
        }
    }

    #[inline]
    pub fn sid(&self) -> TrackSid {
        self.inner.sid()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    pub fn kind(&self) -> TrackKind {
        self.inner.kind()
    }

    #[inline]
    pub fn source(&self) -> TrackSource {
        self.inner.source()
    }

    #[inline]
    pub fn stream_state(&self) -> StreamState {
        self.inner.stream_state()
    }

    #[inline]
    pub fn start(&self) {
        self.inner.start()
    }

    #[inline]
    pub fn stop(&self) {
        self.inner.stop()
    }

    #[inline]
    pub fn is_muted(&self) -> bool {
        self.inner.is_muted()
    }

    #[inline]
    pub fn set_muted(&self, muted: bool) {
        self.inner.set_muted(muted)
    }

    #[inline]
    pub fn rtc_track(&self) -> RtcVideoTrack {
        if let MediaStreamTrack::Video(video) = self.inner.rtc_track() {
            return video;
        }
        unreachable!()
    }

    #[inline]
    pub fn register_observer(&self) -> mpsc::UnboundedReceiver<TrackEvent> {
        self.inner.register_observer()
    }

    #[inline]
    pub fn is_remote(&self) -> bool {
        true
    }

    #[allow(dead_code)]
    #[inline]
    pub fn receiver(&self) -> Option<RtpReceiver> {
        self.inner.receiver.clone()
    }

    #[inline]
    pub(crate) fn transceiver(&self) -> Option<RtpTransceiver> {
        self.inner.transceiver()
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn update_transceiver(&self, transceiver: Option<RtpTransceiver>) {
        self.inner.update_transceiver(transceiver)
    }

    #[inline]
    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        self.inner.update_info(info);
    }
}
