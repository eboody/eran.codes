use domain::chat as domain_chat;

use crate::views::partials::chat;

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<chat::Message>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: Option<domain::user::Id>,
) -> Result<ChatContext, crate::error::Error> {
    let incoming = crate::chat_demo_load_flow::IncomingFlow::from_viewer(user_id);
    let viewer = match incoming.maybe_viewer_user_id() {
        Some(user_id) => user_id,
        None => ensure_demo_user(state).await?.id,
    };
    let viewer_resolved = incoming.resolve_viewer(viewer);
    let room = ensure_room(state, viewer_resolved.chat_user_id()).await?;
    let room_ensured = viewer_resolved.attach_room(room);
    let messages = state
        .chat
        .list_messages(
            app::chat::ListMessages::builder()
                .room_id(room_ensured.room().id)
                .user_id(room_ensured.chat_user_id())
                .build(),
        )
        .await?;
    let messages_loaded = room_ensured.attach_messages(messages);
    let message_views = to_message_views(state, messages_loaded.messages()).await;
    let view_mapped = messages_loaded.map_message_views(message_views);

    Ok(view_mapped.into_context())
}

pub async fn ensure_demo_user(
    state: &crate::State,
) -> Result<domain::user::User, crate::error::Error> {
    let identity = crate::chat_demo_demo_user_flow::IncomingFlow::new()
        .prepare_identity(DEMO_USER_EMAIL, DEMO_USER_NAME)?;
    let existing = state.user.find_by_email(identity.email().clone()).await?;
    identity
        .classify_existing(existing)
        .resolve_user(state)
        .await
}

async fn ensure_room(
    state: &crate::State,
    user_id: domain_chat::UserId,
) -> Result<domain::chat::Room, crate::error::Error> {
    let incoming = crate::chat_demo_room_ensure_flow::IncomingFlow::from_user_id(user_id);
    let room = state.chat.find_room_by_name(incoming.room_name()).await?;
    incoming.classify_lookup(room).resolve_room(state).await
}

async fn to_message_views(
    state: &crate::State,
    messages: &[domain::chat::Message],
) -> Vec<chat::Message> {
    let mut names = std::collections::HashMap::new();
    for message in messages {
        let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
        if names.contains_key(&user_id) {
            continue;
        }
        if let Ok(Some(user)) = state.auth.get_user(&user_id).await {
            names.insert(user_id, user.username.to_string());
        }
    }

    messages
        .iter()
        .map(|message| {
            let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
            let author = names
                .get(&user_id)
                .cloned()
                .unwrap_or_else(|| fallback_author_label(&user_id));
            chat::Message::builder()
                .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
                .author(crate::types::Text::from(author))
                .timestamp(crate::types::Text::from(format_message_time(
                    message.created_at,
                )))
                .body(crate::types::Text::from(message.body.to_string()))
                .status(to_chat_message_status(message.status))
                .build()
        })
        .collect()
}

fn fallback_author_label(user_id: &domain::user::Id) -> String {
    format!("user-{}", &user_id.as_uuid().to_string()[..8])
}

fn to_chat_message_status(value: domain::chat::MessageStatus) -> chat::message::Status {
    match value {
        domain::chat::MessageStatus::Visible => chat::message::Status::Visible,
        domain::chat::MessageStatus::Pending => chat::message::Status::Pending,
        domain::chat::MessageStatus::Removed => chat::message::Status::Removed,
    }
}

pub fn format_message_time(value: std::time::SystemTime) -> String {
    let time = time::OffsetDateTime::from(value);
    let format = time::format_description::parse(
        "[year]-[month]-[day] [hour repr:24 padding:zero]:[minute padding:zero]",
    )
    .unwrap_or_else(|_| Vec::new());
    time.format(&format).unwrap_or_else(|_| "--:--".to_string())
}
