// ci: descriptive-module-import crate::chat_demo
mod context_flow;
mod demo_user_flow;

use crate::views::partials::chat;

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<chat::Message>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: Option<domain::user::UserId>,
) -> Result<ChatContext, crate::error::Error> {
    let incoming = context_flow::IncomingFlow::from_viewer(user_id);
    let viewer_resolved = incoming.resolve_viewer(state).await?;
    let room_ready = viewer_resolved.ensure_room(state).await?;
    let messages_loaded = room_ready.load_messages(state).await?;
    let context_built = messages_loaded.build_context(state).await;

    Ok(context_built.into_context())
}

pub async fn ensure_demo_user(
    state: &crate::State,
) -> Result<domain::user::User, crate::error::Error> {
    let identity = demo_user_flow::IncomingFlow::new()
        .prepare_identity(DEMO_USER_EMAIL, DEMO_USER_NAME)?;
    let existing = state.user.find_by_email(identity.email().clone()).await?;
    identity
        .classify_existing(existing)
        .resolve_user(state)
        .await
}

pub fn format_message_time(value: std::time::SystemTime) -> String {
    let time = time::OffsetDateTime::from(value);
    let format = time::format_description::parse(
        "[year]-[month]-[day] [hour repr:24 padding:zero]:[minute padding:zero]",
    )
    .unwrap_or_else(|_| Vec::new());
    time.format(&format).unwrap_or_else(|_| "--:--".to_string())
}
