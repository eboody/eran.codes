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
use tower_sessions::Session;

use crate::views::{self, pages};
use secrecy::SecretString;

pub async fn health(State(_state): State<crate::State>) -> &'static str {
    "ok"
}

pub async fn home(
    State(_state): State<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Home {
        user: auth_session.user.as_ref().map(|user| {
            crate::views::page::UserNav {
                username: user.username.clone(),
                email: user.email.clone(),
            }
        }),
    }))
}

pub async fn login_form(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    if auth_session.user.is_some() {
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Login {
        message: None,
        user: None,
    })
    .into_response())
}

pub async fn register_form(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    if auth_session.user.is_some() {
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Register {
        message: None,
        user: None,
    })
    .into_response())
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
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
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Login {
        message: Some("Invalid email or password."),
        user: None,
    })
    .into_response())
}

pub async fn register(
    State(state): State<crate::State>,
    mut auth_session: crate::auth::Session,
    Form(form): Form<RegisterForm>,
) -> crate::Result<axum::response::Response> {
    let credentials = app::auth::Credentials {
        email: form.email.clone(),
        password: SecretString::new(form.password.clone().into()),
    };

    let command = app::user::RegisterUser {
        username: form.username,
        email: form.email,
        password: SecretString::new(form.password.into()),
    };

    match state.user.register_user(command).await {
        Ok(_) => {
            if let Some(user) =
                auth_session.authenticate(credentials).await?
            {
                auth_session.login(&user).await?;
                Ok(Redirect::to("/protected").into_response())
            } else {
                Err(crate::Error::Internal)
            }
        }
        Err(app::user::Error::EmailTaken) => Ok(views::render(
            pages::Register {
                message: Some("Email already in use."),
                user: None,
            },
        )
        .into_response()),
        Err(app::user::Error::Domain(_)) => Ok(views::render(
            pages::Register {
                message: Some("Invalid input."),
                user: None,
            },
        )
        .into_response()),
        Err(error) => Err(error.into()),
    }
}

pub async fn logout(
    mut auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    auth_session.logout().await?;
    Ok(Redirect::to("/").into_response())
}

pub async fn protected(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    let Some(user) = auth_session.user else {
        return Ok(Redirect::to("/login").into_response());
    };

    Ok(views::render(pages::Protected {
        username: user.username.clone(),
        email: user.email.clone(),
        user: Some(crate::views::page::UserNav {
            username: user.username,
            email: user.email,
        }),
    })
    .into_response())
}

pub async fn auth_status_partial(
    auth_session: crate::auth::Session,
    session: Session,
) -> impl IntoResponse {
    let user = auth_session.user.as_ref();
    let session_id = session.id().map(|id| id.to_string());
    let expiry = session.expiry().map(|expiry| format!("{expiry:?}"));

    let partial = views::partials::AuthStatus {
        user_id: user.map(|value| value.id.as_str()),
        username: user.map(|value| value.username.as_str()),
        email: user.map(|value| value.email.as_str()),
        session_id,
        expiry,
    };

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

pub async fn session_status_partial(
    session: Session,
) -> impl IntoResponse {
    let session_id = session.id().map(|id| id.to_string());
    let expiry = session.expiry().map(|expiry| format!("{expiry:?}"));

    let partial = views::partials::SessionStatus {
        session_id: session_id.as_deref(),
        expiry: expiry.as_deref(),
    };

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

pub async fn request_meta_partial() -> impl IntoResponse {
    let context = crate::request::current_context();
    let partial = views::partials::RequestMeta {
        request_id: context.as_ref().and_then(|value| value.request_id.as_deref()),
        session_id: context.as_ref().and_then(|value| value.session_id.as_deref()),
        user_id: context.as_ref().and_then(|value| value.user_id.as_deref()),
        client_ip: context.as_ref().and_then(|value| value.client_ip.as_deref()),
        user_agent: context.as_ref().and_then(|value| value.user_agent.as_deref()),
    };

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

#[derive(Deserialize)]
pub struct BoundaryQuery {
    pub case: Option<String>,
}

pub async fn boundary_check_partial(
    axum::extract::Query(query): axum::extract::Query<BoundaryQuery>,
) -> impl IntoResponse {
    let (label, username, email) = match query.case.as_deref() {
        Some("invalid") => ("Invalid input", " ", "not-an-email"),
        _ => ("Valid input", "demo_user", "demo@example.com"),
    };

    let result = match app::user::validate_input(username, email) {
        Ok(_) => "ok",
        Err(err) => {
            tracing::debug!(?err, "boundary validation failed");
            "error"
        }
    };

    let partial = views::partials::BoundaryCheck {
        label,
        username,
        email,
        result,
    };

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
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
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
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
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
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
    let session =
        crate::sse::Handle::from_cookies(&cookies, &state.cookie_key);
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
