// Structs from https://www.w3.org/TR/webrtc-stats/
// serde will handle the magic of correctly deserializing the json into these structs

use crate::data_channel::DataChannelState;
use serde::Deserialize;
use std::collections::HashMap;

// RtcStats enum (the json is flattened + tagged)
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum RtcStats {
    Codec {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        codec: CodecStats,
    },
    InboundRtp {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        stream: RtpStreamStats,

        #[serde(flatten)]
        received: ReceivedRtpStreamStats,

        #[serde(flatten)]
        inbound: InboundRtpStreamStats,
    },
    OutboundRtp {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        stream: RtpStreamStats,

        #[serde(flatten)]
        sent: SentRtpStreamStats,

        #[serde(flatten)]
        outbound: OutboundRtpStreamStats,
    },
    RemoteInboundRtp {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        stream: RtpStreamStats,

        #[serde(flatten)]
        received: ReceivedRtpStreamStats,

        #[serde(flatten)]
        remote_inbound: RemoteInboundRtpStreamStats,
    },
    RemoteOutboundRtp {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        stream: RtpStreamStats,

        #[serde(flatten)]
        sent: SentRtpStreamStats,

        #[serde(flatten)]
        remote_outbound: RemoteOutboundRtpStreamStats,
    },
    MediaSource {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        source: MediaSourceStats,

        #[serde(flatten)]
        audio: AudioSourceStats,

        #[serde(flatten)]
        video: VideoSourceStats,
    },
    MediaPlayout {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        audio_playout: AudioPlayoutStats,
    },
    PeerConnection {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        pc: PeerConnectionStats,
    },
    DataChannel {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        dc: DataChannelStats,
    },
    Transport {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        transport: TransportStats,
    },
    CandidatePair {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        candidate_pair: CandidatePairStats,
    },
    LocalCandidate {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        candidate: IceCandidateStats,
    },
    RemoteCandidate {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        candidate: IceCandidateStats,
    },
    Certificate {
        #[serde(flatten)]
        rtc: RtcStatsData,

        #[serde(flatten)]
        certificate: CertificateStats,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityLimitationReason {
    None,
    CPU,
    Bandwidth,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceRole {
    Unknown,
    Controlling,
    Controlled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DtlsTransportState {
    New,
    Connecting,
    Connected,
    Closed,
    Failed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceTransportState {
    New,
    Checking,
    Connected,
    Completed,
    Disconnected,
    Failed,
    Closed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DtlsRole {
    Client,
    Server,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IceCandidatePairState {
    Frozen,
    Waiting,
    InProgress, // in-progress
    Failed,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceCandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceServerTransportProtocol {
    Udp,
    Tcp,
    Tls,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IceTcpCandidateType {
    Active,
    Passive,
    So,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcStatsData {
    pub id: String,
    pub r#type: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecStats {
    pub payload_type: u32,
    pub transport_id: String,
    pub mime_type: String,
    pub clock_rate: u32,
    pub channels: u32,
    pub sdp_fmtp_line: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtpStreamStats {
    pub ssrc: u32,
    pub kind: String,
    pub transport_id: String,
    pub codec_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedRtpStreamStats {
    pub packets_received: u64,
    pub packets_lost: i64,
    pub jitter: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundRtpStreamStats {
    pub track_identifier: String,
    pub mid: String,
    pub remote_id: String,
    pub frames_decoded: u32,
    pub key_frames_decoded: u32,
    pub frames_rendered: u32,
    pub frames_dropped: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frames_per_second: f64,
    pub qp_sum: u64,
    pub total_decode_time: f64,
    pub total_inter_frame_delay: f64,
    pub total_squared_inter_frame_delay: f64,
    pub pause_count: u32,
    pub total_pause_duration: f64,
    pub freeze_count: u32,
    pub total_freeze_duration: f64,
    pub last_packet_received_timestamp: f64,
    pub header_bytes_received: u64,
    pub packets_discarded: u64,
    pub fec_bytes_received: u64,
    pub fec_packets_received: u64,
    pub fec_packets_discarded: u64,
    pub bytes_received: u64,
    pub nack_count: u32,
    pub fir_count: u32,
    pub pli_count: u32,
    pub total_processing_delay: f64,
    pub estimated_playout_timestamp: f64,
    pub jitter_buffer_delay: f64,
    pub jitter_buffer_target_delay: f64,
    pub jitter_buffer_emitted_count: u64,
    pub jitter_buffer_minimum_delay: f64,
    pub total_samples_received: u64,
    pub concealed_samples: u64,
    pub silent_concealed_samples: u64,
    pub concealment_events: u64,
    pub inserted_samples_for_deceleration: u64,
    pub removed_samples_for_acceleration: u64,
    pub audio_level: f64,
    pub total_audio_energy: f64,
    pub total_samples_duration: f64,
    pub frames_received: u64,
    pub decoder_implementation: String,
    pub playout_id: String,
    pub power_efficient_decoder: bool,
    pub frames_assembled_from_multiple_packets: u64,
    pub total_assembly_time: f64,
    pub retransmitted_packets_received: u64,
    pub retransmitted_bytes_received: u64,
    pub rtx_ssrc: u32,
    pub fec_ssrc: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentRtpStreamStats {
    pub packets_sent: u64,
    pub bytes_sent: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundRtpStreamStats {
    pub mid: String,
    pub media_source_id: String,
    pub remote_id: String,
    pub rid: String,
    pub header_bytes_sent: u64,
    pub retransmitted_packets_sent: u64,
    pub retransmitted_bytes_sent: u64,
    pub rtx_ssrc: u32,
    pub target_bitrate: f64,
    pub total_encoded_bytes_target: u64,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frames_per_second: f64,
    pub frames_sent: u32,
    pub huge_frames_sent: u32,
    pub frames_encoded: u32,
    pub key_frames_encoded: u32,
    pub qp_sum: u64,
    pub total_encode_time: f64,
    pub total_packet_send_delay: f64,
    pub quality_limitation_reason: QualityLimitationReason,
    pub quality_limitation_durations: HashMap<String, f64>,
    pub quality_limitation_resolution_changes: u32,
    pub nack_count: u32,
    pub fir_count: u32,
    pub pli_count: u32,
    pub encoder_implementation: String,
    pub power_efficient_encoder: bool,
    pub active: bool,
    pub scalibility_mode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteInboundRtpStreamStats {
    pub local_id: String,
    pub round_trip_time: f64,
    pub total_round_trip_time: f64,
    pub fraction_lost: f64,
    pub round_trip_time_measurements: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteOutboundRtpStreamStats {
    pub local_id: String,
    pub remote_timestamp: f64,
    pub reports_sent: u64,
    pub round_trip_time: f64,
    pub total_round_trip_time: f64,
    pub round_trip_time_measurements: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSourceStats {
    pub track_identifier: String,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSourceStats {
    pub audio_level: f64,
    pub total_audio_energy: f64,
    pub total_samples_duration: f64,
    pub echo_return_loss: f64,
    pub echo_return_loss_enhancement: f64,
    pub dropped_samples_duration: f64,
    pub dropped_samples_events: u32,
    pub total_capture_delay: f64,
    pub total_samples_captured: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSourceStats {
    pub width: u32,
    pub height: u32,
    pub frames: u32,
    pub frames_per_second: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioPlayoutStats {
    pub kind: String,
    pub synthesized_samples_duration: f64,
    pub synthesized_samples_events: u32,
    pub total_samples_duration: f64,
    pub total_playout_delay: f64,
    pub total_samples_count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerConnectionStats {
    pub data_channels_opened: u32,
    pub data_channels_closed: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataChannelStats {
    pub label: String,
    pub protocol: String,
    pub data_channel_identifier: u16,
    pub state: DataChannelState,
    pub messages_sent: u32,
    pub bytes_sent: u64,
    pub messages_received: u32,
    pub bytes_received: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportStats {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub ice_role: IceRole,
    pub ice_local_username_fragment: String,
    pub dtls_state: DtlsTransportState,
    pub ice_state: IceTransportState,
    pub selected_candidate_pair_id: String,
    pub local_certificate_id: String,
    pub remote_certificate_id: String,
    pub tls_version: String,
    pub dtls_cipher: String,
    pub dtls_role: DtlsRole,
    pub srtp_cipher: String,
    pub selected_candidate_pair_changes: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidatePairStats {
    pub transport_id: String,
    pub local_candidate_id: String,
    pub remote_candidate_id: String,
    pub state: IceCandidatePairState,
    pub nominated: bool,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_packet_sent_timestamp: f64,
    pub last_packet_received_timestamp: f64,
    pub total_round_trip_time: f64,
    pub current_round_trip_time: f64,
    pub available_outgoing_bitrate: f64,
    pub available_incoming_bitrate: f64,
    pub requests_received: u64,
    pub requests_sent: u64,
    pub responses_received: u64,
    pub responses_sent: u64,
    pub consent_requests_sent: u64,
    pub packets_discarded_on_send: u32,
    pub bytes_discarded_on_send: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IceCandidateStats {
    pub transport_id: String,
    pub address: String,
    pub port: i32,
    pub protocol: String,
    pub candidate_type: IceCandidateType,
    pub priority: i32,
    pub url: String,
    pub relay_protocol: IceServerTransportProtocol,
    pub foundation: String,
    pub related_address: String,
    pub related_port: i32,
    pub username_fragment: String,
    pub tcp_type: IceTcpCandidateType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStats {
    pub fingerprint: String,
    pub fingerprint_algorithm: String,
    pub base64_certificate: String,
    pub issuer_certificate_id: String,
}
