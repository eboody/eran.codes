use statum::{machine, state, transition};

use crate::views::partials::chat;
use domain::chat as domain_chat;

#[derive(Clone, Debug)]
pub struct ViewerResolvedData {
    chat_user_id: domain_chat::UserId,
}

#[derive(Clone, Debug)]
pub struct RoomEnsuredData {
    room: domain::chat::Room,
    chat_user_id: domain_chat::UserId,
}

#[derive(Clone, Debug)]
pub struct MessagesLoadedData {
    room: domain::chat::Room,
    messages: Vec<domain::chat::Message>,
}

#[derive(Clone, Debug)]
pub struct ViewMappedData {
    room: domain::chat::Room,
    message_views: Vec<chat::Message>,
}

#[state]
pub enum ChatContextLoadState {
    Incoming,
    ViewerResolved(ViewerResolvedData),
    RoomEnsured(RoomEnsuredData),
    MessagesLoaded(MessagesLoadedData),
    ViewMapped(ViewMappedData),
}

#[machine]
pub(super) struct ChatContextLoadFlow<ChatContextLoadState> {
    viewer_user_id: Option<domain::user::Id>,
}

impl ChatContextLoadFlow<Incoming> {
    pub(super) fn from_viewer(maybe_viewer_user_id: Option<domain::user::Id>) -> Self {
        ChatContextLoadFlow::<Incoming>::builder()
            .maybe_viewer_user_id(maybe_viewer_user_id)
            .build()
    }

    pub(super) fn maybe_viewer_user_id(&self) -> Option<domain::user::Id> {
        self.viewer_user_id
    }
}

#[transition]
impl ChatContextLoadFlow<Incoming> {
    pub(super) fn resolve_viewer(
        self,
        viewer_user_id: domain::user::Id,
    ) -> ChatContextLoadFlow<ViewerResolved> {
        let data = ViewerResolvedData {
            chat_user_id: domain_chat::UserId::from_uuid(*viewer_user_id.as_uuid()),
        };
        self.transition_with(data)
    }
}

#[transition]
impl ChatContextLoadFlow<ViewerResolved> {
    pub(super) fn attach_room(
        self,
        room: domain::chat::Room,
    ) -> ChatContextLoadFlow<RoomEnsured> {
        let data = RoomEnsuredData {
            room,
            chat_user_id: self.state_data.chat_user_id,
        };
        self.transition_with(data)
    }
}

#[transition]
impl ChatContextLoadFlow<RoomEnsured> {
    pub(super) fn attach_messages(
        self,
        messages: Vec<domain::chat::Message>,
    ) -> ChatContextLoadFlow<MessagesLoaded> {
        let data = MessagesLoadedData {
            room: self.state_data.room.clone(),
            messages,
        };
        self.transition_with(data)
    }
}

#[transition]
impl ChatContextLoadFlow<MessagesLoaded> {
    pub(super) fn map_message_views(
        self,
        message_views: Vec<chat::Message>,
    ) -> ChatContextLoadFlow<ViewMapped> {
        let data = ViewMappedData {
            room: self.state_data.room.clone(),
            message_views,
        };
        self.transition_with(data)
    }
}

impl ChatContextLoadFlow<ViewerResolved> {
    pub(super) fn chat_user_id(&self) -> domain_chat::UserId {
        self.state_data.chat_user_id
    }
}

impl ChatContextLoadFlow<RoomEnsured> {
    pub(super) fn room(&self) -> &domain::chat::Room {
        &self.state_data.room
    }

    pub(super) fn chat_user_id(&self) -> domain_chat::UserId {
        self.state_data.chat_user_id
    }
}

impl ChatContextLoadFlow<MessagesLoaded> {
    pub(super) fn messages(&self) -> &[domain::chat::Message] {
        &self.state_data.messages
    }
}

impl ChatContextLoadFlow<ViewMapped> {
    pub(super) fn into_context(self) -> super::chat_demo::ChatContext {
        super::chat_demo::ChatContext {
            room: self.state_data.room,
            messages: self.state_data.message_views,
        }
    }
}

pub(super) type IncomingFlow = ChatContextLoadFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_room() -> domain::chat::Room {
        domain::chat::Room::builder()
            .id(domain::chat::RoomId::new_v4())
            .name(domain::chat::RoomName::Lobby)
            .created_by(domain::chat::UserId::new_v4())
            .build()
    }

    fn sample_message() -> domain::chat::Message {
        domain::chat::Message::builder()
            .id(domain::chat::MessageId::new_v4())
            .room_id(domain::chat::RoomId::new_v4())
            .user_id(domain::chat::UserId::new_v4())
            .body(domain::chat::MessageBody::try_new("hello").expect("valid body"))
            .status(domain::chat::MessageStatus::Visible)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    #[test]
    fn resolve_viewer_maps_to_chat_user_id() {
        let viewer = domain::user::Id::new_v4();
        let incoming = ChatContextLoadFlow::<Incoming>::from_viewer(Some(viewer));
        let resolved = incoming.resolve_viewer(viewer);

        assert_eq!(resolved.chat_user_id().as_uuid(), viewer.as_uuid());
    }

    #[test]
    fn map_message_views_builds_chat_context() {
        let viewer = domain::user::Id::new_v4();
        let room = sample_room();
        let message = sample_message();
        let incoming = ChatContextLoadFlow::<Incoming>::from_viewer(Some(viewer));
        let resolved = incoming.resolve_viewer(viewer);
        let room_ensured = resolved.attach_room(room.clone());
        let loaded = room_ensured.attach_messages(vec![message]);
        let mapped = loaded.map_message_views(vec![]);
        let context = mapped.into_context();

        assert_eq!(context.room, room);
        assert_eq!(context.messages.len(), 0);
    }
}
