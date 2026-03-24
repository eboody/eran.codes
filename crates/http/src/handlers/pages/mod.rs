// ci: descriptive-module-import crate::handlers::pages
mod counter_sync_flow;
mod lab_flow;
mod operations_filter_flow;

use axum::extract::Extension;
use axum::response::{IntoResponse, Redirect};
use datastar::axum::ReadSignals;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::types::{SseTabId, Text};
use crate::views::partials::components::portfolio::content::WorkCaseSlug;
use crate::views::{self, pages};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn home() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Home))
}

pub async fn work() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::Work))
}

pub async fn open_source() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(pages::OpenSource))
}

pub async fn resume_text() -> impl axum::response::IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
        crate::views::partials::components::portfolio::content::resume_text().to_owned(),
    )
}

pub async fn work_chat_realtime() -> impl IntoResponse {
    redirect_to_work_archive(WorkCaseSlug::ChatRealtime)
}

pub async fn work_command_sse() -> impl IntoResponse {
    redirect_to_work_archive(WorkCaseSlug::CommandSse)
}

pub async fn work_operational_visibility() -> impl IntoResponse {
    redirect_to_work_archive(WorkCaseSlug::OperationalVisibility)
}

pub async fn work_sensitive_sync() -> crate::Result<axum::response::Html<String>> {
    Ok(views::render(
        pages::WorkCase::builder()
            .slug(WorkCaseSlug::SensitiveSync)
            .build(),
    ))
}

fn redirect_to_work_archive(slug: WorkCaseSlug) -> Redirect {
    Redirect::permanent(slug.public_href())
}

pub async fn lab(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let page_ready = lab_flow::IncomingFlow::from_auth_user(auth_session.user.clone())
        .resolve_viewer()?
        .load_chat_context(&state)
        .await?
        .bind_live_tab(&cookies, &state.cookie_key, &state.demo.chat_room_bindings);

    Ok(views::render(page_ready.into_page()))
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
    session_bound
        .verify_stream(&state.sse)
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
