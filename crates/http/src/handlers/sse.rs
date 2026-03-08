use async_stream::stream;
use axum::{
    extract::Extension,
    http::header::{CACHE_CONTROL, HeaderName, HeaderValue},
    response::Sse,
};
use core::convert::Infallible;
use datastar::axum::ReadSignals;
use maud::Render;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;
use tokio::time::{Duration, sleep};
use tower_cookies::Cookies;

use crate::types::{SseTabId, Text};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SurrealSignals {
    surreal_message: Option<Text>,
    original_surreal_message: Option<Text>,
    sse_tab_id: Option<SseTabId>,
    _surreal_status: Option<Text>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EventSignals {
    sse_tab_id: Option<SseTabId>,
    operations_filter_query: Option<Text>,
}

fn surreal_payload(message: &Text, status: &Text) -> crate::sse::Event {
    crate::sse::Event::patch_signals(serde_json::json!({
        "surrealMessage": message.to_string(),
        "surrealStatus": status.to_string(),
    }))
}

fn surreal_send(
    state: &crate::State,
    session: &crate::sse::Handle,
    message: Text,
    status: Text,
) -> bool {
    match state.sse.send(session, surreal_payload(&message, &status)) {
        Ok(()) => true,
        Err(err) => {
            tracing::debug!(?err, "sse session missing for surreal update");
            false
        }
    }
}

fn surreal_original(signals: SurrealSignals) -> Text {
    signals
        .original_surreal_message
        .or(signals.surreal_message)
        .unwrap_or_else(|| Text::from("Ready."))
}

pub async fn surreal_message_guarded(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    let session = crate::sse::Handle::from_cookies_with_tab(
        &cookies,
        &state.cookie_key,
        signals.sse_tab_id.clone(),
    );
    let stream_key = session.stream_key().clone();
    let sequence = state
        .demo
        .surreal
        .seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let lock = state
        .demo
        .surreal
        .guard
        .entry(stream_key)
        .or_insert_with(|| std::sync::Arc::new(tokio::sync::Mutex::new(())))
        .clone();

    tokio::spawn(async move {
        let guard = match lock.try_lock() {
            Ok(guard) => {
                if !surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Guarded says hi! #{sequence}")),
                    Text::from(format!("guarded running #{sequence}")),
                ) {
                    return;
                }
                guard
            }
            Err(_) => {
                if !surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Guarded queued #{sequence}")),
                    Text::from(format!("guarded queued #{sequence}")),
                ) {
                    return;
                }
                let guard = lock.lock().await;
                if !surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Guarded says hi! #{sequence}")),
                    Text::from(format!("guarded running #{sequence}")),
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
            original,
            Text::from(format!("guarded done #{sequence}")),
        );
    });

    axum::http::StatusCode::ACCEPTED
}

pub async fn surreal_message_cancel(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    let session = crate::sse::Handle::from_cookies_with_tab(
        &cookies,
        &state.cookie_key,
        signals.sse_tab_id.clone(),
    );
    let stream_key = session.stream_key().clone();
    let sequence = state
        .demo
        .surreal
        .seq
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        + 1;
    let original = surreal_original(signals);

    let token = tokio_util::sync::CancellationToken::new();
    if let Some(previous) = state.demo.surreal.cancel.insert(stream_key, token.clone()) {
        previous.cancel();
    }

    tokio::spawn(async move {
        if !surreal_send(
            &state,
            &session,
            Text::from(format!("Cancelled says hi! #{sequence}")),
            Text::from(format!("cancel running #{sequence}")),
        ) {
            return;
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(1)) => {
                surreal_send(
                    &state,
                    &session,
                    original,
                    Text::from(format!("cancel done #{sequence}")),
                );
            }
            _ = token.cancelled() => {
                surreal_send(
                    &state,
                    &session,
                    Text::from(format!("Cancelled #{sequence}")),
                    Text::from(format!("cancelled #{sequence}")),
                );
            }
        }
    });

    axum::http::StatusCode::ACCEPTED
}

pub async fn events(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<EventSignals>,
) -> impl axum::response::IntoResponse {
    let session = crate::sse::Handle::from_cookies_with_tab(
        &cookies,
        &state.cookie_key,
        signals.sse_tab_id.clone(),
    );
    let session_id = session.id();
    let filter_query = signals
        .operations_filter_query
        .as_ref()
        .map(ToString::to_string);
    state
        .trace_log
        .set_session_flow_filter(&session_id, filter_query.as_deref());
    let stream_key = session.stream_key().clone();
    let (mut receiver, guard) = state.sse.subscribe(&session);
    let cleanup_guard = ConnectionCleanupGuard::new(
        stream_key,
        state.demo.surreal.guard.clone(),
        state.demo.surreal.cancel.clone(),
    );

    tracing::info!(session_id = %session_id, "sse connected");
    let _ = state.sse.send(
        &session,
        crate::sse::Event::patch_signals(serde_json::json!({
            "sseConnected": true
        })),
    );
    let session_entries = state.trace_log.snapshot_session(&session_id);
    let transport_log = crate::views::partials::TransportLogSet::builder()
        .entries(&session_entries)
        .build()
        .render()
        .into_string();
    let _ = state
        .sse
        .send(&session, crate::sse::Event::patch_elements(transport_log));

    let stream = stream! {
        let _cleanup_guard = cleanup_guard;
        let _guard = guard;
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

    let sse = Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    );
    (
        [
            (
                CACHE_CONTROL,
                HeaderValue::from_static("no-cache, no-transform"),
            ),
            (
                HeaderName::from_static("x-accel-buffering"),
                HeaderValue::from_static("no"),
            ),
        ],
        sse,
    )
}

struct ConnectionCleanupGuard {
    stream_key: crate::sse::StreamKey,
    surreal_guard: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, std::sync::Arc<tokio::sync::Mutex<()>>>,
    >,
    surreal_cancel: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, tokio_util::sync::CancellationToken>,
    >,
}

impl ConnectionCleanupGuard {
    fn new(
        stream_key: crate::sse::StreamKey,
        surreal_guard: std::sync::Arc<
            dashmap::DashMap<crate::sse::StreamKey, std::sync::Arc<tokio::sync::Mutex<()>>>,
        >,
        surreal_cancel: std::sync::Arc<
            dashmap::DashMap<crate::sse::StreamKey, tokio_util::sync::CancellationToken>,
        >,
    ) -> Self {
        Self {
            stream_key,
            surreal_guard,
            surreal_cancel,
        }
    }
}

impl Drop for ConnectionCleanupGuard {
    fn drop(&mut self) {
        if let Some((_, token)) = self.surreal_cancel.remove(&self.stream_key) {
            token.cancel();
        }
        self.surreal_guard.remove(&self.stream_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace_log::TraceEntry;
    use crate::types::{
        LogLevelText, LogMessageText, LogTargetText, RequestId, SessionId, TimestampText,
    };
    use dashmap::DashMap;

    fn trace_entry(message: &str) -> TraceEntry {
        TraceEntry::builder()
            .timestamp(TimestampText::new("2026-02-24 00:00:00"))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new("demo.request"))
            .message(LogMessageText::new(message))
            .fields(Vec::new())
            .build()
    }

    #[test]
    fn cleanup_preserves_trace_entries_across_disconnects() {
        let registry = crate::sse::Registry::new();
        let trace_log = crate::trace_log::TraceLogStore::builder()
            .with_sse(registry.clone())
            .with_emit_sse(false)
            .build();
        let session_id = SessionId::new("session-1");
        let tab_a =
            crate::sse::Handle::with_tab(session_id.clone(), Some(SseTabId::new("tab-a")));
        let tab_b =
            crate::sse::Handle::with_tab(session_id.clone(), Some(SseTabId::new("tab-b")));
        let (_rx_a, guard_a) = registry.subscribe(&tab_a);
        let (_rx_b, guard_b) = registry.subscribe(&tab_b);

        let surreal_guard = std::sync::Arc::new(DashMap::<
            crate::sse::StreamKey,
            std::sync::Arc<tokio::sync::Mutex<()>>,
        >::new());
        let surreal_cancel = std::sync::Arc::new(DashMap::<
            crate::sse::StreamKey,
            tokio_util::sync::CancellationToken,
        >::new());
        surreal_guard.insert(
            tab_a.stream_key().clone(),
            std::sync::Arc::new(tokio::sync::Mutex::new(())),
        );
        surreal_guard.insert(
            tab_b.stream_key().clone(),
            std::sync::Arc::new(tokio::sync::Mutex::new(())),
        );
        surreal_cancel.insert(
            tab_a.stream_key().clone(),
            tokio_util::sync::CancellationToken::new(),
        );
        surreal_cancel.insert(
            tab_b.stream_key().clone(),
            tokio_util::sync::CancellationToken::new(),
        );

        let cleanup_a = ConnectionCleanupGuard::new(
            tab_a.stream_key().clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
        );
        let cleanup_b = ConnectionCleanupGuard::new(
            tab_b.stream_key().clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
        );

        trace_log.record_with_session(
            &RequestId::new("req-1"),
            Some(&session_id),
            trace_entry("request.end"),
        );
        assert!(!trace_log.snapshot_session(&session_id).is_empty());

        drop(guard_a);
        drop(cleanup_a);
        assert!(!trace_log.snapshot_session(&session_id).is_empty());
        assert!(surreal_guard.get(tab_a.stream_key()).is_none());
        assert!(surreal_cancel.get(tab_a.stream_key()).is_none());
        assert!(surreal_guard.get(tab_b.stream_key()).is_some());

        drop(guard_b);
        drop(cleanup_b);
        assert!(!trace_log.snapshot_session(&session_id).is_empty());
        assert!(surreal_guard.get(tab_b.stream_key()).is_none());
        assert!(surreal_cancel.get(tab_b.stream_key()).is_none());
    }
}
