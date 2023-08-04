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

use super::room::{FfiParticipant, FfiPublication, FfiTrack};
use super::{
    audio_source, audio_stream, room, video_source, video_stream, FfiConfig, FfiError, FfiResult,
    FfiServer,
};
use crate::proto;
use livekit::prelude::*;
use livekit::webrtc::native::{audio_resampler, yuv_helper};
use livekit::webrtc::prelude::*;
use livekit::webrtc::video_frame::{native::I420BufferExt, BoxVideoFrameBuffer, I420Buffer};
use parking_lot::Mutex;
use std::slice;
use std::sync::Arc;

/// This is the first request called by the foreign language
/// It sets the callback function to be called when an event is received
fn on_initialize(
    server: &'static FfiServer,
    init: proto::InitializeRequest,
) -> FfiResult<proto::InitializeResponse> {
    if server.config.lock().is_some() {
        return Err(FfiError::AlreadyInitialized);
    }

    log::info!("initializing ffi server v{}", env!("CARGO_PKG_VERSION"));

    // # SAFETY: The foreign language is responsible for ensuring that the callback function is valid
    *server.config.lock() = Some(FfiConfig {
        callback_fn: unsafe { std::mem::transmute(init.event_callback_ptr as usize) },
    });

    Ok(proto::InitializeResponse::default())
}

/// Dispose the server, close all rooms and clean up all handles
/// It is not mandatory to call this function.
fn on_dispose(
    server: &'static FfiServer,
    dispose: proto::DisposeRequest,
) -> FfiResult<proto::DisposeResponse> {
    *server.config.lock() = None;

    if !dispose.r#async {
        server.async_runtime.block_on(server.dispose());
        Ok(proto::DisposeResponse::default())
    } else {
        todo!("async dispose");
    }
}

/// Connect to a room, and start listening for events
/// The returned room_handle is used to interact with the room and to
/// recognized the incoming events
fn on_connect(
    server: &'static FfiServer,
    connect: proto::ConnectRequest,
) -> FfiResult<proto::ConnectResponse> {
    Ok(room::FfiRoom::connect(server, connect))
}

/// Disconnect to a room
/// This is an async function, the FfiClient must wait for the DisconnectCallback
fn on_disconnect(
    server: &'static FfiServer,
    disconnect: proto::DisconnectRequest,
) -> FfiResult<proto::DisconnectResponse> {
    let async_id = server.next_id();
    server.async_runtime.spawn(async move {
        let ffi_room = server
            .retrieve_handle::<room::FfiRoom>(disconnect.room_handle)
            .unwrap()
            .clone();

        ffi_room.close().await;

        let _ = server
            .send_event(proto::ffi_event::Message::Disconnect(
                proto::DisconnectCallback { async_id },
            ))
            .await;
    });

    Ok(proto::DisconnectResponse { async_id })
}

/// Publish a track to a room, and send a response to the FfiClient
/// The FfiClient musts wait for the LocalTrackPublication
fn on_publish_track(
    server: &'static FfiServer,
    publish: proto::PublishTrackRequest,
) -> FfiResult<proto::PublishTrackResponse> {
    let ffi_participant = server
        .retrieve_handle::<FfiParticipant>(publish.local_participant_handle)?
        .clone();

    Ok(ffi_participant.room.publish_track(server, publish))
}

fn on_unpublish_track(
    server: &'static FfiServer,
    unpublish: proto::UnpublishTrackRequest,
) -> FfiResult<proto::UnpublishTrackResponse> {
    let ffi_participant = server
        .retrieve_handle::<FfiParticipant>(unpublish.local_participant_handle)?
        .clone();

    Ok(ffi_participant.room.unpublish_track(server, unpublish))
}

/// Publish data to the room
fn on_publish_data(
    server: &'static FfiServer,
    publish: proto::PublishDataRequest,
) -> FfiResult<proto::PublishDataResponse> {
    // Push the data to an async queue (avoid blocking and keep the order)
    let ffi_participant =
        server.retrieve_handle::<FfiParticipant>(publish.local_participant_handle)?;

    ffi_participant.room.publish_data(server, publish)
}

/// Change the desired subscription state of a publication
fn on_set_subscribed(
    server: &'static FfiServer,
    set_subscribed: proto::SetSubscribedRequest,
) -> FfiResult<proto::SetSubscribedResponse> {
    let ffi_publication =
        server.retrieve_handle::<FfiPublication>(set_subscribed.publication_handle)?;

    let TrackPublication::Remote(publication) = &ffi_publication.publication else {
            return Err(FfiError::InvalidRequest("publication is not a RemotePublication".into()));
        };

    let _guard = server.async_runtime.enter();
    publication.set_subscribed(set_subscribed.subscribe);
    Ok(proto::SetSubscribedResponse {})
}

/// Create a new video track from a source
fn on_create_video_track(
    server: &'static FfiServer,
    create: proto::CreateVideoTrackRequest,
) -> FfiResult<proto::CreateVideoTrackResponse> {
    let source = server
        .retrieve_handle::<video_source::FfiVideoSource>(create.source_handle)?
        .source
        .clone();

    let handle_id = server.next_id();
    let video_track = LocalVideoTrack::create_video_track(&create.name, source);
    let ffi_track = FfiTrack {
        handle: handle_id,
        track: Track::LocalVideo(video_track),
    };

    let track_info = proto::TrackInfo::from(proto::FfiOwnedHandle { id: handle_id }, &ffi_track);

    server.store_handle(handle_id, ffi_track);

    Ok(proto::CreateVideoTrackResponse {
        track: Some(track_info),
    })
}

/// Create a new audio track from a source
fn on_create_audio_track(
    server: &'static FfiServer,
    create: proto::CreateAudioTrackRequest,
) -> FfiResult<proto::CreateAudioTrackResponse> {
    let source = server
        .retrieve_handle::<audio_source::FfiAudioSource>(create.source_handle)?
        .source
        .clone();

    let handle_id = server.next_id();
    let audio_track = LocalAudioTrack::create_audio_track(&create.name, source);
    let ffi_track = FfiTrack {
        handle: handle_id,
        track: Track::LocalAudio(audio_track),
    };
    let track_info = proto::TrackInfo::from(proto::FfiOwnedHandle { id: handle_id }, &ffi_track);

    server.store_handle(handle_id, ffi_track);

    Ok(proto::CreateAudioTrackResponse {
        track: Some(track_info),
    })
}

/// Allocate a new video buffer
fn on_alloc_video_buffer(
    server: &'static FfiServer,
    alloc: proto::AllocVideoBufferRequest,
) -> FfiResult<proto::AllocVideoBufferResponse> {
    let buffer: BoxVideoFrameBuffer = match alloc.r#type() {
        proto::VideoFrameBufferType::I420 => Box::new(I420Buffer::new(alloc.width, alloc.height)),
        _ => {
            return Err(FfiError::InvalidRequest(
                "frame type is not supported".into(),
            ))
        }
    };

    let handle_id = server.next_id();
    let buffer_info =
        proto::VideoFrameBufferInfo::from(proto::FfiOwnedHandle { id: handle_id }, &buffer);

    server.store_handle(handle_id, buffer);
    Ok(proto::AllocVideoBufferResponse {
        buffer: Some(buffer_info),
    })
}

/// Create a new VideoStream, a video stream is used to receive frames from a Track
fn on_new_video_stream(
    server: &'static FfiServer,
    new_stream: proto::NewVideoStreamRequest,
) -> FfiResult<proto::NewVideoStreamResponse> {
    let stream_info = video_stream::FfiVideoStream::setup(server, new_stream)?;
    Ok(proto::NewVideoStreamResponse {
        stream: Some(stream_info),
    })
}

/// Create a new video source, used to publish data to a track
fn on_new_video_source(
    server: &'static FfiServer,
    new_source: proto::NewVideoSourceRequest,
) -> FfiResult<proto::NewVideoSourceResponse> {
    let source_info = video_source::FfiVideoSource::setup(server, new_source)?;
    Ok(proto::NewVideoSourceResponse {
        source: Some(source_info),
    })
}

/// Push a frame to a source, libwebrtc will then decide if the frame should be dropped or not
/// The frame can also be adapted (resolution, cropped, ...)
fn on_capture_video_frame(
    server: &'static FfiServer,
    push: proto::CaptureVideoFrameRequest,
) -> FfiResult<proto::CaptureVideoFrameResponse> {
    let source = server.retrieve_handle::<video_source::FfiVideoSource>(push.source_handle)?;
    source.capture_frame(server, push)?;
    Ok(proto::CaptureVideoFrameResponse::default())
}

/// Convert a frame to I420
fn on_to_i420(
    server: &'static FfiServer,
    to_i420: proto::ToI420Request,
) -> FfiResult<proto::ToI420Response> {
    let from = to_i420
        .from
        .ok_or(FfiError::InvalidRequest("from is empty".into()))?;

    let i420 = match from {
        // Create a new I420 buffer from a raw argb buffer
        proto::to_i420_request::From::Argb(info) => {
            let argb = unsafe {
                let len = (info.stride * info.height) as usize;
                slice::from_raw_parts(info.ptr as *const u8, len)
            };

            let w = info.width as i32;
            let h = info.height as i32 * (if to_i420.flip_y { -1 } else { 1 });

            // Create a new I420 buffer
            let mut i420 = I420Buffer::new(info.width, info.height);
            let (sy, su, sv) = i420.strides();
            let (dy, du, dv) = i420.data_mut();

            match info.format() {
                proto::VideoFormatType::FormatArgb => {
                    yuv_helper::argb_to_i420(argb, info.stride, dy, sy, du, su, dv, sv, w, h)
                        .unwrap();
                }
                proto::VideoFormatType::FormatAbgr => {
                    yuv_helper::abgr_to_i420(argb, info.stride, dy, sy, du, su, dv, sv, w, h)
                        .unwrap();
                }
                _ => {
                    return Err(FfiError::InvalidRequest(
                        "the format is not supported".into(),
                    ))
                }
            }

            i420
        }
        // Convert an arbitrary yuv buffer to I420
        // Even if the buffer is already in I420, we're still copying it
        proto::to_i420_request::From::BufferHandle(handle) => server
            .retrieve_handle::<BoxVideoFrameBuffer>(handle)?
            .to_i420(),
    };

    let i420: BoxVideoFrameBuffer = Box::new(i420);
    let handle_id = server.next_id();
    let buffer_info =
        proto::VideoFrameBufferInfo::from(proto::FfiOwnedHandle { id: handle_id }, &i420);
    server.store_handle(handle_id, i420);
    Ok(proto::ToI420Response {
        buffer: Some(buffer_info),
    })
}

/// Convert a YUY buffer to argb
fn on_to_argb(
    server: &'static FfiServer,
    to_argb: proto::ToArgbRequest,
) -> FfiResult<proto::ToArgbResponse> {
    let buffer = server.retrieve_handle::<BoxVideoFrameBuffer>(to_argb.buffer_handle)?;

    let argb = unsafe {
        slice::from_raw_parts_mut(
            to_argb.dst_ptr as *mut u8,
            (to_argb.dst_stride * to_argb.dst_height) as usize,
        )
    };

    let w = to_argb.dst_width as i32;
    let h = to_argb.dst_height as i32 * (if to_argb.flip_y { -1 } else { 1 });

    let video_format = to_argb.dst_format();
    buffer
        .to_argb(video_format.into(), argb, to_argb.dst_stride, w, h)
        .unwrap();

    Ok(proto::ToArgbResponse::default())
}

/// Allocate a new audio buffer
fn on_alloc_audio_buffer(
    server: &'static FfiServer,
    alloc: proto::AllocAudioBufferRequest,
) -> FfiResult<proto::AllocAudioBufferResponse> {
    let frame = AudioFrame::new(
        alloc.sample_rate,
        alloc.num_channels,
        alloc.samples_per_channel,
    );

    let handle_id = server.next_id();
    let frame_info =
        proto::AudioFrameBufferInfo::from(proto::FfiOwnedHandle { id: handle_id }, &frame);
    server.store_handle(handle_id, frame);

    Ok(proto::AllocAudioBufferResponse {
        buffer: Some(frame_info),
    })
}

/// Create a new audio stream (used to receive audio frames from a track)
fn on_new_audio_stream(
    server: &'static FfiServer,
    new_stream: proto::NewAudioStreamRequest,
) -> FfiResult<proto::NewAudioStreamResponse> {
    let stream_info = audio_stream::FfiAudioStream::setup(server, new_stream)?;
    Ok(proto::NewAudioStreamResponse {
        stream: Some(stream_info),
    })
}

/// Create a new audio source (used to publish audio frames to a track)
fn on_new_audio_source(
    server: &'static FfiServer,
    new_source: proto::NewAudioSourceRequest,
) -> FfiResult<proto::NewAudioSourceResponse> {
    let source_info = audio_source::FfiAudioSource::setup(server, new_source)?;
    Ok(proto::NewAudioSourceResponse {
        source: Some(source_info),
    })
}

/// Push a frame to a source
fn on_capture_audio_frame(
    server: &'static FfiServer,
    push: proto::CaptureAudioFrameRequest,
) -> FfiResult<proto::CaptureAudioFrameResponse> {
    let source = server.retrieve_handle::<audio_source::FfiAudioSource>(push.source_handle)?;
    source.capture_frame(server, push)?;
    Ok(proto::CaptureAudioFrameResponse::default())
}

/// Create a new audio resampler
fn new_audio_resampler(
    server: &'static FfiServer,
    _: proto::NewAudioResamplerRequest,
) -> FfiResult<proto::NewAudioResamplerResponse> {
    let resampler = audio_resampler::AudioResampler::default();
    let resampler = Arc::new(Mutex::new(resampler));

    let handle_id = server.next_id();
    server.store_handle(handle_id, resampler);

    Ok(proto::NewAudioResamplerResponse {
        resampler: Some(proto::AudioResamplerInfo {
            handle: Some(proto::FfiOwnedHandle { id: handle_id }),
        }),
    })
}

/// Remix and resample an audio frame
fn remix_and_resample(
    server: &'static FfiServer,
    remix: proto::RemixAndResampleRequest,
) -> FfiResult<proto::RemixAndResampleResponse> {
    let resampler = server
        .retrieve_handle::<Arc<Mutex<audio_resampler::AudioResampler>>>(remix.resampler_handle)?
        .clone();

    let buffer = server.retrieve_handle::<AudioFrame>(remix.buffer_handle)?;
    let data = resampler
        .lock()
        .remix_and_resample(
            &buffer.data,
            buffer.samples_per_channel,
            buffer.num_channels,
            buffer.sample_rate,
            remix.num_channels,
            remix.sample_rate,
        )
        .to_owned();

    drop(buffer);

    let data_len = (data.len() / remix.num_channels as usize) as u32;
    let audio_frame = AudioFrame {
        data,
        num_channels: remix.num_channels,
        samples_per_channel: data_len,
        sample_rate: remix.sample_rate,
    };

    let handle_id = server.next_id();
    let buffer_info =
        proto::AudioFrameBufferInfo::from(proto::FfiOwnedHandle { id: handle_id }, &audio_frame);
    server.store_handle(handle_id, audio_frame);

    Ok(proto::RemixAndResampleResponse {
        buffer: Some(buffer_info),
    })
}

#[allow(clippy::field_reassign_with_default)] // Avoid uggly format
pub fn handle_request(
    server: &'static FfiServer,
    request: proto::FfiRequest,
) -> FfiResult<proto::FfiResponse> {
    let request = request
        .message
        .ok_or(FfiError::InvalidRequest("message is empty".into()))?;

    let mut res = proto::FfiResponse::default();

    res.message = Some(match request {
        proto::ffi_request::Message::Initialize(init) => {
            proto::ffi_response::Message::Initialize(on_initialize(server, init)?)
        }
        proto::ffi_request::Message::Dispose(dispose) => {
            proto::ffi_response::Message::Dispose(on_dispose(server, dispose)?)
        }
        proto::ffi_request::Message::Connect(connect) => {
            proto::ffi_response::Message::Connect(on_connect(server, connect)?)
        }
        proto::ffi_request::Message::Disconnect(disconnect) => {
            proto::ffi_response::Message::Disconnect(on_disconnect(server, disconnect)?)
        }
        proto::ffi_request::Message::PublishTrack(publish) => {
            proto::ffi_response::Message::PublishTrack(on_publish_track(server, publish)?)
        }
        proto::ffi_request::Message::UnpublishTrack(unpublish) => {
            proto::ffi_response::Message::UnpublishTrack(on_unpublish_track(server, unpublish)?)
        }
        proto::ffi_request::Message::PublishData(publish) => {
            proto::ffi_response::Message::PublishData(on_publish_data(server, publish)?)
        }
        proto::ffi_request::Message::SetSubscribed(subscribed) => {
            proto::ffi_response::Message::SetSubscribed(on_set_subscribed(server, subscribed)?)
        }
        proto::ffi_request::Message::CreateVideoTrack(create) => {
            proto::ffi_response::Message::CreateVideoTrack(on_create_video_track(server, create)?)
        }
        proto::ffi_request::Message::CreateAudioTrack(create) => {
            proto::ffi_response::Message::CreateAudioTrack(on_create_audio_track(server, create)?)
        }
        proto::ffi_request::Message::AllocVideoBuffer(alloc) => {
            proto::ffi_response::Message::AllocVideoBuffer(on_alloc_video_buffer(server, alloc)?)
        }
        proto::ffi_request::Message::NewVideoStream(new_stream) => {
            proto::ffi_response::Message::NewVideoStream(on_new_video_stream(server, new_stream)?)
        }
        proto::ffi_request::Message::NewVideoSource(new_source) => {
            proto::ffi_response::Message::NewVideoSource(on_new_video_source(server, new_source)?)
        }
        proto::ffi_request::Message::CaptureVideoFrame(push) => {
            proto::ffi_response::Message::CaptureVideoFrame(on_capture_video_frame(server, push)?)
        }
        proto::ffi_request::Message::ToI420(to_i420) => {
            proto::ffi_response::Message::ToI420(on_to_i420(server, to_i420)?)
        }
        proto::ffi_request::Message::ToArgb(to_argb) => {
            proto::ffi_response::Message::ToArgb(on_to_argb(server, to_argb)?)
        }
        proto::ffi_request::Message::AllocAudioBuffer(alloc) => {
            proto::ffi_response::Message::AllocAudioBuffer(on_alloc_audio_buffer(server, alloc)?)
        }
        proto::ffi_request::Message::NewAudioStream(new_stream) => {
            proto::ffi_response::Message::NewAudioStream(on_new_audio_stream(server, new_stream)?)
        }
        proto::ffi_request::Message::NewAudioSource(new_source) => {
            proto::ffi_response::Message::NewAudioSource(on_new_audio_source(server, new_source)?)
        }
        proto::ffi_request::Message::CaptureAudioFrame(push) => {
            proto::ffi_response::Message::CaptureAudioFrame(on_capture_audio_frame(server, push)?)
        }
        proto::ffi_request::Message::NewAudioResampler(new_res) => {
            proto::ffi_response::Message::NewAudioResampler(new_audio_resampler(server, new_res)?)
        }
        proto::ffi_request::Message::RemixAndResample(remix) => {
            proto::ffi_response::Message::RemixAndResample(remix_and_resample(server, remix)?)
        }
    });

    Ok(res)
}
