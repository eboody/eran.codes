// ci: descriptive-module-import crate::handlers::pages
mod counter_sync_flow;
mod lab_flow;
mod operations_filter_flow;

use axum::extract::Extension;
use datastar::axum::ReadSignals;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::types::{SseTabId, Text};
use crate::views::partials::components::portfolio::content::WorkCaseSlug;
use crate::views::{self, pages};

pub async fn health(Extension(_state): Extension<crate::State>) -> &'static str {
    "OK"
}

pub async fn home() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Home))
}

pub async fn work() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Work))
}

pub async fn work_chat_realtime() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(
        pages::WorkCase::builder()
            .slug(WorkCaseSlug::ChatRealtime)
            .build(),
    ))
}

pub async fn work_command_sse() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(
        pages::WorkCase::builder()
            .slug(WorkCaseSlug::CommandSse)
            .build(),
    ))
}

pub async fn work_operational_visibility() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(
        pages::WorkCase::builder()
            .slug(WorkCaseSlug::OperationalVisibility)
            .build(),
    ))
}

pub async fn lab(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let incoming = lab_flow::IncomingFlow::from_auth_user(auth_session.user.clone());
    let viewer_resolved = incoming.resolve_viewer()?;
    let chat_loaded = viewer_resolved.load_chat_context(&state).await?;

    Ok(views::render(chat_loaded.into_page()))
}

pub async fn error_test() -> crate::Result<axum::response::Html<String>> {
    Err(crate::error::Error::Internal)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterRequestSignals {
    pub delta: Option<i64>,
    pub sse_tab_id: Option<SseTabId>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperationsFilterSignals {
    pub operations_filter_query: Option<Text>,
    pub sse_tab_id: Option<SseTabId>,
}

// ci: datastar-command counter_sync
pub async fn counter_sync(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<CounterRequestSignals>,
) -> axum::http::StatusCode {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        crate::request::set_sse_tab_id(tab_id);
    }
    let incoming = counter_sync_flow::IncomingFlow::new(
        signals.delta.unwrap_or_default(),
        signals.sse_tab_id.clone(),
    );
    let session_bound = incoming.bind_session(&cookies, &state.cookie_key);
    let has_streams = state
        .sse
        .has_streams_for_session(&session_bound.session_id());
    session_bound
        .verify_streams(has_streams)
        .dispatch(state.demo.counter.server_count.as_ref(), &state.sse)
        .status_code()
}

// ci: datastar-command operations_filter_update
pub async fn operations_filter_update(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<OperationsFilterSignals>,
) -> axum::http::StatusCode {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        crate::request::set_sse_tab_id(tab_id);
    }
    let incoming = operations_filter_flow::IncomingFlow::new(
        signals.operations_filter_query,
        signals.sse_tab_id.clone(),
    );
    let session_bound = incoming.bind_session(&cookies, &state.cookie_key);
    let filter_applied = session_bound.apply_filter(&state.trace_log);
    let refreshed = filter_applied.refresh_panels(&state.trace_log);
    refreshed.status_code()
}
