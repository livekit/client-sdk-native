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

use crate::imp::audio_track as imp_at;
use crate::media_stream_track::media_stream_track;
use crate::media_stream_track::RtcTrackState;
use std::fmt::Debug;

#[derive(Clone)]
pub struct RtcAudioTrack {
    pub(crate) handle: imp_at::RtcAudioTrack,
}

impl RtcAudioTrack {
    media_stream_track!();
}

impl Debug for RtcAudioTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RtcAudioTrack")
            .field("id", &self.id())
            .field("enabled", &self.enabled())
            .field("state", &self.state())
            .finish()
    }
}
