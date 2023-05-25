use crate::{server, FfiError, FfiHandleId, FfiResult};
use livekit::prelude::*;

pub fn find_remote_track(
    server: &'static server::FfiServer,
    track_sid: &TrackSid,
    participant_sid: &ParticipantSid,
    room_handle: FfiHandleId,
) -> FfiResult<RemoteTrack> {
    let ffi_room = server
        .ffi_handles()
        .get(&room_handle)
        .ok_or(FfiError::InvalidRequest("room not found"))?;

    let ffi_room = ffi_room
        .downcast_ref::<server::room::FfiRoom>()
        .ok_or(FfiError::InvalidRequest("room is not ffi room"))?;

    let room = ffi_room.room();
    let participants = room.participants();
    let participant = participants
        .get(participant_sid)
        .ok_or(FfiError::InvalidRequest("participant not found"))?;

    let track = participant
        .get_track_publication(track_sid)
        .ok_or(FfiError::InvalidRequest("publication not found"))?
        .track()
        .ok_or(FfiError::InvalidRequest("track not found/subscribed"))?;

    Ok(track)
}
