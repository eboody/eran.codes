use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
};
use maud::Render;
use serde::Deserialize;
use tower_sessions::Session;

use crate::types::Text;
use crate::views;

pub async fn auth_status_partial(
    auth_session: crate::auth::Session,
    session: Session,
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.auth", "auth status requested");
    let user = auth_session.user.as_ref();
    let session_id = session.id().map(|id| id.to_string());
    let expiry = session.expiry().map(|expiry| format!("{expiry:?}"));
    let trace = trace_snapshot(&state);
    let user_id = user.map(|value| Text::from(value.id.to_string()));
    let username = user.map(|value| Text::from(value.username.to_string()));
    let email = user.map(|value| Text::from(value.email.to_string()));
    let session_id = session_id.map(Text::from);
    let expiry = expiry.map(Text::from);

    let partial = views::partials::AuthStatus::builder()
        .maybe_user_id(user_id.clone())
        .maybe_username(username.clone())
        .maybe_email(email.clone())
        .maybe_session_id(session_id.clone())
        .maybe_expiry(expiry.clone())
        .trace(trace)
        .build();

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

pub async fn session_status_partial(
    session: Session,
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.session", "session status requested");
    let session_id = session.id().map(|id| id.to_string());
    let expiry = session.expiry().map(|expiry| format!("{expiry:?}"));
    let trace = trace_snapshot(&state);
    let session_id = session_id.map(Text::from);
    let expiry = expiry.map(Text::from);

    let partial = views::partials::SessionStatus::builder()
        .maybe_session_id(session_id)
        .maybe_expiry(expiry)
        .trace(trace)
        .build();

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

pub async fn request_meta_partial(
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.request", "request metadata requested");
    let context = crate::request::current_context();
    let trace = trace_snapshot(&state);
    let request_id = context
        .as_ref()
        .and_then(|value| value.request_id.clone())
        .map(|value| Text::from(value.to_string()));
    let session_id = context
        .as_ref()
        .and_then(|value| value.session_id.clone())
        .map(|value| Text::from(value.to_string()));
    let user_id = context
        .as_ref()
        .and_then(|value| value.user_id.clone())
        .map(|value| Text::from(value.to_string()));
    let client_ip = context
        .as_ref()
        .and_then(|value| value.client_ip.clone())
        .map(|value| Text::from(value.to_string()));
    let user_agent = context
        .as_ref()
        .and_then(|value| value.user_agent.clone())
        .map(|value| Text::from(value.to_string()));
    let partial = views::partials::RequestMeta::builder()
        .maybe_request_id(request_id)
        .maybe_session_id(session_id)
        .maybe_user_id(user_id)
        .maybe_client_ip(client_ip)
        .maybe_user_agent(user_agent)
        .trace(trace)
        .build();

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

#[derive(Deserialize)]
pub struct BoundaryQuery {
    pub case: Option<Text>,
}

pub async fn boundary_check_partial(
    axum::extract::Query(query): axum::extract::Query<BoundaryQuery>,
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.boundary", "boundary check requested");
    let (label, username, email) = match query.case.as_ref().map(|value| value.to_string()).as_deref() {
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

    let partial = views::partials::BoundaryCheck::builder()
        .label(Text::from(label))
        .username(Text::from(username))
        .email(Text::from(email))
        .result(Text::from(result))
        .trace(trace_snapshot(&state))
        .build();

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

fn trace_snapshot(state: &crate::State) -> Vec<crate::trace_log::TraceEntry> {
    let request_id = crate::request::current_context()
        .and_then(|value| value.request_id)
        .unwrap_or_else(crate::types::RequestId::unknown);
    state.trace_log.snapshot_request(&request_id)
}

#[derive(Deserialize)]
pub struct DbCheckQuery {
    pub email: Option<Text>,
}

pub async fn db_check_partial(
    Extension(state): Extension<crate::State>,
    axum::extract::Query(query): axum::extract::Query<DbCheckQuery>,
) -> impl IntoResponse {
    let email_text = query
        .email
        .map(|value| value.to_string())
        .unwrap_or_else(|| "demo@example.com".to_string());
    tracing::info!(target: "demo.db", "db check requested");

    let exists = match domain::user::Email::try_new(&email_text) {
        Ok(email) => state
            .user
            .find_by_email(email)
            .await
            .ok()
            .flatten()
            .is_some(),
        Err(_) => false,
    };
    let trace = trace_snapshot(&state);
    let partial = views::partials::DbCheck::builder()
        .email(Text::from(email_text))
        .exists(exists)
        .trace(trace)
        .build();

    (
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

pub async fn ping_partial(Extension(_state): Extension<crate::State>) -> impl IntoResponse {
    let elements = views::partials::Ping.render();
    (StatusCode::OK, axum::response::Html(elements.into_string()))
}
