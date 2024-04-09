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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcStats {
    #[prost(oneof="rtc_stats::Stats", tags="3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17")]
    pub stats: ::core::option::Option<rtc_stats::Stats>,
}
/// Nested message and enum types in `RtcStats`.
pub mod rtc_stats {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Codec {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub codec: ::core::option::Option<super::CodecStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InboundRtp {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub stream: ::core::option::Option<super::RtpStreamStats>,
        #[prost(message, optional, tag="3")]
        pub received: ::core::option::Option<super::ReceivedRtpStreamStats>,
        #[prost(message, optional, tag="4")]
        pub inbound: ::core::option::Option<super::InboundRtpStreamStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutboundRtp {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub stream: ::core::option::Option<super::RtpStreamStats>,
        #[prost(message, optional, tag="3")]
        pub sent: ::core::option::Option<super::SentRtpStreamStats>,
        #[prost(message, optional, tag="4")]
        pub outbound: ::core::option::Option<super::OutboundRtpStreamStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteInboundRtp {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub stream: ::core::option::Option<super::RtpStreamStats>,
        #[prost(message, optional, tag="3")]
        pub received: ::core::option::Option<super::ReceivedRtpStreamStats>,
        #[prost(message, optional, tag="4")]
        pub remote_inbound: ::core::option::Option<super::RemoteInboundRtpStreamStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteOutboundRtp {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub stream: ::core::option::Option<super::RtpStreamStats>,
        #[prost(message, optional, tag="3")]
        pub sent: ::core::option::Option<super::SentRtpStreamStats>,
        #[prost(message, optional, tag="4")]
        pub remote_outbound: ::core::option::Option<super::RemoteOutboundRtpStreamStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MediaSource {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub source: ::core::option::Option<super::MediaSourceStats>,
        #[prost(message, optional, tag="3")]
        pub audio: ::core::option::Option<super::AudioSourceStats>,
        #[prost(message, optional, tag="4")]
        pub video: ::core::option::Option<super::VideoSourceStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MediaPlayout {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub audio_playout: ::core::option::Option<super::AudioPlayoutStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerConnection {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub pc: ::core::option::Option<super::PeerConnectionStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataChannel {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub dc: ::core::option::Option<super::DataChannelStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transport {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub transport: ::core::option::Option<super::TransportStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CandidatePair {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub candidate_pair: ::core::option::Option<super::CandidatePairStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalCandidate {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub candidate: ::core::option::Option<super::IceCandidateStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteCandidate {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub candidate: ::core::option::Option<super::IceCandidateStats>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Certificate {
        #[prost(message, optional, tag="1")]
        pub rtc: ::core::option::Option<super::RtcStatsData>,
        #[prost(message, optional, tag="2")]
        pub certificate: ::core::option::Option<super::CertificateStats>,
    }
    /// Deprecated
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Track {
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Stats {
        #[prost(message, tag="3")]
        Codec(Codec),
        #[prost(message, tag="4")]
        InboundRtp(InboundRtp),
        #[prost(message, tag="5")]
        OutboundRtp(OutboundRtp),
        #[prost(message, tag="6")]
        RemoteInboundRtp(RemoteInboundRtp),
        #[prost(message, tag="7")]
        RemoteOutboundRtp(RemoteOutboundRtp),
        #[prost(message, tag="8")]
        MediaSource(MediaSource),
        #[prost(message, tag="9")]
        MediaPlayout(MediaPlayout),
        #[prost(message, tag="10")]
        PeerConnection(PeerConnection),
        #[prost(message, tag="11")]
        DataChannel(DataChannel),
        #[prost(message, tag="12")]
        Transport(Transport),
        #[prost(message, tag="13")]
        CandidatePair(CandidatePair),
        #[prost(message, tag="14")]
        LocalCandidate(LocalCandidate),
        #[prost(message, tag="15")]
        RemoteCandidate(RemoteCandidate),
        #[prost(message, tag="16")]
        Certificate(Certificate),
        #[prost(message, tag="17")]
        Track(Track),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcStatsData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodecStats {
    #[prost(uint32, tag="1")]
    pub payload_type: u32,
    #[prost(string, tag="2")]
    pub transport_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mime_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub clock_rate: u32,
    #[prost(uint32, tag="5")]
    pub channels: u32,
    #[prost(string, tag="6")]
    pub sdp_fmtp_line: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtpStreamStats {
    #[prost(uint32, tag="1")]
    pub ssrc: u32,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub transport_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub codec_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedRtpStreamStats {
    #[prost(uint64, tag="1")]
    pub packets_received: u64,
    #[prost(int64, tag="2")]
    pub packets_lost: i64,
    #[prost(double, tag="3")]
    pub jitter: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InboundRtpStreamStats {
    #[prost(string, tag="1")]
    pub track_identifier: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub remote_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub frames_decoded: u32,
    #[prost(uint32, tag="5")]
    pub key_frames_decoded: u32,
    #[prost(uint32, tag="6")]
    pub frames_rendered: u32,
    #[prost(uint32, tag="7")]
    pub frames_dropped: u32,
    #[prost(uint32, tag="8")]
    pub frame_width: u32,
    #[prost(uint32, tag="9")]
    pub frame_height: u32,
    #[prost(double, tag="10")]
    pub frames_per_second: f64,
    #[prost(uint64, tag="11")]
    pub qp_sum: u64,
    #[prost(double, tag="12")]
    pub total_decode_time: f64,
    #[prost(double, tag="13")]
    pub total_inter_frame_delay: f64,
    #[prost(double, tag="14")]
    pub total_squared_inter_frame_delay: f64,
    #[prost(uint32, tag="15")]
    pub pause_count: u32,
    #[prost(double, tag="16")]
    pub total_pause_duration: f64,
    #[prost(uint32, tag="17")]
    pub freeze_count: u32,
    #[prost(double, tag="18")]
    pub total_freeze_duration: f64,
    #[prost(double, tag="19")]
    pub last_packet_received_timestamp: f64,
    #[prost(uint64, tag="20")]
    pub header_bytes_received: u64,
    #[prost(uint64, tag="21")]
    pub packets_discarded: u64,
    #[prost(uint64, tag="22")]
    pub fec_bytes_received: u64,
    #[prost(uint64, tag="23")]
    pub fec_packets_received: u64,
    #[prost(uint64, tag="24")]
    pub fec_packets_discarded: u64,
    #[prost(uint64, tag="25")]
    pub bytes_received: u64,
    #[prost(uint32, tag="26")]
    pub nack_count: u32,
    #[prost(uint32, tag="27")]
    pub fir_count: u32,
    #[prost(uint32, tag="28")]
    pub pli_count: u32,
    #[prost(double, tag="29")]
    pub total_processing_delay: f64,
    #[prost(double, tag="30")]
    pub estimated_playout_timestamp: f64,
    #[prost(double, tag="31")]
    pub jitter_buffer_delay: f64,
    #[prost(double, tag="32")]
    pub jitter_buffer_target_delay: f64,
    #[prost(uint64, tag="33")]
    pub jitter_buffer_emitted_count: u64,
    #[prost(double, tag="34")]
    pub jitter_buffer_minimum_delay: f64,
    #[prost(uint64, tag="35")]
    pub total_samples_received: u64,
    #[prost(uint64, tag="36")]
    pub concealed_samples: u64,
    #[prost(uint64, tag="37")]
    pub silent_concealed_samples: u64,
    #[prost(uint64, tag="38")]
    pub concealment_events: u64,
    #[prost(uint64, tag="39")]
    pub inserted_samples_for_deceleration: u64,
    #[prost(uint64, tag="40")]
    pub removed_samples_for_acceleration: u64,
    #[prost(double, tag="41")]
    pub audio_level: f64,
    #[prost(double, tag="42")]
    pub total_audio_energy: f64,
    #[prost(double, tag="43")]
    pub total_samples_duration: f64,
    #[prost(uint64, tag="44")]
    pub frames_received: u64,
    #[prost(string, tag="45")]
    pub decoder_implementation: ::prost::alloc::string::String,
    #[prost(string, tag="46")]
    pub playout_id: ::prost::alloc::string::String,
    #[prost(bool, tag="47")]
    pub power_efficient_decoder: bool,
    #[prost(uint64, tag="48")]
    pub frames_assembled_from_multiple_packets: u64,
    #[prost(double, tag="49")]
    pub total_assembly_time: f64,
    #[prost(uint64, tag="50")]
    pub retransmitted_packets_received: u64,
    #[prost(uint64, tag="51")]
    pub retransmitted_bytes_received: u64,
    #[prost(uint32, tag="52")]
    pub rtx_ssrc: u32,
    #[prost(uint32, tag="53")]
    pub fec_ssrc: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentRtpStreamStats {
    #[prost(uint64, tag="1")]
    pub packets_sent: u64,
    #[prost(uint64, tag="2")]
    pub bytes_sent: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutboundRtpStreamStats {
    #[prost(string, tag="1")]
    pub mid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub media_source_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub remote_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub rid: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub header_bytes_sent: u64,
    #[prost(uint64, tag="6")]
    pub retransmitted_packets_sent: u64,
    #[prost(uint64, tag="7")]
    pub retransmitted_bytes_sent: u64,
    #[prost(uint32, tag="8")]
    pub rtx_ssrc: u32,
    #[prost(double, tag="9")]
    pub target_bitrate: f64,
    #[prost(uint64, tag="10")]
    pub total_encoded_bytes_target: u64,
    #[prost(uint32, tag="11")]
    pub frame_width: u32,
    #[prost(uint32, tag="12")]
    pub frame_height: u32,
    #[prost(double, tag="13")]
    pub frames_per_second: f64,
    #[prost(uint32, tag="14")]
    pub frames_sent: u32,
    #[prost(uint32, tag="15")]
    pub huge_frames_sent: u32,
    #[prost(uint32, tag="16")]
    pub frames_encoded: u32,
    #[prost(uint32, tag="17")]
    pub key_frames_encoded: u32,
    #[prost(uint64, tag="18")]
    pub qp_sum: u64,
    #[prost(double, tag="19")]
    pub total_encode_time: f64,
    #[prost(double, tag="20")]
    pub total_packet_send_delay: f64,
    #[prost(enumeration="QualityLimitationReason", tag="21")]
    pub quality_limitation_reason: i32,
    #[prost(map="string, double", tag="22")]
    pub quality_limitation_durations: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(uint32, tag="23")]
    pub quality_limitation_resolution_changes: u32,
    #[prost(uint32, tag="24")]
    pub nack_count: u32,
    #[prost(uint32, tag="25")]
    pub fir_count: u32,
    #[prost(uint32, tag="26")]
    pub pli_count: u32,
    #[prost(string, tag="27")]
    pub encoder_implementation: ::prost::alloc::string::String,
    #[prost(bool, tag="28")]
    pub power_efficient_encoder: bool,
    #[prost(bool, tag="29")]
    pub active: bool,
    #[prost(string, tag="30")]
    pub scalibility_mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteInboundRtpStreamStats {
    #[prost(string, tag="1")]
    pub local_id: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub round_trip_time: f64,
    #[prost(double, tag="3")]
    pub total_round_trip_time: f64,
    #[prost(double, tag="4")]
    pub fraction_lost: f64,
    #[prost(uint64, tag="5")]
    pub round_trip_time_measurements: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteOutboundRtpStreamStats {
    #[prost(string, tag="1")]
    pub local_id: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub remote_timestamp: f64,
    #[prost(uint64, tag="3")]
    pub reports_sent: u64,
    #[prost(double, tag="4")]
    pub round_trip_time: f64,
    #[prost(double, tag="5")]
    pub total_round_trip_time: f64,
    #[prost(uint64, tag="6")]
    pub round_trip_time_measurements: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaSourceStats {
    #[prost(string, tag="1")]
    pub track_identifier: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioSourceStats {
    #[prost(double, tag="1")]
    pub audio_level: f64,
    #[prost(double, tag="2")]
    pub total_audio_energy: f64,
    #[prost(double, tag="3")]
    pub total_samples_duration: f64,
    #[prost(double, tag="4")]
    pub echo_return_loss: f64,
    #[prost(double, tag="5")]
    pub echo_return_loss_enhancement: f64,
    #[prost(double, tag="6")]
    pub dropped_samples_duration: f64,
    #[prost(uint32, tag="7")]
    pub dropped_samples_events: u32,
    #[prost(double, tag="8")]
    pub total_capture_delay: f64,
    #[prost(uint64, tag="9")]
    pub total_samples_captured: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSourceStats {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(uint32, tag="3")]
    pub frames: u32,
    #[prost(double, tag="4")]
    pub frames_per_second: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioPlayoutStats {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub synthesized_samples_duration: f64,
    #[prost(uint32, tag="3")]
    pub synthesized_samples_events: u32,
    #[prost(double, tag="4")]
    pub total_samples_duration: f64,
    #[prost(double, tag="5")]
    pub total_playout_delay: f64,
    #[prost(uint64, tag="6")]
    pub total_samples_count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerConnectionStats {
    #[prost(uint32, tag="1")]
    pub data_channels_opened: u32,
    #[prost(uint32, tag="2")]
    pub data_channels_closed: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataChannelStats {
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub protocol: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub data_channel_identifier: i32,
    #[prost(enumeration="DataChannelState", optional, tag="4")]
    pub state: ::core::option::Option<i32>,
    #[prost(uint32, tag="5")]
    pub messages_sent: u32,
    #[prost(uint64, tag="6")]
    pub bytes_sent: u64,
    #[prost(uint32, tag="7")]
    pub messages_received: u32,
    #[prost(uint64, tag="8")]
    pub bytes_received: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransportStats {
    #[prost(uint64, tag="1")]
    pub packets_sent: u64,
    #[prost(uint64, tag="2")]
    pub packets_received: u64,
    #[prost(uint64, tag="3")]
    pub bytes_sent: u64,
    #[prost(uint64, tag="4")]
    pub bytes_received: u64,
    #[prost(enumeration="IceRole", tag="5")]
    pub ice_role: i32,
    #[prost(string, tag="6")]
    pub ice_local_username_fragment: ::prost::alloc::string::String,
    #[prost(enumeration="DtlsTransportState", optional, tag="7")]
    pub dtls_state: ::core::option::Option<i32>,
    #[prost(enumeration="IceTransportState", optional, tag="8")]
    pub ice_state: ::core::option::Option<i32>,
    #[prost(string, tag="9")]
    pub selected_candidate_pair_id: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub local_certificate_id: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub remote_certificate_id: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub tls_version: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub dtls_cipher: ::prost::alloc::string::String,
    #[prost(enumeration="DtlsRole", tag="14")]
    pub dtls_role: i32,
    #[prost(string, tag="15")]
    pub srtp_cipher: ::prost::alloc::string::String,
    #[prost(uint32, tag="16")]
    pub selected_candidate_pair_changes: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandidatePairStats {
    #[prost(string, tag="1")]
    pub transport_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub local_candidate_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub remote_candidate_id: ::prost::alloc::string::String,
    #[prost(enumeration="IceCandidatePairState", optional, tag="4")]
    pub state: ::core::option::Option<i32>,
    #[prost(bool, tag="5")]
    pub nominated: bool,
    #[prost(uint64, tag="6")]
    pub packets_sent: u64,
    #[prost(uint64, tag="7")]
    pub packets_received: u64,
    #[prost(uint64, tag="8")]
    pub bytes_sent: u64,
    #[prost(uint64, tag="9")]
    pub bytes_received: u64,
    #[prost(double, tag="10")]
    pub last_packet_sent_timestamp: f64,
    #[prost(double, tag="11")]
    pub last_packet_received_timestamp: f64,
    #[prost(double, tag="12")]
    pub total_round_trip_time: f64,
    #[prost(double, tag="13")]
    pub current_round_trip_time: f64,
    #[prost(double, tag="14")]
    pub available_outgoing_bitrate: f64,
    #[prost(double, tag="15")]
    pub available_incoming_bitrate: f64,
    #[prost(uint64, tag="16")]
    pub requests_received: u64,
    #[prost(uint64, tag="17")]
    pub requests_sent: u64,
    #[prost(uint64, tag="18")]
    pub responses_received: u64,
    #[prost(uint64, tag="19")]
    pub responses_sent: u64,
    #[prost(uint64, tag="20")]
    pub consent_requests_sent: u64,
    #[prost(uint32, tag="21")]
    pub packets_discarded_on_send: u32,
    #[prost(uint64, tag="22")]
    pub bytes_discarded_on_send: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IceCandidateStats {
    #[prost(string, tag="1")]
    pub transport_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub port: i32,
    #[prost(string, tag="4")]
    pub protocol: ::prost::alloc::string::String,
    #[prost(enumeration="IceCandidateType", optional, tag="5")]
    pub candidate_type: ::core::option::Option<i32>,
    #[prost(int32, tag="6")]
    pub priority: i32,
    #[prost(string, tag="7")]
    pub url: ::prost::alloc::string::String,
    #[prost(enumeration="IceServerTransportProtocol", optional, tag="8")]
    pub relay_protocol: ::core::option::Option<i32>,
    #[prost(string, tag="9")]
    pub foundation: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub related_address: ::prost::alloc::string::String,
    #[prost(int32, tag="11")]
    pub related_port: i32,
    #[prost(string, tag="12")]
    pub username_fragment: ::prost::alloc::string::String,
    #[prost(enumeration="IceTcpCandidateType", optional, tag="13")]
    pub tcp_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateStats {
    #[prost(string, tag="1")]
    pub fingerprint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fingerprint_algorithm: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub base64_certificate: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub issuer_certificate_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataChannelState {
    DcConnecting = 0,
    DcOpen = 1,
    DcClosing = 2,
    DcClosed = 3,
}
impl DataChannelState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataChannelState::DcConnecting => "DC_CONNECTING",
            DataChannelState::DcOpen => "DC_OPEN",
            DataChannelState::DcClosing => "DC_CLOSING",
            DataChannelState::DcClosed => "DC_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DC_CONNECTING" => Some(Self::DcConnecting),
            "DC_OPEN" => Some(Self::DcOpen),
            "DC_CLOSING" => Some(Self::DcClosing),
            "DC_CLOSED" => Some(Self::DcClosed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QualityLimitationReason {
    LimitationNone = 0,
    LimitationCpu = 1,
    LimitationBandwidth = 2,
    LimitationOther = 3,
}
impl QualityLimitationReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QualityLimitationReason::LimitationNone => "LIMITATION_NONE",
            QualityLimitationReason::LimitationCpu => "LIMITATION_CPU",
            QualityLimitationReason::LimitationBandwidth => "LIMITATION_BANDWIDTH",
            QualityLimitationReason::LimitationOther => "LIMITATION_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIMITATION_NONE" => Some(Self::LimitationNone),
            "LIMITATION_CPU" => Some(Self::LimitationCpu),
            "LIMITATION_BANDWIDTH" => Some(Self::LimitationBandwidth),
            "LIMITATION_OTHER" => Some(Self::LimitationOther),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceRole {
    IceUnknown = 0,
    IceControlling = 1,
    IceControlled = 2,
}
impl IceRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceRole::IceUnknown => "ICE_UNKNOWN",
            IceRole::IceControlling => "ICE_CONTROLLING",
            IceRole::IceControlled => "ICE_CONTROLLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ICE_UNKNOWN" => Some(Self::IceUnknown),
            "ICE_CONTROLLING" => Some(Self::IceControlling),
            "ICE_CONTROLLED" => Some(Self::IceControlled),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DtlsTransportState {
    DtlsTransportNew = 0,
    DtlsTransportConnecting = 1,
    DtlsTransportConnected = 2,
    DtlsTransportClosed = 3,
    DtlsTransportFailed = 4,
}
impl DtlsTransportState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DtlsTransportState::DtlsTransportNew => "DTLS_TRANSPORT_NEW",
            DtlsTransportState::DtlsTransportConnecting => "DTLS_TRANSPORT_CONNECTING",
            DtlsTransportState::DtlsTransportConnected => "DTLS_TRANSPORT_CONNECTED",
            DtlsTransportState::DtlsTransportClosed => "DTLS_TRANSPORT_CLOSED",
            DtlsTransportState::DtlsTransportFailed => "DTLS_TRANSPORT_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DTLS_TRANSPORT_NEW" => Some(Self::DtlsTransportNew),
            "DTLS_TRANSPORT_CONNECTING" => Some(Self::DtlsTransportConnecting),
            "DTLS_TRANSPORT_CONNECTED" => Some(Self::DtlsTransportConnected),
            "DTLS_TRANSPORT_CLOSED" => Some(Self::DtlsTransportClosed),
            "DTLS_TRANSPORT_FAILED" => Some(Self::DtlsTransportFailed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceTransportState {
    IceTransportNew = 0,
    IceTransportChecking = 1,
    IceTransportConnected = 2,
    IceTransportCompleted = 3,
    IceTransportDisconnected = 4,
    IceTransportFailed = 5,
    IceTransportClosed = 6,
}
impl IceTransportState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceTransportState::IceTransportNew => "ICE_TRANSPORT_NEW",
            IceTransportState::IceTransportChecking => "ICE_TRANSPORT_CHECKING",
            IceTransportState::IceTransportConnected => "ICE_TRANSPORT_CONNECTED",
            IceTransportState::IceTransportCompleted => "ICE_TRANSPORT_COMPLETED",
            IceTransportState::IceTransportDisconnected => "ICE_TRANSPORT_DISCONNECTED",
            IceTransportState::IceTransportFailed => "ICE_TRANSPORT_FAILED",
            IceTransportState::IceTransportClosed => "ICE_TRANSPORT_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ICE_TRANSPORT_NEW" => Some(Self::IceTransportNew),
            "ICE_TRANSPORT_CHECKING" => Some(Self::IceTransportChecking),
            "ICE_TRANSPORT_CONNECTED" => Some(Self::IceTransportConnected),
            "ICE_TRANSPORT_COMPLETED" => Some(Self::IceTransportCompleted),
            "ICE_TRANSPORT_DISCONNECTED" => Some(Self::IceTransportDisconnected),
            "ICE_TRANSPORT_FAILED" => Some(Self::IceTransportFailed),
            "ICE_TRANSPORT_CLOSED" => Some(Self::IceTransportClosed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DtlsRole {
    DtlsClient = 0,
    DtlsServer = 1,
    DtlsUnknown = 2,
}
impl DtlsRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DtlsRole::DtlsClient => "DTLS_CLIENT",
            DtlsRole::DtlsServer => "DTLS_SERVER",
            DtlsRole::DtlsUnknown => "DTLS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DTLS_CLIENT" => Some(Self::DtlsClient),
            "DTLS_SERVER" => Some(Self::DtlsServer),
            "DTLS_UNKNOWN" => Some(Self::DtlsUnknown),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceCandidatePairState {
    PairFrozen = 0,
    PairWaiting = 1,
    PairInProgress = 2,
    PairFailed = 3,
    PairSucceeded = 4,
}
impl IceCandidatePairState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceCandidatePairState::PairFrozen => "PAIR_FROZEN",
            IceCandidatePairState::PairWaiting => "PAIR_WAITING",
            IceCandidatePairState::PairInProgress => "PAIR_IN_PROGRESS",
            IceCandidatePairState::PairFailed => "PAIR_FAILED",
            IceCandidatePairState::PairSucceeded => "PAIR_SUCCEEDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAIR_FROZEN" => Some(Self::PairFrozen),
            "PAIR_WAITING" => Some(Self::PairWaiting),
            "PAIR_IN_PROGRESS" => Some(Self::PairInProgress),
            "PAIR_FAILED" => Some(Self::PairFailed),
            "PAIR_SUCCEEDED" => Some(Self::PairSucceeded),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceCandidateType {
    Host = 0,
    Srflx = 1,
    Prflx = 2,
    Relay = 3,
}
impl IceCandidateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceCandidateType::Host => "HOST",
            IceCandidateType::Srflx => "SRFLX",
            IceCandidateType::Prflx => "PRFLX",
            IceCandidateType::Relay => "RELAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOST" => Some(Self::Host),
            "SRFLX" => Some(Self::Srflx),
            "PRFLX" => Some(Self::Prflx),
            "RELAY" => Some(Self::Relay),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceServerTransportProtocol {
    TransportUdp = 0,
    TransportTcp = 1,
    TransportTls = 2,
}
impl IceServerTransportProtocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceServerTransportProtocol::TransportUdp => "TRANSPORT_UDP",
            IceServerTransportProtocol::TransportTcp => "TRANSPORT_TCP",
            IceServerTransportProtocol::TransportTls => "TRANSPORT_TLS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSPORT_UDP" => Some(Self::TransportUdp),
            "TRANSPORT_TCP" => Some(Self::TransportTcp),
            "TRANSPORT_TLS" => Some(Self::TransportTls),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IceTcpCandidateType {
    CandidateActive = 0,
    CandidatePassive = 1,
    CandidateSo = 2,
}
impl IceTcpCandidateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IceTcpCandidateType::CandidateActive => "CANDIDATE_ACTIVE",
            IceTcpCandidateType::CandidatePassive => "CANDIDATE_PASSIVE",
            IceTcpCandidateType::CandidateSo => "CANDIDATE_SO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CANDIDATE_ACTIVE" => Some(Self::CandidateActive),
            "CANDIDATE_PASSIVE" => Some(Self::CandidatePassive),
            "CANDIDATE_SO" => Some(Self::CandidateSo),
            _ => None,
        }
    }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsRequest {
    #[prost(uint64, tag="1")]
    pub track_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub stats: ::prost::alloc::vec::Vec<RtcStats>,
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
/// Create a new VideoStream
/// VideoStream is used to receive video frames from a track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVideoStreamRequest {
    #[prost(uint64, tag="1")]
    pub track_handle: u64,
    #[prost(enumeration="VideoStreamType", tag="2")]
    pub r#type: i32,
    /// Get the frame on a specific format
    #[prost(enumeration="VideoBufferType", optional, tag="3")]
    pub format: ::core::option::Option<i32>,
    /// if true, stride will be set to width/chroma_width
    #[prost(bool, tag="4")]
    pub normalize_stride: bool,
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
    pub buffer: ::core::option::Option<VideoBufferInfo>,
    /// In microseconds
    #[prost(int64, tag="3")]
    pub timestamp_us: i64,
    #[prost(enumeration="VideoRotation", tag="4")]
    pub rotation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureVideoFrameResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoConvertRequest {
    #[prost(bool, tag="1")]
    pub flip_y: bool,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<VideoBufferInfo>,
    #[prost(enumeration="VideoBufferType", tag="3")]
    pub dst_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoConvertResponse {
    #[prost(string, optional, tag="1")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<OwnedVideoBuffer>,
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
pub struct VideoBufferInfo {
    #[prost(enumeration="VideoBufferType", tag="1")]
    pub r#type: i32,
    #[prost(uint32, tag="2")]
    pub width: u32,
    #[prost(uint32, tag="3")]
    pub height: u32,
    #[prost(uint64, tag="4")]
    pub data_ptr: u64,
    /// only for packed formats
    #[prost(uint32, tag="6")]
    pub stride: u32,
    #[prost(message, repeated, tag="7")]
    pub components: ::prost::alloc::vec::Vec<video_buffer_info::ComponentInfo>,
}
/// Nested message and enum types in `VideoBufferInfo`.
pub mod video_buffer_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComponentInfo {
        #[prost(uint64, tag="1")]
        pub data_ptr: u64,
        #[prost(uint32, tag="2")]
        pub stride: u32,
        #[prost(uint32, tag="3")]
        pub size: u32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnedVideoBuffer {
    #[prost(message, optional, tag="1")]
    pub handle: ::core::option::Option<FfiOwnedHandle>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<VideoBufferInfo>,
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
    pub buffer: ::core::option::Option<OwnedVideoBuffer>,
    /// In microseconds
    #[prost(int64, tag="2")]
    pub timestamp_us: i64,
    #[prost(enumeration="VideoRotation", tag="3")]
    pub rotation: i32,
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
    Vp9 = 3,
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
            VideoCodec::Vp9 => "VP9",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VP8" => Some(Self::Vp8),
            "H264" => Some(Self::H264),
            "AV1" => Some(Self::Av1),
            "VP9" => Some(Self::Vp9),
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
pub enum VideoBufferType {
    Rgba = 0,
    Abgr = 1,
    Argb = 2,
    Bgra = 3,
    Rgb24 = 4,
    I420 = 5,
    I420a = 6,
    I422 = 7,
    I444 = 8,
    I010 = 9,
    Nv12 = 10,
}
impl VideoBufferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoBufferType::Rgba => "RGBA",
            VideoBufferType::Abgr => "ABGR",
            VideoBufferType::Argb => "ARGB",
            VideoBufferType::Bgra => "BGRA",
            VideoBufferType::Rgb24 => "RGB24",
            VideoBufferType::I420 => "I420",
            VideoBufferType::I420a => "I420A",
            VideoBufferType::I422 => "I422",
            VideoBufferType::I444 => "I444",
            VideoBufferType::I010 => "I010",
            VideoBufferType::Nv12 => "NV12",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RGBA" => Some(Self::Rgba),
            "ABGR" => Some(Self::Abgr),
            "ARGB" => Some(Self::Argb),
            "BGRA" => Some(Self::Bgra),
            "RGB24" => Some(Self::Rgb24),
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
    #[prost(string, optional, tag="6")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionStatsRequest {
    #[prost(uint64, tag="1")]
    pub room_handle: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionStatsResponse {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionStatsCallback {
    #[prost(uint64, tag="1")]
    pub async_id: u64,
    #[prost(string, optional, tag="2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub publisher_stats: ::prost::alloc::vec::Vec<RtcStats>,
    #[prost(message, repeated, tag="4")]
    pub subscriber_stats: ::prost::alloc::vec::Vec<RtcStats>,
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
    #[prost(uint32, tag="6")]
    pub join_retries: u32,
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
    #[prost(oneof="room_event::Message", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 21, 22, 23, 24, 25, 26")]
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
        #[prost(message, tag="26")]
        DataPacketReceived(super::DataPacketReceived),
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
pub struct UserPacket {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<OwnedBuffer>,
    #[prost(string, optional, tag="2")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SipDtmf {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, optional, tag="2")]
    pub digit: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataPacketReceived {
    #[prost(enumeration="DataPacketKind", tag="1")]
    pub kind: i32,
    /// Can be empty if the data is sent a server SDK
    #[prost(string, tag="2")]
    pub participant_identity: ::prost::alloc::string::String,
    /// Can be empty if the data is sent a server SDK
    #[deprecated]
    #[prost(string, optional, tag="3")]
    pub participant_sid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="data_packet_received::Value", tags="4, 5")]
    pub value: ::core::option::Option<data_packet_received::Value>,
}
/// Nested message and enum types in `DataPacketReceived`.
pub mod data_packet_received {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="4")]
        User(super::UserPacket),
        #[prost(message, tag="5")]
        SipDtmf(super::SipDtmf),
    }
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
    QualityLost = 3,
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
            ConnectionQuality::QualityLost => "QUALITY_LOST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUALITY_POOR" => Some(Self::QualityPoor),
            "QUALITY_GOOD" => Some(Self::QualityGood),
            "QUALITY_EXCELLENT" => Some(Self::QualityExcellent),
            "QUALITY_LOST" => Some(Self::QualityLost),
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
    #[prost(oneof="ffi_request::Message", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 22, 23, 24, 25, 26, 27")]
    pub message: ::core::option::Option<ffi_request::Message>,
}
/// Nested message and enum types in `FfiRequest`.
pub mod ffi_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
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
        #[prost(message, tag="11")]
        GetSessionStats(super::GetSessionStatsRequest),
        /// Track
        #[prost(message, tag="12")]
        CreateVideoTrack(super::CreateVideoTrackRequest),
        #[prost(message, tag="13")]
        CreateAudioTrack(super::CreateAudioTrackRequest),
        #[prost(message, tag="14")]
        GetStats(super::GetStatsRequest),
        /// Video
        #[prost(message, tag="16")]
        NewVideoStream(super::NewVideoStreamRequest),
        #[prost(message, tag="17")]
        NewVideoSource(super::NewVideoSourceRequest),
        #[prost(message, tag="18")]
        CaptureVideoFrame(super::CaptureVideoFrameRequest),
        #[prost(message, tag="19")]
        VideoConvert(super::VideoConvertRequest),
        /// Audio
        #[prost(message, tag="22")]
        NewAudioStream(super::NewAudioStreamRequest),
        #[prost(message, tag="23")]
        NewAudioSource(super::NewAudioSourceRequest),
        #[prost(message, tag="24")]
        CaptureAudioFrame(super::CaptureAudioFrameRequest),
        #[prost(message, tag="25")]
        NewAudioResampler(super::NewAudioResamplerRequest),
        #[prost(message, tag="26")]
        RemixAndResample(super::RemixAndResampleRequest),
        #[prost(message, tag="27")]
        E2ee(super::E2eeRequest),
    }
}
/// This is the output of livekit_ffi_request function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiResponse {
    #[prost(oneof="ffi_response::Message", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 22, 23, 24, 25, 26, 27")]
    pub message: ::core::option::Option<ffi_response::Message>,
}
/// Nested message and enum types in `FfiResponse`.
pub mod ffi_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
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
        #[prost(message, tag="11")]
        GetSessionStats(super::GetSessionStatsResponse),
        /// Track
        #[prost(message, tag="12")]
        CreateVideoTrack(super::CreateVideoTrackResponse),
        #[prost(message, tag="13")]
        CreateAudioTrack(super::CreateAudioTrackResponse),
        #[prost(message, tag="14")]
        GetStats(super::GetStatsResponse),
        /// Video
        #[prost(message, tag="16")]
        NewVideoStream(super::NewVideoStreamResponse),
        #[prost(message, tag="17")]
        NewVideoSource(super::NewVideoSourceResponse),
        #[prost(message, tag="18")]
        CaptureVideoFrame(super::CaptureVideoFrameResponse),
        #[prost(message, tag="19")]
        VideoConvert(super::VideoConvertResponse),
        /// Audio
        #[prost(message, tag="22")]
        NewAudioStream(super::NewAudioStreamResponse),
        #[prost(message, tag="23")]
        NewAudioSource(super::NewAudioSourceResponse),
        #[prost(message, tag="24")]
        CaptureAudioFrame(super::CaptureAudioFrameResponse),
        #[prost(message, tag="25")]
        NewAudioResampler(super::NewAudioResamplerResponse),
        #[prost(message, tag="26")]
        RemixAndResample(super::RemixAndResampleResponse),
        #[prost(message, tag="27")]
        E2ee(super::E2eeResponse),
    }
}
/// To minimize complexity, participant events are not included in the protocol.
/// It is easily deducible from the room events and it turned out that is is easier to implement
/// on the ffi client side.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FfiEvent {
    #[prost(oneof="ffi_event::Message", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17")]
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
        #[prost(message, tag="14")]
        GetStats(super::GetStatsCallback),
        #[prost(message, tag="15")]
        Logs(super::LogBatch),
        #[prost(message, tag="16")]
        GetSessionStats(super::GetSessionStatsCallback),
        #[prost(message, tag="17")]
        Panic(super::Panic),
    }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRecord {
    #[prost(enumeration="LogLevel", tag="1")]
    pub level: i32,
    /// e.g "livekit", "libwebrtc", "tokio-tungstenite", etc...
    #[prost(string, tag="2")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub module_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub file: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="5")]
    pub line: ::core::option::Option<u32>,
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogBatch {
    #[prost(message, repeated, tag="1")]
    pub records: ::prost::alloc::vec::Vec<LogRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Panic {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    LogError = 0,
    LogWarn = 1,
    LogInfo = 2,
    LogDebug = 3,
    LogTrace = 4,
}
impl LogLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogLevel::LogError => "LOG_ERROR",
            LogLevel::LogWarn => "LOG_WARN",
            LogLevel::LogInfo => "LOG_INFO",
            LogLevel::LogDebug => "LOG_DEBUG",
            LogLevel::LogTrace => "LOG_TRACE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_ERROR" => Some(Self::LogError),
            "LOG_WARN" => Some(Self::LogWarn),
            "LOG_INFO" => Some(Self::LogInfo),
            "LOG_DEBUG" => Some(Self::LogDebug),
            "LOG_TRACE" => Some(Self::LogTrace),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
