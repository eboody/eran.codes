use axum::extract::Extension;

use crate::views::{self, pages};
use crate::types::Text;
use crate::views::partials::ChatDemoSection;

pub async fn health(Extension(_state): Extension<crate::State>) -> &'static str {
    "OK"
}

pub async fn home(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session.user.as_ref().map(|user| {
        crate::views::page::UserNav::builder()
            .username(Text::from(user.username.to_string()))
            .email(Text::from(user.email.to_string()))
            .build()
    });
    let chat_demo = if let Some(user) = auth_session.user.as_ref() {
        let context =
            crate::chat_demo::load_chat_context(&state, user.id.to_domain()?).await?;
        Some(
            ChatDemoSection::builder()
                .room_id(crate::types::Text::from(
                    context.room.id.as_uuid().to_string(),
                ))
                .room_name(crate::types::Text::from(
                    context.room.name.to_string(),
                ))
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
