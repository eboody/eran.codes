use async_stream::stream;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Sse},
    Form,
};
use core::convert::Infallible;
use maud::Render;
use tokio::sync::broadcast::error::RecvError;
use tower_cookies::Cookies;

use crate::views::{self, pages};
use serde::Deserialize;
use secrecy::SecretString;

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(
    State(_state): State<crate::State>,
) -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Home))
}

pub async fn login_form() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Login { message: None }))
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    mut auth_session: crate::auth::Session,
    Form(form): Form<LoginForm>,
) -> crate::Result<axum::response::Response> {
    let credentials = app::auth::Credentials {
        email: form.email,
        password: SecretString::new(form.password.into()),
    };

    if let Some(user) = auth_session.authenticate(credentials).await? {
        auth_session.login(&user).await?;
        return Ok(Redirect::to("/").into_response());
    }

    Ok(views::render(pages::Login {
        message: Some("Invalid email or password."),
    })
    .into_response())
}

pub async fn logout(
    mut auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    auth_session.logout().await?;
    Ok(Redirect::to("/").into_response())
}

pub async fn events(
    State(state): State<crate::State>,
    Extension(cookies): Extension<Cookies>,
) -> impl IntoResponse {
    // TODO: Support per-tab SSE streams by mixing a tab id into the session key.
    let session = crate::sse::Handle::from_cookies(&cookies);
    let session_id = session.id().to_string();
    let mut receiver = state.sse.subscribe(&session);

    tracing::info!(session_id = %session_id, "sse connected");

    let stream = stream! {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let mut sse_event = axum::response::sse::Event::default().data(event.data);
                    if let Some(name) = event.name {
                        sse_event = sse_event.event(name);
                    }
                    yield Ok::<_, Infallible>(sse_event);
                }
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => {
                    tracing::info!(session_id = %session_id, "sse disconnected");
                    break;
                }
            }
        }
    };

    Sse::new(stream)
}

pub async fn ping_partial(
    State(_state): State<crate::State>,
) -> impl IntoResponse {
    let elements = views::partials::Ping.render();
    (StatusCode::OK, axum::response::Html(elements.into_string()))
}

pub async fn error_test() -> crate::Result<axum::response::Html<String>> {
    Err(crate::error::Error::Internal)
}
