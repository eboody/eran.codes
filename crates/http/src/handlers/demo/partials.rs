use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
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
    let prepared = super::partials_auth_status_flow::IncomingFlow::new().prepare_snapshot(
        &auth_session,
        &session,
        &state,
    );
    prepared.into_response()
}

pub async fn session_status_partial(
    session: Session,
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.session", "session status requested");
    let prepared = super::partials_session_status_flow::IncomingFlow::new()
        .prepare_snapshot(&session, &state);
    prepared.into_response()
}

pub async fn request_meta_partial(
    Extension(state): Extension<crate::State>,
) -> impl IntoResponse {
    tracing::info!(target: "demo.request", "request metadata requested");
    let prepared =
        super::partials_request_meta_flow::IncomingFlow::new().prepare_snapshot(&state);
    prepared.into_response()
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
    let incoming =
        super::partials_boundary_check_flow::IncomingFlow::from_query(query.case);
    let case_resolved = incoming.resolve_case();
    let evaluated = case_resolved.evaluate(&state);
    evaluated.into_response()
}

pub(super) fn trace_snapshot(state: &crate::State) -> Vec<crate::trace_log::TraceEntry> {
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
    let incoming = super::partials_db_check_flow::IncomingFlow::from_query(query.email);
    let prepared = incoming.prepare_email();
    let evaluated = prepared.evaluate_lookup(&state).await;
    evaluated.into_response(&state)
}

pub async fn ping_partial(Extension(_state): Extension<crate::State>) -> impl IntoResponse {
    let elements = views::partials::Ping.render();
    (StatusCode::OK, axum::response::Html(elements.into_string()))
}

pub async fn request_burst_probe() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
