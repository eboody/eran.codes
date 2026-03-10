use statum::{machine, state, transition};

use super::{Error, ListMessages};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct MessagesData {
    messages: Vec<chat::Message>,
}

#[state]
pub enum ListMessagesState {
    Incoming,
    RoomVerified,
    MembershipVerified,
    Loaded(MessagesData),
}

#[machine]
pub(super) struct ListMessagesFlow<ListMessagesState> {
    room_id: chat::RoomId,
    user_id: chat::UserId,
    limit: usize,
}

impl ListMessagesFlow<Incoming> {
    pub(super) fn from_command(command: ListMessages) -> Self {
        let ListMessages {
            room_id,
            user_id,
            limit,
        } = command;
        ListMessagesFlow::<Incoming>::builder()
            .room_id(room_id)
            .user_id(user_id)
            .limit(limit)
            .build()
    }
}

impl<S: ListMessagesStateTrait> ListMessagesFlow<S> {
    pub(super) fn room_id(&self) -> &chat::RoomId {
        &self.room_id
    }

    pub(super) fn user_id(&self) -> &chat::UserId {
        &self.user_id
    }

    pub(super) fn limit(&self) -> usize {
        self.limit
    }
}

#[transition]
impl ListMessagesFlow<Incoming> {
    fn mark_room_verified(self) -> ListMessagesFlow<RoomVerified> {
        self.transition()
    }
}

impl ListMessagesFlow<Incoming> {
    pub(super) fn classify_room_lookup(self, room_exists: bool) -> RoomLookupOutcome {
        if room_exists {
            RoomLookupOutcome::Found(self.mark_room_verified())
        } else {
            RoomLookupOutcome::Missing
        }
    }
}

#[transition]
impl ListMessagesFlow<RoomVerified> {
    fn mark_membership_verified(self) -> ListMessagesFlow<MembershipVerified> {
        self.transition()
    }
}

impl ListMessagesFlow<RoomVerified> {
    pub(super) fn classify_membership(self, is_member: bool) -> MembershipOutcome {
        if is_member {
            MembershipOutcome::Member(self.mark_membership_verified())
        } else {
            MembershipOutcome::NotMember
        }
    }
}

#[transition]
impl ListMessagesFlow<MembershipVerified> {
    pub(super) fn attach_messages(
        self,
        messages: Vec<chat::Message>,
    ) -> ListMessagesFlow<Loaded> {
        self.transition_with(MessagesData { messages })
    }
}

impl ListMessagesFlow<Loaded> {
    pub(super) fn into_messages(self) -> Vec<chat::Message> {
        self.state_data.messages
    }
}

pub(super) enum RoomLookupOutcome {
    Found(ListMessagesFlow<RoomVerified>),
    Missing,
}

impl RoomLookupOutcome {
    pub(super) fn require_room(self) -> Result<ListMessagesFlow<RoomVerified>, Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(Error::RoomNotFound),
        }
    }
}

pub(super) enum MembershipOutcome {
    Member(ListMessagesFlow<MembershipVerified>),
    NotMember,
}

impl MembershipOutcome {
    pub(super) fn require_member(
        self,
    ) -> Result<ListMessagesFlow<MembershipVerified>, Error> {
        match self {
            Self::Member(member) => Ok(member),
            Self::NotMember => Err(Error::NotMember),
        }
    }
}

pub(super) type IncomingFlow = ListMessagesFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn command() -> ListMessages {
        ListMessages::builder()
            .room_id(chat::RoomId::new_v4())
            .user_id(chat::UserId::new_v4())
            .limit(10)
            .build()
    }

    fn message(room_id: chat::RoomId, user_id: chat::UserId) -> chat::Message {
        chat::Message::builder()
            .id(chat::MessageId::new_v4())
            .room_id(room_id)
            .user_id(user_id)
            .body(chat::MessageBody::try_new("hello").expect("valid body"))
            .status(chat::MessageStatus::Visible)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    #[test]
    fn classify_room_lookup_rejects_missing_room() {
        let incoming = ListMessagesFlow::<Incoming>::from_command(command());
        let result = incoming.classify_room_lookup(false).require_room();
        assert!(matches!(result, Err(Error::RoomNotFound)));
    }

    #[test]
    fn classify_membership_rejects_non_member() {
        let incoming = ListMessagesFlow::<Incoming>::from_command(command());
        let room_verified = incoming
            .classify_room_lookup(true)
            .require_room()
            .expect("room verified");
        let result = room_verified.classify_membership(false).require_member();
        assert!(matches!(result, Err(Error::NotMember)));
    }

    #[test]
    fn loaded_state_returns_messages() {
        let incoming = ListMessagesFlow::<Incoming>::from_command(command());
        let room_id = *incoming.room_id();
        let user_id = *incoming.user_id();
        let verified = incoming
            .classify_room_lookup(true)
            .require_room()
            .expect("room verified")
            .classify_membership(true)
            .require_member()
            .expect("member verified");
        let loaded = verified.attach_messages(vec![message(room_id, user_id)]);

        assert_eq!(loaded.into_messages().len(), 1);
    }
}
