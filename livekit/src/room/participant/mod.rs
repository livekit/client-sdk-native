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

use crate::prelude::*;
use crate::rtc_engine::RtcEngine;
use livekit_protocol as proto;
use livekit_protocol::enum_dispatch;
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

mod local_participant;
mod remote_participant;

pub use local_participant::*;
pub use remote_participant::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Poor,
}

#[derive(Debug, Clone)]
pub enum Participant {
    Local(LocalParticipant),
    Remote(RemoteParticipant),
}

impl Participant {
    enum_dispatch!(
        [Local, Remote];
        pub fn sid(self: &Self) -> ParticipantSid;
        pub fn identity(self: &Self) -> ParticipantIdentity;
        pub fn name(self: &Self) -> String;
        pub fn metadata(self: &Self) -> String;
        pub fn is_speaking(self: &Self) -> bool;
        pub fn audio_level(self: &Self) -> f32;
        pub fn connection_quality(self: &Self) -> ConnectionQuality;

        pub(crate) fn update_info(self: &Self, info: proto::ParticipantInfo) -> ();

        // Internal functions called by the Room when receiving the associated signal messages
        pub(crate) fn set_speaking(self: &Self, speaking: bool) -> ();
        pub(crate) fn set_audio_level(self: &Self, level: f32) -> ();
        pub(crate) fn set_connection_quality(self: &Self, quality: ConnectionQuality) -> ();
        pub(crate) fn add_publication(self: &Self, publication: TrackPublication) -> ();
        pub(crate) fn remove_publication(self: &Self, sid: &TrackSid) -> ();
    );

    pub fn tracks(&self) -> HashMap<TrackSid, TrackPublication> {
        match self {
            Participant::Local(p) => p.internal_tracks(),
            Participant::Remote(p) => p.internal_tracks(),
        }
    }
}

struct ParticipantInfo {
    pub sid: ParticipantSid,
    pub identity: ParticipantIdentity,
    pub name: String,
    pub metadata: String,
    pub speaking: bool,
    pub audio_level: f32,
    pub connection_quality: ConnectionQuality,
}

type TrackMutedHandler = Box<dyn Fn(Participant, TrackPublication, Track) + Send>;
type TrackUnmutedHandler = Box<dyn Fn(Participant, TrackPublication, Track) + Send>;

#[derive(Default)]
struct ParticipantEvents {
    track_muted: Mutex<Option<TrackMutedHandler>>,
    track_unmuted: Mutex<Option<TrackUnmutedHandler>>,
}

pub(super) struct ParticipantInner {
    rtc_engine: Arc<RtcEngine>,
    info: RwLock<ParticipantInfo>,
    tracks: RwLock<HashMap<TrackSid, TrackPublication>>,
    events: Arc<ParticipantEvents>,
}

pub(super) fn new_inner(
    rtc_engine: Arc<RtcEngine>,
    sid: ParticipantSid,
    identity: ParticipantIdentity,
    name: String,
    metadata: String,
) -> Arc<ParticipantInner> {
    Arc::new(ParticipantInner {
        rtc_engine,
        info: RwLock::new(ParticipantInfo {
            sid,
            identity,
            name,
            metadata,
            speaking: false,
            audio_level: 0.0,
            connection_quality: ConnectionQuality::Excellent,
        }),
        tracks: Default::default(),
        events: Default::default(),
    })
}

pub(super) fn update_info(
    inner: &Arc<ParticipantInner>,
    _participant: &Participant,
    new_info: proto::ParticipantInfo,
) {
    let mut info = inner.info.write();
    info.sid = new_info.sid.try_into().unwrap();
    info.name = new_info.name;
    info.identity = new_info.identity.into();
    info.metadata = new_info.metadata; // TODO(theomonnom): callback MetadataChanged
}

pub(super) fn set_speaking(
    inner: &Arc<ParticipantInner>,
    _participant: &Participant,
    speaking: bool,
) {
    inner.info.write().speaking = speaking;
}

pub(super) fn set_audio_level(
    inner: &Arc<ParticipantInner>,
    _participant: &Participant,
    audio_level: f32,
) {
    inner.info.write().audio_level = audio_level;
}

pub(super) fn set_connection_quality(
    inner: &Arc<ParticipantInner>,
    _participant: &Participant,
    quality: ConnectionQuality,
) {
    inner.info.write().connection_quality = quality;
}

pub(super) fn remove_publication(
    inner: &Arc<ParticipantInner>,
    _participant: &Participant,
    sid: &TrackSid,
) -> Option<TrackPublication> {
    let mut tracks = inner.tracks.write();
    let publication = tracks.remove(sid);
    if let Some(publication) = publication.clone() {
        // remove events
        publication.on_muted(|_, _| {});
        publication.on_unmuted(|_, _| {});
    } else {
        // shouldn't happen (internal)
        log::warn!("could not find publication to remove: {:?}", sid);
    }

    publication
}

pub(super) fn add_publication(
    inner: &Arc<ParticipantInner>,
    participant: &Participant,
    publication: TrackPublication,
) {
    let mut tracks = inner.tracks.write();
    tracks.insert(publication.sid(), publication.clone());

    let events = inner.events.clone();
    let particiant = participant.clone();
    publication.on_muted(move |publication, track| {
        if let Some(cb) = events.track_muted.lock().as_ref() {
            cb(particiant.clone(), publication, track);
        }
    });

    let events = inner.events.clone();
    let participant = participant.clone();
    publication.on_unmuted(move |publication, track| {
        if let Some(cb) = events.track_unmuted.lock().as_ref() {
            cb(participant.clone(), publication, track);
        }
    });
}
