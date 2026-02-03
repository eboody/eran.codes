use axum::extract::Extension;

use crate::views::{self, pages};
use crate::views::partials::ChatDemoSection;

pub async fn health(Extension(_state): Extension<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session.user.as_ref().map(|user| {
        crate::views::page::UserNav::builder()
            .username(user.username.clone())
            .email(user.email.clone())
            .build()
    });
    let chat_demo = if let Some(user) = auth_session.user.as_ref() {
        let context = crate::chat_demo::load_chat_context(&state, &user.id).await?;
        Some(
            ChatDemoSection::builder()
                .room_id(context.room.id.as_uuid().to_string())
                .room_name(context.room.name.to_string())
                .messages(context.messages)
                .build(),
        )
    } else {
        None
    };

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
