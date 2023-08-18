use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit")]
pub mod ffi {

    extern "C++" {
        include!("livekit/webrtc.h");
        include!("livekit/rtp_parameters.h");
        include!("livekit/helper.h");
        include!("livekit/media_stream.h");
        include!("livekit/frame_transformer.h");

        type MediaType = crate::webrtc::ffi::MediaType;
        type RtpParameters = crate::rtp_parameters::ffi::RtpParameters;
        type MediaStreamPtr = crate::helper::ffi::MediaStreamPtr;
        type MediaStreamTrack = crate::media_stream::ffi::MediaStreamTrack;
        type MediaStream = crate::media_stream::ffi::MediaStream;
        type AdaptedNativeFrameTransformer = crate::frame_transformer::ffi::AdaptedNativeFrameTransformer;
    }

    unsafe extern "C++" {
        include!("livekit/rtp_receiver.h");

        type RtpReceiver;
        // type FrameTransformerInterface;
        // type AdaptedNativeFrameTransformer;

        fn track(self: &RtpReceiver) -> SharedPtr<MediaStreamTrack>;
        fn stream_ids(self: &RtpReceiver) -> Vec<String>;
        fn streams(self: &RtpReceiver) -> Vec<MediaStreamPtr>;
        fn media_type(self: &RtpReceiver) -> MediaType;
        fn id(self: &RtpReceiver) -> String;
        fn get_parameters(self: &RtpReceiver) -> RtpParameters;
        fn set_jitter_buffer_minimum_delay(self: &RtpReceiver, is_some: bool, delay_seconds: f64);
        fn set_depacketizer_to_decoder_frame_transformer(self: &RtpReceiver, frame_transformer: SharedPtr<AdaptedNativeFrameTransformer>);

        fn request_key_frame(self: &RtpReceiver);

        fn _shared_rtp_receiver() -> SharedPtr<RtpReceiver>;
    }
}

impl_thread_safety!(ffi::RtpReceiver, Send + Sync);