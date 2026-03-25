use domain::chat;

use super::{DemoChatSignals, Signals};
use crate::request;
use crate::types::{Text, UserIdText};

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
pub(super) enum ChatSender {
    #[strum(serialize = "you")]
    You,
    #[strum(serialize = "demo")]
    Demo,
}

#[derive(Clone, Debug)]
pub(super) struct PostInput {
    pub room_id: chat::room::Id,
    pub user_id: chat::UserId,
    pub body: chat::message::Body,
    pub body_text: String,
    pub sender: ChatSender,
    pub author_name: String,
    pub user_id_text: UserIdText,
    pub request_id: Text,
}

impl PostInput {
    pub(super) fn from_authenticated_signals(
        state: &crate::State,
        signals: Signals,
        user: &crate::auth::User,
    ) -> crate::Result<Self> {
        let room_id = room_id_from_binding(state)?;
        let body_text = signals.draft_body.to_string();

        Ok(Self {
            room_id,
            user_id: chat::UserId::from(domain::user::Id::try_from(&user.id)?),
            body: parse_message_body(&body_text)?,
            body_text,
            sender: ChatSender::You,
            author_name: user.username.to_string(),
            user_id_text: UserIdText::new(user.id.to_string()),
            request_id: request_id_from_context(),
        })
    }

    pub(super) async fn from_demo_signals(
        state: &crate::State,
        signals: DemoChatSignals,
    ) -> crate::Result<Self> {
        let demo_user = crate::chat_demo::ensure_demo_user(state).await?;
        let room_id = room_id_from_binding(state)?;
        let body_text = signals.draft_body.to_string();

        Ok(Self {
            room_id,
            user_id: chat::UserId::from(demo_user.id),
            body: parse_message_body(&body_text)?,
            body_text,
            sender: ChatSender::Demo,
            author_name: demo_user.username.to_string(),
            user_id_text: UserIdText::new(demo_user.id.as_ref().to_string()),
            request_id: request_id_from_context(),
        })
    }
}

fn parse_message_body(value: &str) -> crate::Result<domain::chat::message::Body> {
    domain::chat::message::Body::try_new(value)
        .map_err(domain::chat::Error::from)
        .map_err(app::chat::failure::Error::from)
        .map_err(crate::Error::from)
}

fn request_id_from_context() -> Text {
    request::current_context()
        .and_then(|context| context.request_id)
        .map(|request_id| Text::from(request_id.to_string()))
        .unwrap_or_else(|| Text::from(format!("fallback-{}", uuid::Uuid::new_v4())))
}

fn current_handle() -> crate::Result<crate::sse::Handle> {
    let Some(context) = request::current_context() else {
        return Err(crate::Error::Internal);
    };
    let Some(session_id) = context.session_id else {
        return Err(crate::Error::Internal);
    };
    let Some(tab_id) = context.sse_tab_id else {
        return Err(crate::Error::ChatRoomBindingMissing);
    };

    Ok(crate::sse::Handle::with_tab(session_id, Some(tab_id)))
}

fn room_id_from_binding(state: &crate::State) -> crate::Result<domain::chat::room::Id> {
    let handle = current_handle()?;
    state
        .demo
        .chat_room_bindings
        .room_id_for(&handle)
        .ok_or(crate::Error::ChatRoomBindingMissing)
}
