use livekit::prelude::{RoomEvent, Track, TrackSource};
use tokio::sync::mpsc;

use super::{
    room::{FfiParticipant},
};
use crate::{server, FfiError, FfiHandleId};

pub async fn track_changed_trigger(
    participant: FfiParticipant,
    track_source: TrackSource,
    track_tx: mpsc::Sender<Track>,
) {
    for track_pub in participant.participant.track_publications().values() {
        if track_pub.source() == track_source.into() {
            let track = track_pub.track();
            match track {
                Some(track) => {
                    let _ = track_tx.send(track).await;
                }
                _ => {}
            }
        }
    }
    let room = &participant.room.room;
    let mut room_event_rx = room.subscribe();
    while let Some(event) = room_event_rx.recv().await {
        match event {
            RoomEvent::TrackPublished { publication, participant: p } => {
                if participant.participant.identity() != p.identity() {
                    continue;
                }
                if publication.source() == track_source.into() {
                    let track = publication.track();
                    match track {
                        Some(track) => {
                            let _ = track_tx.send(track.into()).await;
                        }
                        _ => {}
                    }
                }
            }
            RoomEvent::ParticipantDisconnected(participant) => {
                if participant.identity() == participant.identity() {
                    break;
                }
            }
            RoomEvent::Disconnected { reason: _ } => {
                break;
            }
            _ => {}
        }
    }
}

pub fn ffi_participant_from_handle(
    server: &'static server::FfiServer,
    handle_id: FfiHandleId,
) -> Result<FfiParticipant, FfiError> {
    let ffi_participant_handle = server.retrieve_handle::<FfiParticipant>(handle_id);
    if ffi_participant_handle.is_err() {
        return Err(FfiError::InvalidRequest("participant not found".into()));
    }
    return Ok(ffi_participant_handle.unwrap().clone());
}
