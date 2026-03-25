// ci: descriptive-module-import crate::handlers::sse
mod surreal;

use async_stream::stream;
use axum::{
    extract::Extension,
    http::header::{CACHE_CONTROL, HeaderName, HeaderValue},
    response::Sse,
};
use core::convert::Infallible;
use datastar::axum::ReadSignals;
use serde::Deserialize;
use statum::{machine, state, transition};
use tokio::sync::broadcast::error;
use tokio::time::Duration;
use tower_cookies::Cookies;

use crate::types::{SessionId, SseTabId, Text};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SurrealSignals {
    pub(crate) surreal_message: Option<Text>,
    pub(crate) original_surreal_message: Option<Text>,
    pub(crate) sse_tab_id: Option<SseTabId>,
    pub(crate) _surreal_status: Option<Text>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EventSignals {
    sse_tab_id: Option<SseTabId>,
    operations_filter_query: Option<Text>,
}

#[state]
enum EventsConnectionState {
    Incoming,
    FilterApplied,
    Ready(ReadyData),
}

struct ReadyData {
    receiver: tokio::sync::broadcast::Receiver<crate::sse::Event>,
    session_guard: crate::sse::SessionGuard,
    cleanup_guard: ConnectionCleanupGuard,
}

#[machine]
struct EventsConnectionFlow<EventsConnectionState> {
    state: crate::State,
    session: crate::sse::Handle,
    session_id: SessionId,
    tab_id: Option<SseTabId>,
    filter_query: Option<Text>,
}

impl EventsConnectionFlow<Incoming> {
    fn from_request(state: crate::State, cookies: Cookies, signals: EventSignals) -> Self {
        let tab_id = signals.sse_tab_id.clone();
        if let Some(tab_id) = tab_id.clone() {
            crate::request::set_sse_tab_id(tab_id);
        }
        let session = crate::sse::Handle::from_cookies_with_tab(
            &cookies,
            &state.cookie_key,
            tab_id.clone(),
        );
        let session_id = session.id();

        EventsConnectionFlow::<Incoming>::builder()
            .state(state)
            .session(session)
            .session_id(session_id)
            .tab_id(tab_id)
            .filter_query(signals.operations_filter_query)
            .build()
    }
}

impl<S: EventsConnectionStateTrait> EventsConnectionFlow<S> {
    fn session_id(&self) -> &SessionId {
        &self.session_id
    }
}

#[transition]
impl EventsConnectionFlow<Incoming> {
    fn apply_flow_filter(self) -> EventsConnectionFlow<FilterApplied> {
        let query = self.filter_query.as_ref().map(ToString::to_string);
        self.state.trace_log.set_stream_flow_filter(
            self.session_id(),
            self.tab_id.as_ref(),
            query.as_deref(),
        );
        self.transition()
    }
}

#[transition]
impl EventsConnectionFlow<FilterApplied> {
    fn prepare_stream(self) -> EventsConnectionFlow<Ready> {
        let stream_key = self.session.stream_key().clone();
        let (receiver, session_guard) = self.state.sse.subscribe(&self.session);
        let cleanup_guard = ConnectionCleanupGuard::new(
            stream_key,
            self.state.sse.clone(),
            self.state.demo.surreal.guard.clone(),
            self.state.demo.surreal.cancel.clone(),
            self.state.demo.chat_room_bindings.clone(),
            self.state.trace_log.clone(),
        );
        let session_id = self.session_id().clone();
        tracing::info!(session_id = %session_id, "sse connected");
        if let Err(error) = self.state.sse.send(
            &self.session,
            crate::sse::Event::patch_signals(serde_json::json!({
                "sseConnected": true
            })),
        ) {
            tracing::warn!(session_id = %session_id, ?error, "initial SSE signal patch failed");
        }
        self.state
            .trace_log
            .refresh_stream_log_panels(&session_id, self.tab_id.as_ref());
        self.transition_with(ReadyData {
            receiver,
            session_guard,
            cleanup_guard,
        })
    }
}

impl EventsConnectionFlow<Ready> {
    fn into_response(self) -> impl axum::response::IntoResponse {
        let ReadyData {
            mut receiver,
            session_guard,
            cleanup_guard,
        } = self.state_data;
        let session_id = self.session_id;

        let stream = stream! {
            // Bind cleanup first so the session guard drops before cleanup runs.
            // Registry presence is the truth surface for "last connection wins".
            let _cleanup_guard = cleanup_guard;
            let _session_guard = session_guard;
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        let sse_event = event.as_ref().write_as_axum_sse_event();
                        yield Ok::<_, Infallible>(sse_event);
                    }
                    Err(error::RecvError::Lagged(_)) => continue,
                    Err(error::RecvError::Closed) => {
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
}

fn surreal_payload(message: &Text, status: &Text) -> crate::sse::Event {
    crate::sse::Event::patch_signals(serde_json::json!({
        "surrealMessage": message.to_string(),
        "surrealStatus": status.to_string(),
    }))
}

pub(super) fn surreal_send(
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

pub(super) fn surreal_original(signals: SurrealSignals) -> Text {
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
    surreal::guarded_flow::IncomingFlow::from_request(state, cookies, signals)
        .prepare_lock()
        .spawn()
        .status_code()
}

pub async fn surreal_message_cancel(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<SurrealSignals>,
) -> impl axum::response::IntoResponse {
    surreal::cancel_flow::IncomingFlow::from_request(state, cookies, signals)
        .prepare_token()
        .spawn()
        .status_code()
}

pub async fn events(
    Extension(state): Extension<crate::State>,
    Extension(cookies): Extension<Cookies>,
    ReadSignals(signals): ReadSignals<EventSignals>,
) -> impl axum::response::IntoResponse {
    EventsConnectionFlow::<Incoming>::from_request(state, cookies, signals)
        .apply_flow_filter()
        .prepare_stream()
        .into_response()
}

struct ConnectionCleanupGuard {
    stream_key: crate::sse::StreamKey,
    sse: crate::sse::Registry,
    surreal_guard: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, std::sync::Arc<tokio::sync::Mutex<()>>>,
    >,
    surreal_cancel: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, tokio_util::sync::CancellationToken>,
    >,
    chat_room_bindings: crate::chat_demo::room::Bindings,
    trace_log: crate::trace_log::Store,
}

impl ConnectionCleanupGuard {
    fn new(
        stream_key: crate::sse::StreamKey,
        sse: crate::sse::Registry,
        surreal_guard: std::sync::Arc<
            dashmap::DashMap<crate::sse::StreamKey, std::sync::Arc<tokio::sync::Mutex<()>>>,
        >,
        surreal_cancel: std::sync::Arc<
            dashmap::DashMap<crate::sse::StreamKey, tokio_util::sync::CancellationToken>,
        >,
        chat_room_bindings: crate::chat_demo::room::Bindings,
        trace_log: crate::trace_log::Store,
    ) -> Self {
        Self {
            stream_key,
            sse,
            surreal_guard,
            surreal_cancel,
            chat_room_bindings,
            trace_log,
        }
    }
}

impl Drop for ConnectionCleanupGuard {
    fn drop(&mut self) {
        if self.sse.has_stream_key(&self.stream_key) {
            return;
        }
        if let Some((_, token)) = self.surreal_cancel.remove(&self.stream_key) {
            token.cancel();
        }
        self.surreal_guard.remove(&self.stream_key);
        self.chat_room_bindings.remove(&self.stream_key);
        self.trace_log.clear_stream_flow_filter(&self.stream_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace_log::store::TraceEntry;
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
        let trace_log = crate::trace_log::Store::builder()
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
            registry.clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
            crate::chat_demo::room::Bindings::new(),
            trace_log.clone(),
        );
        let cleanup_b = ConnectionCleanupGuard::new(
            tab_b.stream_key().clone(),
            registry.clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
            crate::chat_demo::room::Bindings::new(),
            trace_log.clone(),
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

    #[test]
    fn cleanup_waits_for_last_connection_on_same_stream_key() {
        let registry = crate::sse::Registry::new();
        let trace_log = crate::trace_log::Store::builder()
            .with_sse(registry.clone())
            .with_emit_sse(false)
            .build();
        let session_id = SessionId::new("session-dup");
        let handle = crate::sse::Handle::with_tab(
            session_id.clone(),
            Some(SseTabId::new("tab-dup")),
        );
        let (_rx_a, guard_a) = registry.subscribe(&handle);
        let (_rx_b, guard_b) = registry.subscribe(&handle);

        let surreal_guard = std::sync::Arc::new(DashMap::<
            crate::sse::StreamKey,
            std::sync::Arc<tokio::sync::Mutex<()>>,
        >::new());
        let surreal_cancel = std::sync::Arc::new(DashMap::<
            crate::sse::StreamKey,
            tokio_util::sync::CancellationToken,
        >::new());
        surreal_guard.insert(
            handle.stream_key().clone(),
            std::sync::Arc::new(tokio::sync::Mutex::new(())),
        );
        surreal_cancel.insert(
            handle.stream_key().clone(),
            tokio_util::sync::CancellationToken::new(),
        );
        let room_bindings = crate::chat_demo::room::Bindings::new();
        let room_id = domain::chat::room::Id::new_v4();
        room_bindings.bind(&handle, room_id);
        trace_log.set_stream_flow_filter(&session_id, handle.tab_id(), Some("chat"));

        let cleanup_a = ConnectionCleanupGuard::new(
            handle.stream_key().clone(),
            registry.clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
            room_bindings.clone(),
            trace_log.clone(),
        );
        let cleanup_b = ConnectionCleanupGuard::new(
            handle.stream_key().clone(),
            registry.clone(),
            surreal_guard.clone(),
            surreal_cancel.clone(),
            room_bindings.clone(),
            trace_log.clone(),
        );

        drop(guard_a);
        drop(cleanup_a);

        assert!(registry.has_stream_key(handle.stream_key()));
        assert!(trace_log.has_stream_flow_filter(handle.stream_key()));
        assert!(surreal_guard.get(handle.stream_key()).is_some());
        assert!(surreal_cancel.get(handle.stream_key()).is_some());
        assert_eq!(room_bindings.room_id_for(&handle), Some(room_id));

        drop(guard_b);
        drop(cleanup_b);

        assert!(!registry.has_stream_key(handle.stream_key()));
        assert!(!trace_log.has_stream_flow_filter(handle.stream_key()));
        assert!(surreal_guard.get(handle.stream_key()).is_none());
        assert!(surreal_cancel.get(handle.stream_key()).is_none());
        assert_eq!(room_bindings.room_id_for(&handle), None);
    }
}
