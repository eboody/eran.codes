use statum::{machine, state, transition};

use super::{Error, JoinRoom, RoomRole};
use domain::chat;

#[state]
pub enum JoinRoomState {
    Incoming,
    RoomVerified,
    MembershipAdded,
    Audited,
}

#[machine]
pub(super) struct JoinRoomFlow<JoinRoomState> {
    room_id: chat::RoomId,
    user_id: chat::UserId,
    role: RoomRole,
}

impl JoinRoomFlow<Incoming> {
    pub(super) fn from_command(command: JoinRoom) -> Self {
        let JoinRoom {
            room_id,
            user_id,
            role,
        } = command;
        JoinRoomFlow::<Incoming>::builder()
            .room_id(room_id)
            .user_id(user_id)
            .role(role)
            .build()
    }
}

impl<S: JoinRoomStateTrait> JoinRoomFlow<S> {
    pub(super) fn room_id(&self) -> chat::RoomId {
        self.room_id
    }

    pub(super) fn user_id(&self) -> chat::UserId {
        self.user_id
    }

    pub(super) fn role(&self) -> RoomRole {
        self.role
    }
}

#[transition]
impl JoinRoomFlow<Incoming> {
    fn mark_room_verified(self) -> JoinRoomFlow<RoomVerified> {
        self.transition()
    }
}

impl JoinRoomFlow<Incoming> {
    pub(super) fn classify_room_lookup(self, room_exists: bool) -> RoomLookupOutcome {
        if room_exists {
            RoomLookupOutcome::Found(self.mark_room_verified())
        } else {
            RoomLookupOutcome::Missing
        }
    }
}

#[transition]
impl JoinRoomFlow<RoomVerified> {
    pub(super) fn mark_membership_added(self) -> JoinRoomFlow<MembershipAdded> {
        self.transition()
    }
}

#[transition]
impl JoinRoomFlow<MembershipAdded> {
    pub(super) fn mark_audited(self) -> JoinRoomFlow<Audited> {
        self.transition()
    }
}

pub(super) enum RoomLookupOutcome {
    Found(JoinRoomFlow<RoomVerified>),
    Missing,
}

impl RoomLookupOutcome {
    pub(super) fn require_room(self) -> Result<JoinRoomFlow<RoomVerified>, Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(Error::RoomNotFound),
        }
    }
}

pub(super) type IncomingFlow = JoinRoomFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn command() -> JoinRoom {
        JoinRoom::builder()
            .room_id(chat::RoomId::new_v4())
            .user_id(chat::UserId::new_v4())
            .role(RoomRole::Member)
            .build()
    }

    #[test]
    fn classify_room_lookup_rejects_missing_room() {
        let incoming = JoinRoomFlow::<Incoming>::from_command(command());
        let result = incoming.classify_room_lookup(false).require_room();
        assert!(matches!(result, Err(Error::RoomNotFound)));
    }

    #[test]
    fn happy_path_reaches_audited_state() {
        let incoming = JoinRoomFlow::<Incoming>::from_command(command());
        let room_id = incoming.room_id();
        let user_id = incoming.user_id();
        let role = incoming.role();

        let verified = incoming
            .classify_room_lookup(true)
            .require_room()
            .expect("room exists");
        let membership_added = verified.mark_membership_added();
        let audited = membership_added.mark_audited();

        assert_eq!(audited.room_id(), room_id);
        assert_eq!(audited.user_id(), user_id);
        assert_eq!(audited.role(), role);
    }
}
