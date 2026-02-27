use axum::extract::Extension;

use crate::types::Text;
use crate::views::partials::chat;
use crate::views::{self, pages};

pub async fn health(Extension(_state): Extension<crate::State>) -> &'static str {
    "OK"
}

pub async fn home(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let is_authenticated = auth_session.user.is_some();
    let user = auth_session.user.as_ref().map(|user| {
        crate::views::page::UserNav::builder()
            .username(Text::from(user.username.to_string()))
            .email(Text::from(user.email.to_string()))
            .build()
    });
    let viewer_id = auth_session
        .user
        .as_ref()
        .map(|user| user.id.to_domain())
        .transpose()?;
    let context = crate::chat_demo::load_chat_context(&state, viewer_id).await?;
    let chat_demo = Some(
        chat::DemoSection::builder()
            .room_id(crate::types::Text::from(
                context.room.id.as_uuid().to_string(),
            ))
            .room_name(crate::types::Text::from(context.room.name.to_string()))
            .messages(context.messages)
            .interactivity(chat::Mode::from(is_authenticated))
            .build(),
    );

    Ok(views::render(
        pages::Home::builder()
            .maybe_user(user)
            .maybe_chat_demo(chat_demo)
            .build(),
    ))
}

pub async fn error_test() -> crate::Result<axum::response::Html<String>> {
    Err(crate::error::Error::Internal)
}
