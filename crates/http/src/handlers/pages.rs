use axum::extract::Extension;
use datastar::axum::ReadSignals;
use serde::Deserialize;
use std::sync::atomic::Ordering;
use tower_cookies::Cookies;

use crate::types::{SseTabId, Text};
use crate::views::partials::chat;
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
    let is_authenticated = auth_session.user.is_some();
    let user = auth_session.user.as_ref().map(|user| {
        crate::views::page::UserNav::builder()
            .username(Text::from(user.username.to_string()))
            .email(Text::from(user.email.to_string()))
            .build()
    });
    let viewer_id = auth_session
        .user
        .as_ref()
        .map(|user| user.id.to_domain())
        .transpose()?;
    let context = crate::chat_demo::load_chat_context(&state, viewer_id).await?;
    let chat_demo = Some(
        chat::DemoSection::builder()
            .room_id(crate::types::Text::from(
                context.room.id.as_uuid().to_string(),
            ))
            .room_name(crate::types::Text::from(context.room.name.to_string()))
            .messages(context.messages)
            .interactivity(chat::Mode::from(is_authenticated))
            .build(),
    );

    Ok(views::render(
        pages::Lab::builder()
            .maybe_user(user)
            .maybe_chat_demo(chat_demo)
            .build(),
    ))
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
    let delta = signals.delta.unwrap_or_default();
    let session = crate::sse::Handle::from_cookies_with_tab(
        &cookies,
        &state.cookie_key,
        signals.sse_tab_id.clone(),
    );
    if !state.sse.has_streams_for_session(&session.id()) {
        return axum::http::StatusCode::PRECONDITION_REQUIRED;
    }

    let mut current = state.demo.counter.server_count.load(Ordering::Relaxed);
    loop {
        let next = (current + delta).max(0);
        match state.demo.counter.server_count.compare_exchange(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                let event = crate::sse::Event::patch_signals(serde_json::json!({
                    "server_count": next,
                    "server_connected": true
                }));
                return match state.sse.send(&session, event) {
                    Ok(_) => axum::http::StatusCode::NO_CONTENT,
                    Err(crate::sse::SendError::SessionMissing) => {
                        axum::http::StatusCode::PRECONDITION_REQUIRED
                    }
                    Err(crate::sse::SendError::SendFailed) => {
                        axum::http::StatusCode::SERVICE_UNAVAILABLE
                    }
                };
            }
            Err(observed) => current = observed,
        }
    }
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
    let session = crate::sse::Handle::from_cookies_with_tab(
        &cookies,
        &state.cookie_key,
        signals.sse_tab_id.clone(),
    );
    let query = signals
        .operations_filter_query
        .map(|value| value.to_string());
    state.trace_log.set_stream_flow_filter(
        &session.id(),
        signals.sse_tab_id.as_ref(),
        query.as_deref(),
    );
    state
        .trace_log
        .refresh_stream_log_panels(&session.id(), signals.sse_tab_id.as_ref());
    axum::http::StatusCode::NO_CONTENT
}
