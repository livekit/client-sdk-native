use crate::{participant, track, DataPacketKind};
use livekit_protocol::*;

// Conversions
impl From<ConnectionQuality> for participant::ConnectionQuality {
    fn from(value: ConnectionQuality) -> Self {
        match value {
            ConnectionQuality::Excellent => Self::Excellent,
            ConnectionQuality::Good => Self::Good,
            ConnectionQuality::Poor => Self::Poor,
        }
    }
}

impl TryFrom<TrackType> for track::TrackKind {
    type Error = &'static str;

    fn try_from(r#type: TrackType) -> Result<Self, Self::Error> {
        match r#type {
            TrackType::Audio => Ok(Self::Audio),
            TrackType::Video => Ok(Self::Video),
            TrackType::Data => Err("data tracks are not implemented yet"),
        }
    }
}

impl From<track::TrackKind> for TrackType {
    fn from(kind: track::TrackKind) -> Self {
        match kind {
            track::TrackKind::Audio => Self::Audio,
            track::TrackKind::Video => Self::Video,
        }
    }
}

impl From<TrackSource> for track::TrackSource {
    fn from(source: TrackSource) -> Self {
        match source {
            TrackSource::Camera => Self::Camera,
            TrackSource::Microphone => Self::Microphone,
            TrackSource::ScreenShare => Self::Screenshare,
            TrackSource::ScreenShareAudio => Self::ScreenshareAudio,
            TrackSource::Unknown => Self::Unknown,
        }
    }
}

impl From<track::TrackSource> for TrackSource {
    fn from(source: track::TrackSource) -> Self {
        match source {
            track::TrackSource::Camera => Self::Camera,
            track::TrackSource::Microphone => Self::Microphone,
            track::TrackSource::Screenshare => Self::ScreenShare,
            track::TrackSource::ScreenshareAudio => Self::ScreenShareAudio,
            track::TrackSource::Unknown => Self::Unknown,
        }
    }
}

impl From<DataPacketKind> for data_packet::Kind {
    fn from(kind: DataPacketKind) -> Self {
        match kind {
            DataPacketKind::Lossy => Self::Lossy,
            DataPacketKind::Reliable => Self::Reliable,
        }
    }
}

impl From<data_packet::Kind> for DataPacketKind {
    fn from(kind: data_packet::Kind) -> Self {
        match kind {
            data_packet::Kind::Lossy => Self::Lossy,
            data_packet::Kind::Reliable => Self::Reliable,
        }
    }
}
