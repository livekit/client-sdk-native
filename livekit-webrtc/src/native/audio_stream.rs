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

use crate::audio_frame::AudioFrame;
use crate::audio_track::RtcAudioTrack;
use cxx::SharedPtr;
use futures::stream::Stream;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use webrtc_sys::audio_track as sys_at;

pub struct NativeAudioStream {
    native_sink: SharedPtr<sys_at::ffi::NativeAudioSink>,
    audio_track: RtcAudioTrack,
    frame_rx: mpsc::UnboundedReceiver<AudioFrame>,
}

impl NativeAudioStream {
    pub fn new(audio_track: RtcAudioTrack) -> Self {
        let (frame_tx, frame_rx) = mpsc::unbounded_channel();
        let observer = Arc::new(AudioTrackObserver { frame_tx });
        let native_sink = sys_at::ffi::new_native_audio_sink(Box::new(
            sys_at::AudioSinkWrapper::new(observer.clone()),
        ));

        let audio = unsafe { sys_at::ffi::media_to_audio(audio_track.sys_handle()) };
        audio.add_sink(&native_sink);

        Self {
            native_sink,
            audio_track,
            frame_rx,
        }
    }

    pub fn track(&self) -> RtcAudioTrack {
        self.audio_track.clone()
    }

    pub fn close(&mut self) {
        let audio = unsafe { sys_at::ffi::media_to_audio(self.audio_track.sys_handle()) };
        audio.remove_sink(&self.native_sink);

        self.frame_rx.close();
    }
}

impl Drop for NativeAudioStream {
    fn drop(&mut self) {
        self.close();
    }
}

impl Stream for NativeAudioStream {
    type Item = AudioFrame;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.frame_rx.poll_recv(cx)
    }
}

pub struct AudioTrackObserver {
    frame_tx: mpsc::UnboundedSender<AudioFrame>,
}

impl sys_at::AudioSink for AudioTrackObserver {
    fn on_data(&self, data: &[i16], sample_rate: i32, nb_channels: usize, nb_frames: usize) {
        // TODO(theomonnom): Should we avoid copy here?
        let _ = self.frame_tx.send(AudioFrame {
            data: data.to_owned(),
            sample_rate: sample_rate as u32,
            num_channels: nb_channels as u32,
            samples_per_channel: nb_frames as u32,
        });
    }
}
