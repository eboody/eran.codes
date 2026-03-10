use statum::{machine, state, transition};

use domain::chat as domain_chat;

#[derive(Clone, Debug)]
pub struct RoomData {
    room: domain::chat::Room,
}

#[state]
pub enum RoomEnsureState {
    Incoming,
    ExistingRoom(RoomData),
    MissingRoom,
    Resolved(RoomData),
}

#[machine]
pub(super) struct RoomEnsureFlow<RoomEnsureState> {
    room_name: domain_chat::RoomName,
    user_id: domain_chat::UserId,
}

impl RoomEnsureFlow<Incoming> {
    pub(super) fn from_user_id(user_id: domain_chat::UserId) -> Self {
        RoomEnsureFlow::<Incoming>::builder()
            .room_name(domain_chat::RoomName::Lobby)
            .user_id(user_id)
            .build()
    }
}

#[transition]
impl RoomEnsureFlow<Incoming> {
    fn mark_existing(self, room: domain::chat::Room) -> RoomEnsureFlow<ExistingRoom> {
        self.transition_with(RoomData { room })
    }

    fn mark_missing(self) -> RoomEnsureFlow<MissingRoom> {
        self.transition()
    }
}

impl RoomEnsureFlow<Incoming> {
    pub(super) fn classify_lookup(
        self,
        room: Option<domain::chat::Room>,
    ) -> RoomLookupOutcome {
        match room {
            Some(room) => RoomLookupOutcome::Existing(self.mark_existing(room)),
            None => RoomLookupOutcome::Missing(self.mark_missing()),
        }
    }
}

#[transition]
impl RoomEnsureFlow<ExistingRoom> {
    pub(super) fn mark_membership_joined(self) -> RoomEnsureFlow<Resolved> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

#[transition]
impl RoomEnsureFlow<MissingRoom> {
    pub(super) fn mark_created(self, room: domain::chat::Room) -> RoomEnsureFlow<Resolved> {
        self.transition_with(RoomData { room })
    }
}

impl<S: RoomEnsureStateTrait> RoomEnsureFlow<S> {
    pub(super) fn user_id(&self) -> domain_chat::UserId {
        self.user_id
    }

    pub(super) fn room_name(&self) -> domain_chat::RoomName {
        self.room_name
    }
}

impl RoomEnsureFlow<ExistingRoom> {
    pub(super) fn room(&self) -> &domain::chat::Room {
        &self.state_data.room
    }

    async fn join_membership(
        self,
        state: &crate::State,
    ) -> crate::Result<RoomEnsureFlow<Resolved>> {
        state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(self.room().id)
                    .user_id(self.user_id())
                    .build(),
            )
            .await?;
        Ok(self.mark_membership_joined())
    }
}

impl RoomEnsureFlow<MissingRoom> {
    async fn create_room(
        self,
        state: &crate::State,
    ) -> crate::Result<RoomEnsureFlow<Resolved>> {
        let room = state
            .chat
            .create_room(
                app::chat::CreateRoom::builder()
                    .name(self.room_name())
                    .created_by(self.user_id())
                    .build(),
            )
            .await?;
        Ok(self.mark_created(room))
    }
}

impl RoomEnsureFlow<Resolved> {
    pub(super) fn into_room(self) -> domain::chat::Room {
        self.state_data.room
    }
}

pub(super) enum RoomLookupOutcome {
    Existing(RoomEnsureFlow<ExistingRoom>),
    Missing(RoomEnsureFlow<MissingRoom>),
}

impl RoomLookupOutcome {
    pub(super) async fn resolve_room(
        self,
        state: &crate::State,
    ) -> crate::Result<domain::chat::Room> {
        match self {
            Self::Existing(existing) => {
                Ok(existing.join_membership(state).await?.into_room())
            }
            Self::Missing(missing) => Ok(missing.create_room(state).await?.into_room()),
        }
    }
}

pub(super) type IncomingFlow = RoomEnsureFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_lookup_missing_branch() {
        let incoming =
            RoomEnsureFlow::<Incoming>::from_user_id(domain_chat::UserId::new_v4());
        let outcome = incoming.classify_lookup(None);
        assert!(matches!(outcome, RoomLookupOutcome::Missing(_)));
    }
}
