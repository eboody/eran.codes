use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
};
use maud::Render;
use serde::Deserialize;
use tower_sessions::Session;

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

    let partial = views::partials::AuthStatus::builder()
        .maybe_user_id(user.map(|value| value.id.as_str()))
        .maybe_username(user.map(|value| value.username.as_str()))
        .maybe_email(user.map(|value| value.email.as_str()))
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
    let session_id = session_id.as_deref();
    let expiry = expiry.as_deref();

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
    let request_id = context.as_ref().and_then(|value| value.request_id.as_deref());
    let session_id = context.as_ref().and_then(|value| value.session_id.as_deref());
    let user_id = context.as_ref().and_then(|value| value.user_id.as_deref());
    let client_ip = context.as_ref().and_then(|value| value.client_ip.as_deref());
    let user_agent = context.as_ref().and_then(|value| value.user_agent.as_deref());
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
    pub case: Option<String>,
}

pub async fn boundary_check_partial(
    axum::extract::Query(query): axum::extract::Query<BoundaryQuery>,
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.boundary", "boundary check requested");
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

    let partial = views::partials::BoundaryCheck::builder()
        .label(label)
        .username(username)
        .email(email)
        .result(result)
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
        .unwrap_or_else(|| "unknown".to_string());
    state.trace_log.snapshot_request(&request_id)
}

#[derive(Deserialize)]
pub struct DbCheckQuery {
    pub email: Option<String>,
}

pub async fn db_check_partial(
    Extension(state): Extension<crate::State>,
    axum::extract::Query(query): axum::extract::Query<DbCheckQuery>,
) -> impl IntoResponse {
    let email = query
        .email
        .unwrap_or_else(|| "demo@example.com".to_string());
    tracing::info!(target: "demo.db", "db check requested");

    let exists = state.user.find_by_email(email.clone()).await.ok().flatten().is_some();
    let trace = trace_snapshot(&state);
    let partial = views::partials::DbCheck::builder()
        .email(&email)
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
