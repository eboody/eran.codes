use async_stream::stream;
use axum::{
    Form,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Sse},
};
use core::convert::Infallible;
use datastar::axum::ReadSignals;
use maud::Render;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::views::{self, pages};
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SurrealSignals {
    surreal_message: Option<String>,
    original_surreal_message: Option<String>,
    _surreal_status: Option<String>,
}

fn surreal_payload(
    message: &str,
    status: &str,
) -> crate::sse::Event {
    crate::sse::Event::patch_signals(serde_json::json!({
        "surrealMessage": message,
        "surrealStatus": status,
    }))
}

fn surreal_send(
    state: &crate::State,
    session: &crate::sse::Handle,
    message: &str,
    status: &str,
) -> bool {
    match state.sse.send(session, surreal_payload(message, status)) {
        Ok(()) => true,
        Err(err) => {
            tracing::debug!(?err, "sse session missing for surreal update");
            false
        }
    }
}

fn surreal_original(signals: SurrealSignals) -> String {
    signals
        .original_surreal_message
        .or(signals.surreal_message)
        .unwrap_or_else(|| "Ready.".to_string())
}

pub async fn surreal_message_guarded(
    State(state): State<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl IntoResponse {
    let session = crate::sse::Handle::from_cookies(&cookies);
    let session_id = session.id().to_string();
    let sequence = state
        .surreal_seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let lock = state
        .surreal_guard
        .entry(session_id)
        .or_insert_with(|| std::sync::Arc::new(tokio::sync::Mutex::new(())))
        .clone();

    tokio::spawn(async move {
        let guard = match lock.try_lock() {
            Ok(guard) => {
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded says hi! #{sequence}"),
                    &format!("guarded running #{sequence}"),
                ) {
                    return;
                }
                guard
            }
            Err(_) => {
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded queued #{sequence}"),
                    &format!("guarded queued #{sequence}"),
                ) {
                    return;
                }
                let guard = lock.lock().await;
                if !surreal_send(
                    &state,
                    &session,
                    &format!("Guarded says hi! #{sequence}"),
                    &format!("guarded running #{sequence}"),
                ) {
                    return;
                }
                guard
            }
        };

        sleep(Duration::from_secs(1)).await;
        drop(guard);
        surreal_send(
            &state,
            &session,
            &original,
            &format!("guarded done #{sequence}"),
        );
    });

    StatusCode::ACCEPTED
}

pub async fn surreal_message_cancel(
    State(state): State<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl IntoResponse {
    let session = crate::sse::Handle::from_cookies(&cookies);
    let session_id = session.id().to_string();
    let sequence = state
        .surreal_seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let token = tokio_util::sync::CancellationToken::new();
    if let Some(previous) = state.surreal_cancel.insert(session_id, token.clone()) {
        previous.cancel();
    }

    tokio::spawn(async move {
        if !surreal_send(
            &state,
            &session,
            &format!("Cancelled says hi! #{sequence}"),
            &format!("cancel running #{sequence}"),
        ) {
            return;
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                surreal_send(
                    &state,
                    &session,
                    &original,
                    &format!("cancel done #{sequence}"),
                );
            }
            _ = token.cancelled() => {
                surreal_send(
                    &state,
                    &session,
                    &format!("Cancelled #{sequence}"),
                    &format!("cancelled #{sequence}"),
                );
            }
        }
    });

    StatusCode::ACCEPTED
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
                    let sse_event = event.as_datastar_event().write_as_axum_sse_event();
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

pub async fn ping_partial(State(_state): State<crate::State>) -> impl IntoResponse {
    let elements = views::partials::Ping.render();
    (StatusCode::OK, axum::response::Html(elements.into_string()))
}

pub async fn error_test() -> crate::Result<axum::response::Html<String>> {
    Err(crate::error::Error::Internal)
}
