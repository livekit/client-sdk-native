// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameCryptor {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub key_index: i32,
    #[prost(bool, tag="4")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyProviderOptions {
    /// Only specify if you want to use a shared_key
    #[prost(bytes="vec", optional, tag="1")]
    pub shared_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, tag="2")]
    pub ratchet_window_size: i32,
    #[prost(bytes="vec", tag="3")]
    pub ratchet_salt: ::prost::alloc::vec::Vec<u8>,
    /// -1 = no tolerence
    #[prost(int32, tag="4")]
    pub failure_tolerance: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeOptions {
    #[prost(enumeration="EncryptionType", tag="1")]
    pub encryption_type: i32,
    #[prost(message, optional, tag="2")]
    pub key_provider_options: ::core::option::Option<KeyProviderOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeManagerSetEnabledRequest {
    #[prost(bool, tag="1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeManagerSetEnabledResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeManagerGetFrameCryptorsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeManagerGetFrameCryptorsResponse {
    #[prost(message, repeated, tag="1")]
    pub frame_cryptors: ::prost::alloc::vec::Vec<FrameCryptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameCryptorSetEnabledRequest {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameCryptorSetEnabledResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameCryptorSetKeyIndexRequest {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameCryptorSetKeyIndexResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSharedKeyRequest {
    #[prost(bytes="vec", tag="1")]
    pub shared_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSharedKeyResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RatchetSharedKeyRequest {
    #[prost(int32, tag="1")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RatchetSharedKeyResponse {
    #[prost(bytes="vec", optional, tag="1")]
    pub new_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedKeyRequest {
    #[prost(int32, tag="1")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedKeyResponse {
    #[prost(bytes="vec", optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyRequest {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="3")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RatchetKeyRequest {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RatchetKeyResponse {
    #[prost(bytes="vec", optional, tag="1")]
    pub new_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    #[prost(string, tag="1")]
    pub participant_identity: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub key_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyResponse {
    #[prost(bytes="vec", optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeRequest {
    #[prost(uint64, tag="1")]
    pub room_handle: u64,
    #[prost(oneof="e2ee_request::Message", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub message: ::core::option::Option<e2ee_request::Message>,
}
/// Nested message and enum types in `E2eeRequest`.
pub mod e2ee_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="2")]
        ManagerSetEnabled(super::E2eeManagerSetEnabledRequest),
        #[prost(message, tag="3")]
        ManagerGetFrameCryptors(super::E2eeManagerGetFrameCryptorsRequest),
        #[prost(message, tag="4")]
        CryptorSetEnabled(super::FrameCryptorSetEnabledRequest),
        #[prost(message, tag="5")]
        CryptorSetKeyIndex(super::FrameCryptorSetKeyIndexRequest),
        #[prost(message, tag="6")]
        SetSharedKey(super::SetSharedKeyRequest),
        #[prost(message, tag="7")]
        RatchetSharedKey(super::RatchetSharedKeyRequest),
        #[prost(message, tag="8")]
        GetSharedKey(super::GetSharedKeyRequest),
        #[prost(message, tag="9")]
        SetKey(super::SetKeyRequest),
        #[prost(message, tag="10")]
        RatchetKey(super::RatchetKeyRequest),
        #[prost(message, tag="11")]
        GetKey(super::GetKeyRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeResponse {
    #[prost(oneof="e2ee_response::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub message: ::core::option::Option<e2ee_response::Message>,
}
/// Nested message and enum types in `E2eeResponse`.
pub mod e2ee_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        ManagerSetEnabled(super::E2eeManagerSetEnabledResponse),
        #[prost(message, tag="2")]
        ManagerGetFrameCryptors(super::E2eeManagerGetFrameCryptorsResponse),
        #[prost(message, tag="3")]
        CryptorSetEnabled(super::FrameCryptorSetEnabledResponse),
        #[prost(message, tag="4")]
        CryptorSetKeyIndex(super::FrameCryptorSetKeyIndexResponse),
        #[prost(message, tag="5")]
        SetSharedKey(super::SetSharedKeyResponse),
        #[prost(message, tag="6")]
        RatchetSharedKey(super::RatchetSharedKeyResponse),
        #[prost(message, tag="7")]
        GetSharedKey(super::GetSharedKeyResponse),
        #[prost(message, tag="8")]
        SetKey(super::SetKeyResponse),
        #[prost(message, tag="9")]
        RatchetKey(super::RatchetKeyResponse),
        #[prost(message, tag="10")]
        GetKey(super::GetKeyResponse),
    }
}
// TODO(theomonnom): Should FrameCryptor be stateful on the client side and have their own handle?

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptionType {
    None = 0,
    Gcm = 1,
    Custom = 2,
}
impl EncryptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncryptionType::None => "NONE",
            EncryptionType::Gcm => "GCM",
            EncryptionType::Custom => "CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "GCM" => Some(Self::Gcm),
            "CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptionState {
    New = 0,
    Ok = 1,
    EncryptionFailed = 2,
    DecryptionFailed = 3,
    MissingKey = 4,
    KeyRatcheted = 5,
    InternalError = 6,
}
impl EncryptionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncryptionState::New => "NEW",
            EncryptionState::Ok => "OK",
            EncryptionState::EncryptionFailed => "ENCRYPTION_FAILED",
            EncryptionState::DecryptionFailed => "DECRYPTION_FAILED",
            EncryptionState::MissingKey => "MISSING_KEY",
            EncryptionState::KeyRatcheted => "KEY_RATCHETED",
            EncryptionState::InternalError => "INTERNAL_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NEW" => Some(Self::New),
            "OK" => Some(Self::Ok),
            "ENCRYPTION_FAILED" => Some(Self::EncryptionFailed),
            "DECRYPTION_FAILED" => Some(Self::DecryptionFailed),
            "MISSING_KEY" => Some(Self::MissingKey),
            "KEY_RATCHETED" => Some(Self::KeyRatcheted),
            "INTERNAL_ERROR" => Some(Self::InternalError),
            _ => None,
        }
    }
}
/// # Safety
/// The foreign language is responsable for disposing handles
/// Forgetting to dispose the handle may lead to memory leaks
/// 
/// Dropping a handle doesn't necessarily mean that the object is destroyed if it is still used
/// on the FfiServer (Atomic reference counting)
/// 
/// When refering to a handle without owning it, we just use a uint32 without this message. 
/// (the variable name is suffixed with "_handle")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiOwnedHandle {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// Create a new VideoTrack from a VideoSource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVideoTrackRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub source_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVideoTrackResponse {
    #[prost(message, optional, tag="1")]
    pub track: ::core::option::Option<OwnedTrack>,
}
/// Create a new AudioTrack from a AudioSource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAudioTrackRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub source_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAudioTrackResponse {
    #[prost(message, optional, tag="1")]
    pub track: ::core::option::Option<OwnedTrack>,
}
//
// Track
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPublicationInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="TrackKind", tag="3")]
    pub kind: i32,
    #[prost(enumeration="TrackSource", tag="4")]
    pub source: i32,
    #[prost(bool, tag="5")]
    pub simulcasted: bool,
    #[prost(uint32, tag="6")]
    pub width: u32,
    #[prost(uint32, tag="7")]
    pub height: u32,
    #[prost(string, tag="8")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub muted: bool,
    #[prost(bool, tag="10")]
    pub remote: bool,
    #[prost(enumeration="EncryptionType", tag="11")]
    pub encryption_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedTrackPublication {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<TrackPublicationInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="TrackKind", tag="3")]
    pub kind: i32,
    #[prost(enumeration="StreamState", tag="4")]
    pub stream_state: i32,
    #[prost(bool, tag="5")]
    pub muted: bool,
    #[prost(bool, tag="6")]
    pub remote: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedTrack {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<TrackInfo>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrackKind {
    KindUnknown = 0,
    KindAudio = 1,
    KindVideo = 2,
}
impl TrackKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrackKind::KindUnknown => "KIND_UNKNOWN",
            TrackKind::KindAudio => "KIND_AUDIO",
            TrackKind::KindVideo => "KIND_VIDEO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KIND_UNKNOWN" => Some(Self::KindUnknown),
            "KIND_AUDIO" => Some(Self::KindAudio),
            "KIND_VIDEO" => Some(Self::KindVideo),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrackSource {
    SourceUnknown = 0,
    SourceCamera = 1,
    SourceMicrophone = 2,
    SourceScreenshare = 3,
    SourceScreenshareAudio = 4,
}
impl TrackSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrackSource::SourceUnknown => "SOURCE_UNKNOWN",
            TrackSource::SourceCamera => "SOURCE_CAMERA",
            TrackSource::SourceMicrophone => "SOURCE_MICROPHONE",
            TrackSource::SourceScreenshare => "SOURCE_SCREENSHARE",
            TrackSource::SourceScreenshareAudio => "SOURCE_SCREENSHARE_AUDIO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOURCE_UNKNOWN" => Some(Self::SourceUnknown),
            "SOURCE_CAMERA" => Some(Self::SourceCamera),
            "SOURCE_MICROPHONE" => Some(Self::SourceMicrophone),
            "SOURCE_SCREENSHARE" => Some(Self::SourceScreenshare),
            "SOURCE_SCREENSHARE_AUDIO" => Some(Self::SourceScreenshareAudio),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StreamState {
    StateUnknown = 0,
    StateActive = 1,
    StatePaused = 2,
}
impl StreamState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StreamState::StateUnknown => "STATE_UNKNOWN",
            StreamState::StateActive => "STATE_ACTIVE",
            StreamState::StatePaused => "STATE_PAUSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNKNOWN" => Some(Self::StateUnknown),
            "STATE_ACTIVE" => Some(Self::StateActive),
            "STATE_PAUSED" => Some(Self::StatePaused),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedParticipant {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<ParticipantInfo>,
}
/// Allocate a new VideoFrameBuffer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocVideoBufferRequest {
    /// Only I420 is supported atm
    #[prost(enumeration="VideoFrameBufferType", tag="1")]
    pub r#type: i32,
    #[prost(uint32, tag="2")]
    pub width: u32,
    #[prost(uint32, tag="3")]
    pub height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocVideoBufferResponse {
    #[prost(message, optional, tag="1")]
    pub buffer: ::core::option::Option<OwnedVideoFrameBuffer>,
}
/// Create a new VideoStream
/// VideoStream is used to receive video frames from a track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVideoStreamRequest {
    #[prost(uint64, tag="1")]
    pub track_handle: u64,
    #[prost(enumeration="VideoStreamType", tag="2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVideoStreamResponse {
    #[prost(message, optional, tag="1")]
    pub stream: ::core::option::Option<OwnedVideoStream>,
}
/// Create a new VideoSource
/// VideoSource is used to send video frame to a track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVideoSourceRequest {
    #[prost(enumeration="VideoSourceType", tag="1")]
    pub r#type: i32,
    /// Used to determine which encodings to use + simulcast layers
    /// Most of the time it corresponds to the source resolution 
    #[prost(message, optional, tag="2")]
    pub resolution: ::core::option::Option<VideoSourceResolution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVideoSourceResponse {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<OwnedVideoSource>,
}
/// Push a frame to a VideoSource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureVideoFrameRequest {
    #[prost(uint64, tag="1")]
    pub source_handle: u64,
    #[prost(message, optional, tag="2")]
    pub frame: ::core::option::Option<VideoFrameInfo>,
    #[prost(oneof="capture_video_frame_request::From", tags="3, 4")]
    pub from: ::core::option::Option<capture_video_frame_request::From>,
}
/// Nested message and enum types in `CaptureVideoFrameRequest`.
pub mod capture_video_frame_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum From {
        #[prost(message, tag="3")]
        Info(super::VideoFrameBufferInfo),
        #[prost(uint64, tag="4")]
        Handle(u64),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureVideoFrameResponse {
}
/// Convert a RGBA frame to a I420 YUV frame
/// Or convert another YUV frame format to I420
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToI420Request {
    #[prost(bool, tag="1")]
    pub flip_y: bool,
    #[prost(oneof="to_i420_request::From", tags="2, 3, 4")]
    pub from: ::core::option::Option<to_i420_request::From>,
}
/// Nested message and enum types in `ToI420Request`.
pub mod to_i420_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum From {
        #[prost(message, tag="2")]
        Argb(super::ArgbBufferInfo),
        #[prost(message, tag="3")]
        Buffer(super::VideoFrameBufferInfo),
        #[prost(uint64, tag="4")]
        Handle(u64),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToI420Response {
    #[prost(message, optional, tag="1")]
    pub buffer: ::core::option::Option<OwnedVideoFrameBuffer>,
}
/// Convert a YUV frame to a RGBA frame
/// Only I420 is supported atm
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToArgbRequest {
    #[prost(message, optional, tag="1")]
    pub buffer: ::core::option::Option<VideoFrameBufferInfo>,
    #[prost(uint64, tag="2")]
    pub dst_ptr: u64,
    #[prost(enumeration="VideoFormatType", tag="3")]
    pub dst_format: i32,
    #[prost(uint32, tag="4")]
    pub dst_stride: u32,
    #[prost(uint32, tag="5")]
    pub dst_width: u32,
    #[prost(uint32, tag="6")]
    pub dst_height: u32,
    #[prost(bool, tag="7")]
    pub flip_y: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToArgbResponse {
}
//
// VideoFrame buffers
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoResolution {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(double, tag="3")]
    pub frame_rate: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArgbBufferInfo {
    #[prost(uint64, tag="1")]
    pub ptr: u64,
    #[prost(enumeration="VideoFormatType", tag="2")]
    pub format: i32,
    #[prost(uint32, tag="3")]
    pub stride: u32,
    #[prost(uint32, tag="4")]
    pub width: u32,
    #[prost(uint32, tag="5")]
    pub height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoFrameInfo {
    /// In microseconds
    #[prost(int64, tag="1")]
    pub timestamp_us: i64,
    #[prost(enumeration="VideoRotation", tag="2")]
    pub rotation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoFrameBufferInfo {
    #[prost(enumeration="VideoFrameBufferType", tag="1")]
    pub buffer_type: i32,
    #[prost(uint32, tag="2")]
    pub width: u32,
    #[prost(uint32, tag="3")]
    pub height: u32,
    #[prost(oneof="video_frame_buffer_info::Buffer", tags="4, 5, 6")]
    pub buffer: ::core::option::Option<video_frame_buffer_info::Buffer>,
}
/// Nested message and enum types in `VideoFrameBufferInfo`.
pub mod video_frame_buffer_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Buffer {
        #[prost(message, tag="4")]
        Yuv(super::PlanarYuvBufferInfo),
        #[prost(message, tag="5")]
        BiYuv(super::BiplanarYuvBufferInfo),
        #[prost(message, tag="6")]
        Native(super::NativeBufferInfo),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedVideoFrameBuffer {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<VideoFrameBufferInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanarYuvBufferInfo {
    #[prost(uint32, tag="1")]
    pub chroma_width: u32,
    #[prost(uint32, tag="2")]
    pub chroma_height: u32,
    #[prost(uint32, tag="3")]
    pub stride_y: u32,
    #[prost(uint32, tag="4")]
    pub stride_u: u32,
    #[prost(uint32, tag="5")]
    pub stride_v: u32,
    #[prost(uint32, tag="6")]
    pub stride_a: u32,
    /// *const u8 or *const u16
    #[prost(uint64, tag="7")]
    pub data_y_ptr: u64,
    #[prost(uint64, tag="8")]
    pub data_u_ptr: u64,
    #[prost(uint64, tag="9")]
    pub data_v_ptr: u64,
    /// nullptr = no alpha
    #[prost(uint64, tag="10")]
    pub data_a_ptr: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiplanarYuvBufferInfo {
    #[prost(uint32, tag="1")]
    pub chroma_width: u32,
    #[prost(uint32, tag="2")]
    pub chroma_height: u32,
    #[prost(uint32, tag="3")]
    pub stride_y: u32,
    #[prost(uint32, tag="4")]
    pub stride_uv: u32,
    #[prost(uint64, tag="5")]
    pub data_y_ptr: u64,
    #[prost(uint64, tag="6")]
    pub data_uv_ptr: u64,
}
/// TODO(theomonnom): Expose graphic context?
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeBufferInfo {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInfo {
    #[prost(enumeration="VideoStreamType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedVideoStream {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<VideoStreamInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamEvent {
    #[prost(uint64, tag="1")]
    pub stream_handle: u64,
    #[prost(oneof="video_stream_event::Message", tags="2, 3")]
    pub message: ::core::option::Option<video_stream_event::Message>,
}
/// Nested message and enum types in `VideoStreamEvent`.
pub mod video_stream_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="2")]
        FrameReceived(super::VideoFrameReceived),
        #[prost(message, tag="3")]
        Eos(super::VideoStreamEos),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoFrameReceived {
    #[prost(message, optional, tag="1")]
    pub frame: ::core::option::Option<VideoFrameInfo>,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<OwnedVideoFrameBuffer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamEos {
}
//
// VideoSource
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSourceResolution {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSourceInfo {
    #[prost(enumeration="VideoSourceType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedVideoSource {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<VideoSourceInfo>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoCodec {
    Vp8 = 0,
    H264 = 1,
    Av1 = 2,
}
impl VideoCodec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoCodec::Vp8 => "VP8",
            VideoCodec::H264 => "H264",
            VideoCodec::Av1 => "AV1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VP8" => Some(Self::Vp8),
            "H264" => Some(Self::H264),
            "AV1" => Some(Self::Av1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoRotation {
    VideoRotation0 = 0,
    VideoRotation90 = 1,
    VideoRotation180 = 2,
    VideoRotation270 = 3,
}
impl VideoRotation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoRotation::VideoRotation0 => "VIDEO_ROTATION_0",
            VideoRotation::VideoRotation90 => "VIDEO_ROTATION_90",
            VideoRotation::VideoRotation180 => "VIDEO_ROTATION_180",
            VideoRotation::VideoRotation270 => "VIDEO_ROTATION_270",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIDEO_ROTATION_0" => Some(Self::VideoRotation0),
            "VIDEO_ROTATION_90" => Some(Self::VideoRotation90),
            "VIDEO_ROTATION_180" => Some(Self::VideoRotation180),
            "VIDEO_ROTATION_270" => Some(Self::VideoRotation270),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoFormatType {
    FormatArgb = 0,
    FormatBgra = 1,
    FormatAbgr = 2,
    FormatRgba = 3,
}
impl VideoFormatType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoFormatType::FormatArgb => "FORMAT_ARGB",
            VideoFormatType::FormatBgra => "FORMAT_BGRA",
            VideoFormatType::FormatAbgr => "FORMAT_ABGR",
            VideoFormatType::FormatRgba => "FORMAT_RGBA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FORMAT_ARGB" => Some(Self::FormatArgb),
            "FORMAT_BGRA" => Some(Self::FormatBgra),
            "FORMAT_ABGR" => Some(Self::FormatAbgr),
            "FORMAT_RGBA" => Some(Self::FormatRgba),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoFrameBufferType {
    Native = 0,
    I420 = 1,
    I420a = 2,
    I422 = 3,
    I444 = 4,
    I010 = 5,
    Nv12 = 6,
}
impl VideoFrameBufferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoFrameBufferType::Native => "NATIVE",
            VideoFrameBufferType::I420 => "I420",
            VideoFrameBufferType::I420a => "I420A",
            VideoFrameBufferType::I422 => "I422",
            VideoFrameBufferType::I444 => "I444",
            VideoFrameBufferType::I010 => "I010",
            VideoFrameBufferType::Nv12 => "NV12",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NATIVE" => Some(Self::Native),
            "I420" => Some(Self::I420),
            "I420A" => Some(Self::I420a),
            "I422" => Some(Self::I422),
            "I444" => Some(Self::I444),
            "I010" => Some(Self::I010),
            "NV12" => Some(Self::Nv12),
            _ => None,
        }
    }
}
//
// VideoStream
//

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoStreamType {
    VideoStreamNative = 0,
    VideoStreamWebgl = 1,
    VideoStreamHtml = 2,
}
impl VideoStreamType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoStreamType::VideoStreamNative => "VIDEO_STREAM_NATIVE",
            VideoStreamType::VideoStreamWebgl => "VIDEO_STREAM_WEBGL",
            VideoStreamType::VideoStreamHtml => "VIDEO_STREAM_HTML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIDEO_STREAM_NATIVE" => Some(Self::VideoStreamNative),
            "VIDEO_STREAM_WEBGL" => Some(Self::VideoStreamWebgl),
            "VIDEO_STREAM_HTML" => Some(Self::VideoStreamHtml),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoSourceType {
    VideoSourceNative = 0,
}
impl VideoSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoSourceType::VideoSourceNative => "VIDEO_SOURCE_NATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIDEO_SOURCE_NATIVE" => Some(Self::VideoSourceNative),
            _ => None,
        }
    }
}
/// Connect to a new LiveKit room
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<RoomOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub room: ::core::option::Option<OwnedRoom>,
    #[prost(message, optional, tag="4")]
    pub local_participant: ::core::option::Option<OwnedParticipant>,
    #[prost(message, repeated, tag="5")]
    pub participants: ::prost::alloc::vec::Vec<connect_callback::ParticipantWithTracks>,
}
/// Nested message and enum types in `ConnectCallback`.
pub mod connect_callback {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParticipantWithTracks {
        #[prost(message, optional, tag="1")]
        pub participant: ::core::option::Option<super::OwnedParticipant>,
        /// TrackInfo are not needed here, if we're subscribed to a track, the FfiServer will send
        /// a TrackSubscribed event
        #[prost(message, repeated, tag="2")]
        pub publications: ::prost::alloc::vec::Vec<super::OwnedTrackPublication>,
    }
}
/// Disconnect from the a room
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectRequest {
    #[prost(uint64, tag="1")]
    pub room_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
/// Publish a track to the room
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishTrackRequest {
    #[prost(uint64, tag="1")]
    pub local_participant_handle: u64,
    #[prost(uint64, tag="2")]
    pub track_handle: u64,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<TrackPublishOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishTrackResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishTrackCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub publication: ::core::option::Option<OwnedTrackPublication>,
}
/// Unpublish a track from the room
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpublishTrackRequest {
    #[prost(uint64, tag="1")]
    pub local_participant_handle: u64,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub stop_on_unpublish: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpublishTrackResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpublishTrackCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Publish data to other participants
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishDataRequest {
    #[prost(uint64, tag="1")]
    pub local_participant_handle: u64,
    #[prost(uint64, tag="2")]
    pub data_ptr: u64,
    #[prost(uint64, tag="3")]
    pub data_len: u64,
    #[prost(enumeration="DataPacketKind", tag="4")]
    pub kind: i32,
    /// destination
    #[prost(string, repeated, tag="5")]
    pub destination_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishDataResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishDataCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Change the local participant's metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalMetadataRequest {
    #[prost(uint64, tag="1")]
    pub local_participant_handle: u64,
    #[prost(string, tag="2")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalMetadataResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalMetadataCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
/// Change the local participant's name
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalNameRequest {
    #[prost(uint64, tag="1")]
    pub local_participant_handle: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalNameResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocalNameCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
/// Change the "desire" to subs2ribe to a track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSubscribedRequest {
    #[prost(bool, tag="1")]
    pub subscribe: bool,
    #[prost(uint64, tag="2")]
    pub publication_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSubscribedResponse {
}
//
// Options
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoEncoding {
    #[prost(uint64, tag="1")]
    pub max_bitrate: u64,
    #[prost(double, tag="2")]
    pub max_framerate: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioEncoding {
    #[prost(uint64, tag="1")]
    pub max_bitrate: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPublishOptions {
    /// encodings are optional
    #[prost(message, optional, tag="1")]
    pub video_encoding: ::core::option::Option<VideoEncoding>,
    #[prost(message, optional, tag="2")]
    pub audio_encoding: ::core::option::Option<AudioEncoding>,
    #[prost(enumeration="VideoCodec", tag="3")]
    pub video_codec: i32,
    #[prost(bool, tag="4")]
    pub dtx: bool,
    #[prost(bool, tag="5")]
    pub red: bool,
    #[prost(bool, tag="6")]
    pub simulcast: bool,
    #[prost(enumeration="TrackSource", tag="7")]
    pub source: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IceServer {
    #[prost(string, repeated, tag="1")]
    pub urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcConfig {
    #[prost(enumeration="IceTransportType", optional, tag="1")]
    pub ice_transport_type: ::core::option::Option<i32>,
    #[prost(enumeration="ContinualGatheringPolicy", optional, tag="2")]
    pub continual_gathering_policy: ::core::option::Option<i32>,
    /// empty fallback to default
    #[prost(message, repeated, tag="3")]
    pub ice_servers: ::prost::alloc::vec::Vec<IceServer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomOptions {
    #[prost(bool, tag="1")]
    pub auto_subscribe: bool,
    #[prost(bool, tag="2")]
    pub adaptive_stream: bool,
    #[prost(bool, tag="3")]
    pub dynacast: bool,
    #[prost(message, optional, tag="4")]
    pub e2ee: ::core::option::Option<E2eeOptions>,
    /// allow to setup a custom RtcConfiguration
    #[prost(message, optional, tag="5")]
    pub rtc_config: ::core::option::Option<RtcConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferInfo {
    #[prost(uint64, tag="1")]
    pub data_ptr: u64,
    #[prost(uint64, tag="2")]
    pub data_len: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedBuffer {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<BufferInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomEvent {
    #[prost(uint64, tag="1")]
    pub room_handle: u64,
    #[prost(oneof="room_event::Message", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25")]
    pub message: ::core::option::Option<room_event::Message>,
}
/// Nested message and enum types in `RoomEvent`.
pub mod room_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="2")]
        ParticipantConnected(super::ParticipantConnected),
        #[prost(message, tag="3")]
        ParticipantDisconnected(super::ParticipantDisconnected),
        #[prost(message, tag="4")]
        LocalTrackPublished(super::LocalTrackPublished),
        #[prost(message, tag="5")]
        LocalTrackUnpublished(super::LocalTrackUnpublished),
        #[prost(message, tag="6")]
        TrackPublished(super::TrackPublished),
        #[prost(message, tag="7")]
        TrackUnpublished(super::TrackUnpublished),
        #[prost(message, tag="8")]
        TrackSubscribed(super::TrackSubscribed),
        #[prost(message, tag="9")]
        TrackUnsubscribed(super::TrackUnsubscribed),
        #[prost(message, tag="10")]
        TrackSubscriptionFailed(super::TrackSubscriptionFailed),
        #[prost(message, tag="11")]
        TrackMuted(super::TrackMuted),
        #[prost(message, tag="12")]
        TrackUnmuted(super::TrackUnmuted),
        #[prost(message, tag="13")]
        ActiveSpeakersChanged(super::ActiveSpeakersChanged),
        #[prost(message, tag="14")]
        RoomMetadataChanged(super::RoomMetadataChanged),
        #[prost(message, tag="15")]
        ParticipantMetadataChanged(super::ParticipantMetadataChanged),
        #[prost(message, tag="16")]
        ParticipantNameChanged(super::ParticipantNameChanged),
        #[prost(message, tag="17")]
        ConnectionQualityChanged(super::ConnectionQualityChanged),
        #[prost(message, tag="18")]
        DataReceived(super::DataReceived),
        #[prost(message, tag="19")]
        ConnectionStateChanged(super::ConnectionStateChanged),
        /// Connected connected = 20;
        #[prost(message, tag="21")]
        Disconnected(super::Disconnected),
        #[prost(message, tag="22")]
        Reconnecting(super::Reconnecting),
        #[prost(message, tag="23")]
        Reconnected(super::Reconnected),
        #[prost(message, tag="24")]
        E2eeStateChanged(super::E2eeStateChanged),
        /// The stream of room events has ended
        #[prost(message, tag="25")]
        Eos(super::RoomEos),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomInfo {
    #[prost(string, tag="1")]
    pub sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedRoom {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<RoomInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantConnected {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<OwnedParticipant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantDisconnected {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalTrackPublished {
    /// The TrackPublicationInfo comes from the PublishTrack response
    /// and the FfiClient musts wait for it before firing this event
    #[prost(string, tag="1")]
    pub track_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalTrackUnpublished {
    #[prost(string, tag="1")]
    pub publication_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPublished {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub publication: ::core::option::Option<OwnedTrackPublication>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUnpublished {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub publication_sid: ::prost::alloc::string::String,
}
/// Publication isn't needed for subscription events on the FFI
/// The FFI will retrieve the publication using the Track sid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackSubscribed {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub track: ::core::option::Option<OwnedTrack>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUnsubscribed {
    /// The FFI language can dispose/remove the VideoSink here
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackSubscriptionFailed {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackMuted {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUnmuted {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub track_sid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eeStateChanged {
    /// Using sid instead of identity for ffi communication
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(enumeration="EncryptionState", tag="2")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveSpeakersChanged {
    #[prost(string, repeated, tag="1")]
    pub participant_sids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMetadataChanged {
    #[prost(string, tag="1")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantMetadataChanged {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantNameChanged {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionQualityChanged {
    #[prost(string, tag="1")]
    pub participant_sid: ::prost::alloc::string::String,
    #[prost(enumeration="ConnectionQuality", tag="2")]
    pub quality: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataReceived {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<OwnedBuffer>,
    /// Can be empty if the data is sent a server SDK
    #[prost(string, optional, tag="2")]
    pub participant_sid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="DataPacketKind", tag="3")]
    pub kind: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateChanged {
    #[prost(enumeration="ConnectionState", tag="1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connected {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Disconnected {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reconnecting {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reconnected {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomEos {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceTransportType {
    TransportRelay = 0,
    TransportNohost = 1,
    TransportAll = 2,
}
impl IceTransportType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceTransportType::TransportRelay => "TRANSPORT_RELAY",
            IceTransportType::TransportNohost => "TRANSPORT_NOHOST",
            IceTransportType::TransportAll => "TRANSPORT_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSPORT_RELAY" => Some(Self::TransportRelay),
            "TRANSPORT_NOHOST" => Some(Self::TransportNohost),
            "TRANSPORT_ALL" => Some(Self::TransportAll),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContinualGatheringPolicy {
    GatherOnce = 0,
    GatherContinually = 1,
}
impl ContinualGatheringPolicy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContinualGatheringPolicy::GatherOnce => "GATHER_ONCE",
            ContinualGatheringPolicy::GatherContinually => "GATHER_CONTINUALLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GATHER_ONCE" => Some(Self::GatherOnce),
            "GATHER_CONTINUALLY" => Some(Self::GatherContinually),
            _ => None,
        }
    }
}
//
// Room
//

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionQuality {
    QualityPoor = 0,
    QualityGood = 1,
    QualityExcellent = 2,
}
impl ConnectionQuality {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionQuality::QualityPoor => "QUALITY_POOR",
            ConnectionQuality::QualityGood => "QUALITY_GOOD",
            ConnectionQuality::QualityExcellent => "QUALITY_EXCELLENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUALITY_POOR" => Some(Self::QualityPoor),
            "QUALITY_GOOD" => Some(Self::QualityGood),
            "QUALITY_EXCELLENT" => Some(Self::QualityExcellent),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConnectionState {
    ConnDisconnected = 0,
    ConnConnected = 1,
    ConnReconnecting = 2,
}
impl ConnectionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionState::ConnDisconnected => "CONN_DISCONNECTED",
            ConnectionState::ConnConnected => "CONN_CONNECTED",
            ConnectionState::ConnReconnecting => "CONN_RECONNECTING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONN_DISCONNECTED" => Some(Self::ConnDisconnected),
            "CONN_CONNECTED" => Some(Self::ConnConnected),
            "CONN_RECONNECTING" => Some(Self::ConnReconnecting),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataPacketKind {
    KindLossy = 0,
    KindReliable = 1,
}
impl DataPacketKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataPacketKind::KindLossy => "KIND_LOSSY",
            DataPacketKind::KindReliable => "KIND_RELIABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KIND_LOSSY" => Some(Self::KindLossy),
            "KIND_RELIABLE" => Some(Self::KindReliable),
            _ => None,
        }
    }
}
/// Allocate a new AudioFrameBuffer
/// This is not necessary required because the data structure is fairly simple
/// But keep the API consistent with VideoFrame
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocAudioBufferRequest {
    #[prost(uint32, tag="1")]
    pub sample_rate: u32,
    #[prost(uint32, tag="2")]
    pub num_channels: u32,
    #[prost(uint32, tag="3")]
    pub samples_per_channel: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocAudioBufferResponse {
    #[prost(message, optional, tag="1")]
    pub buffer: ::core::option::Option<OwnedAudioFrameBuffer>,
}
/// Create a new AudioStream
/// AudioStream is used to receive audio frames from a track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioStreamRequest {
    #[prost(uint64, tag="1")]
    pub track_handle: u64,
    #[prost(enumeration="AudioStreamType", tag="2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioStreamResponse {
    #[prost(message, optional, tag="1")]
    pub stream: ::core::option::Option<OwnedAudioStream>,
}
/// Create a new AudioSource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioSourceRequest {
    #[prost(enumeration="AudioSourceType", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<AudioSourceOptions>,
    #[prost(uint32, tag="3")]
    pub sample_rate: u32,
    #[prost(uint32, tag="4")]
    pub num_channels: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioSourceResponse {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<OwnedAudioSource>,
}
/// Push a frame to an AudioSource 
/// The data provided must be available as long as the client receive the callback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureAudioFrameRequest {
    #[prost(uint64, tag="1")]
    pub source_handle: u64,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<AudioFrameBufferInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureAudioFrameResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureAudioFrameCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Create a new AudioResampler
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioResamplerRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAudioResamplerResponse {
    #[prost(message, optional, tag="1")]
    pub resampler: ::core::option::Option<OwnedAudioResampler>,
}
/// Remix and resample an audio frame
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemixAndResampleRequest {
    #[prost(uint64, tag="1")]
    pub resampler_handle: u64,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<AudioFrameBufferInfo>,
    #[prost(uint32, tag="3")]
    pub num_channels: u32,
    #[prost(uint32, tag="4")]
    pub sample_rate: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemixAndResampleResponse {
    #[prost(message, optional, tag="1")]
    pub buffer: ::core::option::Option<OwnedAudioFrameBuffer>,
}
//
// AudioFrame buffer
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioFrameBufferInfo {
    /// *const i16
    #[prost(uint64, tag="1")]
    pub data_ptr: u64,
    #[prost(uint32, tag="2")]
    pub num_channels: u32,
    #[prost(uint32, tag="3")]
    pub sample_rate: u32,
    #[prost(uint32, tag="4")]
    pub samples_per_channel: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedAudioFrameBuffer {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<AudioFrameBufferInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStreamInfo {
    #[prost(enumeration="AudioStreamType", tag="1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedAudioStream {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<AudioStreamInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStreamEvent {
    #[prost(uint64, tag="1")]
    pub stream_handle: u64,
    #[prost(oneof="audio_stream_event::Message", tags="2, 3")]
    pub message: ::core::option::Option<audio_stream_event::Message>,
}
/// Nested message and enum types in `AudioStreamEvent`.
pub mod audio_stream_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="2")]
        FrameReceived(super::AudioFrameReceived),
        #[prost(message, tag="3")]
        Eos(super::AudioStreamEos),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioFrameReceived {
    #[prost(message, optional, tag="1")]
    pub frame: ::core::option::Option<OwnedAudioFrameBuffer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStreamEos {
}
//
// AudioSource
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioSourceOptions {
    #[prost(bool, tag="1")]
    pub echo_cancellation: bool,
    #[prost(bool, tag="2")]
    pub noise_suppression: bool,
    #[prost(bool, tag="3")]
    pub auto_gain_control: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioSourceInfo {
    #[prost(enumeration="AudioSourceType", tag="2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedAudioSource {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<AudioSourceInfo>,
}
//
// AudioResampler
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioResamplerInfo {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedAudioResampler {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<AudioResamplerInfo>,
}
//
// AudioStream
//

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioStreamType {
    AudioStreamNative = 0,
    AudioStreamHtml = 1,
}
impl AudioStreamType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioStreamType::AudioStreamNative => "AUDIO_STREAM_NATIVE",
            AudioStreamType::AudioStreamHtml => "AUDIO_STREAM_HTML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO_STREAM_NATIVE" => Some(Self::AudioStreamNative),
            "AUDIO_STREAM_HTML" => Some(Self::AudioStreamHtml),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioSourceType {
    AudioSourceNative = 0,
}
impl AudioSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AudioSourceType::AudioSourceNative => "AUDIO_SOURCE_NATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUDIO_SOURCE_NATIVE" => Some(Self::AudioSourceNative),
            _ => None,
        }
    }
}
// **How is the livekit-ffi working:
// We refer as the ffi server the Rust server that is running the LiveKit client implementation, and we
// refer as the ffi client the foreign language that commumicates with the ffi server. (e.g Python SDK, Unity SDK, etc...)
//
// We expose the Rust client implementation of livekit using the protocol defined here.
// Everything starts with a FfiRequest, which is a oneof message that contains all the possible
// requests that can be made to the ffi server.
// The server will then respond with a FfiResponse, which is also a oneof message that contains
// all the possible responses.
// The first request sent to the server must be an InitializeRequest, which contains the a pointer
// to the callback function that will be used to send events and async responses to the ffi client.
// (e.g participant joined, track published, etc...)
//
// **Useful things know when collaborating on the protocol:**
// Everything is subject to discussion and change :-)
//
// - The ffi client implementation must never forget to correctly dispose all the owned handles
//    that it receives from the server.
//
// Therefore, the ffi client is easier to implement if there is less handles to manage.
// 
// - We are mainly using FfiHandle on info messages (e.g: RoomInfo, TrackInfo, etc...)
//    For this reason, info are only sent once, at creation (We're not using them for updates, we can infer them from
//    events on the client implementation).
//    e.g: set speaking to true when we receive a ActiveSpeakerChanged event.

/// This is the input of livekit_ffi_request function
/// We always expect a response (FFIResponse, even if it's empty)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiRequest {
    #[prost(oneof="ffi_request::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25")]
    pub message: ::core::option::Option<ffi_request::Message>,
}
/// Nested message and enum types in `FfiRequest`.
pub mod ffi_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        Initialize(super::InitializeRequest),
        #[prost(message, tag="2")]
        Dispose(super::DisposeRequest),
        /// Room
        #[prost(message, tag="3")]
        Connect(super::ConnectRequest),
        #[prost(message, tag="4")]
        Disconnect(super::DisconnectRequest),
        #[prost(message, tag="5")]
        PublishTrack(super::PublishTrackRequest),
        #[prost(message, tag="6")]
        UnpublishTrack(super::UnpublishTrackRequest),
        #[prost(message, tag="7")]
        PublishData(super::PublishDataRequest),
        #[prost(message, tag="8")]
        SetSubscribed(super::SetSubscribedRequest),
        #[prost(message, tag="9")]
        UpdateLocalMetadata(super::UpdateLocalMetadataRequest),
        #[prost(message, tag="10")]
        UpdateLocalName(super::UpdateLocalNameRequest),
        /// Track
        #[prost(message, tag="11")]
        CreateVideoTrack(super::CreateVideoTrackRequest),
        #[prost(message, tag="12")]
        CreateAudioTrack(super::CreateAudioTrackRequest),
        /// Video
        #[prost(message, tag="13")]
        AllocVideoBuffer(super::AllocVideoBufferRequest),
        #[prost(message, tag="14")]
        NewVideoStream(super::NewVideoStreamRequest),
        #[prost(message, tag="15")]
        NewVideoSource(super::NewVideoSourceRequest),
        #[prost(message, tag="16")]
        CaptureVideoFrame(super::CaptureVideoFrameRequest),
        #[prost(message, tag="17")]
        ToI420(super::ToI420Request),
        #[prost(message, tag="18")]
        ToArgb(super::ToArgbRequest),
        /// Audio
        #[prost(message, tag="19")]
        AllocAudioBuffer(super::AllocAudioBufferRequest),
        #[prost(message, tag="20")]
        NewAudioStream(super::NewAudioStreamRequest),
        #[prost(message, tag="21")]
        NewAudioSource(super::NewAudioSourceRequest),
        #[prost(message, tag="22")]
        CaptureAudioFrame(super::CaptureAudioFrameRequest),
        #[prost(message, tag="23")]
        NewAudioResampler(super::NewAudioResamplerRequest),
        #[prost(message, tag="24")]
        RemixAndResample(super::RemixAndResampleRequest),
        #[prost(message, tag="25")]
        E2ee(super::E2eeRequest),
    }
}
/// This is the output of livekit_ffi_request function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiResponse {
    #[prost(oneof="ffi_response::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25")]
    pub message: ::core::option::Option<ffi_response::Message>,
}
/// Nested message and enum types in `FfiResponse`.
pub mod ffi_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        Initialize(super::InitializeResponse),
        #[prost(message, tag="2")]
        Dispose(super::DisposeResponse),
        /// Room
        #[prost(message, tag="3")]
        Connect(super::ConnectResponse),
        #[prost(message, tag="4")]
        Disconnect(super::DisconnectResponse),
        #[prost(message, tag="5")]
        PublishTrack(super::PublishTrackResponse),
        #[prost(message, tag="6")]
        UnpublishTrack(super::UnpublishTrackResponse),
        #[prost(message, tag="7")]
        PublishData(super::PublishDataResponse),
        #[prost(message, tag="8")]
        SetSubscribed(super::SetSubscribedResponse),
        #[prost(message, tag="9")]
        UpdateLocalMetadata(super::UpdateLocalMetadataResponse),
        #[prost(message, tag="10")]
        UpdateLocalName(super::UpdateLocalNameResponse),
        /// Track
        #[prost(message, tag="11")]
        CreateVideoTrack(super::CreateVideoTrackResponse),
        #[prost(message, tag="12")]
        CreateAudioTrack(super::CreateAudioTrackResponse),
        /// Video
        #[prost(message, tag="13")]
        AllocVideoBuffer(super::AllocVideoBufferResponse),
        #[prost(message, tag="14")]
        NewVideoStream(super::NewVideoStreamResponse),
        #[prost(message, tag="15")]
        NewVideoSource(super::NewVideoSourceResponse),
        #[prost(message, tag="16")]
        CaptureVideoFrame(super::CaptureVideoFrameResponse),
        #[prost(message, tag="17")]
        ToI420(super::ToI420Response),
        #[prost(message, tag="18")]
        ToArgb(super::ToArgbResponse),
        /// Audio
        #[prost(message, tag="19")]
        AllocAudioBuffer(super::AllocAudioBufferResponse),
        #[prost(message, tag="20")]
        NewAudioStream(super::NewAudioStreamResponse),
        #[prost(message, tag="21")]
        NewAudioSource(super::NewAudioSourceResponse),
        #[prost(message, tag="22")]
        CaptureAudioFrame(super::CaptureAudioFrameResponse),
        #[prost(message, tag="23")]
        NewAudioResampler(super::NewAudioResamplerResponse),
        #[prost(message, tag="24")]
        RemixAndResample(super::RemixAndResampleResponse),
        #[prost(message, tag="25")]
        E2ee(super::E2eeResponse),
    }
}
/// To minimize complexity, participant events are not included in the protocol.
/// It is easily deducible from the room events and it turned out that is is easier to implement
/// on the ffi client side.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiEvent {
    #[prost(oneof="ffi_event::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub message: ::core::option::Option<ffi_event::Message>,
}
/// Nested message and enum types in `FfiEvent`.
pub mod ffi_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        RoomEvent(super::RoomEvent),
        #[prost(message, tag="2")]
        TrackEvent(super::TrackEvent),
        #[prost(message, tag="3")]
        VideoStreamEvent(super::VideoStreamEvent),
        #[prost(message, tag="4")]
        AudioStreamEvent(super::AudioStreamEvent),
        #[prost(message, tag="5")]
        Connect(super::ConnectCallback),
        #[prost(message, tag="6")]
        Disconnect(super::DisconnectCallback),
        #[prost(message, tag="7")]
        Dispose(super::DisposeCallback),
        #[prost(message, tag="8")]
        PublishTrack(super::PublishTrackCallback),
        #[prost(message, tag="9")]
        UnpublishTrack(super::UnpublishTrackCallback),
        #[prost(message, tag="10")]
        PublishData(super::PublishDataCallback),
        #[prost(message, tag="11")]
        CaptureAudioFrame(super::CaptureAudioFrameCallback),
        #[prost(message, tag="12")]
        UpdateLocalMetadata(super::UpdateLocalMetadataCallback),
        #[prost(message, tag="13")]
        UpdateLocalName(super::UpdateLocalNameCallback),
    }
}
/// Setup the callback where the foreign language can receive events
/// and responses to asynchronous requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRequest {
    #[prost(uint64, tag="1")]
    pub event_callback_ptr: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeResponse {
}
/// Stop all rooms synchronously (Do we need async here?).
/// e.g: This is used for the Unity Editor after each assemblies reload.
/// TODO(theomonnom): Implement a debug mode where we can find all leaked handles?
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisposeRequest {
    #[prost(bool, tag="1")]
    pub r#async: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisposeResponse {
    /// None if sync
    #[prost(uint64, optional, tag="1")]
    pub async_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisposeCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
// @@protoc_insertion_point(module)
