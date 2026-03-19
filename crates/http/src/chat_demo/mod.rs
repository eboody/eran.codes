// ci: descriptive-module-import crate::chat_demo
mod context_flow;
mod demo_user_flow;
mod room_bindings;

use crate::views::partials;

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

pub mod room {
    pub use super::room_bindings::{Match, RoomBindings as Bindings};
}

pub struct ChatContext {
    pub room: domain::chat::Room,
    pub messages: Vec<partials::components::chat::Message>,
}

pub async fn load_chat_context(
    state: &crate::State,
    user_id: Option<domain::user::Id>,
) -> Result<ChatContext, crate::Error> {
    context_flow::IncomingFlow::from_viewer(user_id)
        .load(state)
        .await
}

pub async fn ensure_demo_user(
    state: &crate::State,
) -> Result<domain::user::User, crate::Error> {
    demo_user_flow::IncomingFlow::new()
        .ensure(state, DEMO_USER_EMAIL, DEMO_USER_NAME)
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
