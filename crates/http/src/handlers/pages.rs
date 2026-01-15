use axum::extract::Extension;

use crate::views::{self, pages};

pub async fn health(Extension(_state): Extension<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(
    Extension(_state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session.user.as_ref().map(|user| {
        crate::views::page::UserNav::builder()
            .username(user.username.clone())
            .email(user.email.clone())
            .build()
    });

    Ok(views::render(
        pages::Home::builder().maybe_user(user).build(),
    ))
}

pub async fn error_test() -> crate::Result<axum::response::Html<String>> {
    Err(crate::error::Error::Internal)
}
