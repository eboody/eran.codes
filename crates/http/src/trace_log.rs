use std::{
    collections::VecDeque,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use tracing::{Event, Level};
use tracing_subscriber::{Layer, registry::LookupSpan};

use crate::{request, sse, views};
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
    RequestId, SessionId, TimestampText,
};
use crate::paths::Route;
use bon::{bon, Builder};
use maud::Render;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Builder)]
pub struct TraceEntry {
    pub timestamp: TimestampText,
    pub level: LogLevelText,
    pub target: LogTargetText,
    pub message: LogMessageText,
    pub fields: Vec<(LogFieldName, LogFieldValue)>,
}

#[derive(Clone)]
pub struct TraceLogStore {
    requests: Arc<DashMap<RequestId, VecDeque<TraceEntry>>>,
    sessions: Arc<DashMap<SessionId, VecDeque<TraceEntry>>>,
    global: Arc<Mutex<VecDeque<TraceEntry>>>,
    max_entries: usize,
    sse: sse::Registry,
}

impl TraceLogStore {
    pub fn new(
        sse: sse::Registry,
        max_entries: usize,
    ) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            sessions: Arc::new(DashMap::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
            max_entries,
            sse,
        }
    }

    pub fn record_with_session(
        &self,
        request_id: &RequestId,
        session_id: Option<&SessionId>,
        entry: TraceEntry,
    ) {
        let mut queue = self
            .requests
            .entry(request_id.clone())
            .or_insert_with(VecDeque::new);
        if queue.len() >= self.max_entries {
            queue.pop_front();
        }
        queue.push_back(entry);

        let entry = queue.back().cloned().expect("entry");

        if let Some(session_id) = session_id {
            let mut session_queue = self
                .sessions
                .entry(session_id.clone())
                .or_insert_with(VecDeque::new);
            if session_queue.len() >= self.max_entries {
                session_queue.pop_front();
            }
            session_queue.push_back(entry.clone());
        }

        if let Ok(mut global) = self.global.lock() {
            if global.len() >= self.max_entries {
                global.pop_front();
            }
            global.push_back(entry);
        }

        if let Some(session_id) = session_id {
            let entries = self.snapshot_session(session_id);
            let live_log = views::partials::LiveLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let network_log = views::partials::NetworkLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(live_log));
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(network_log));
        }
    }

    pub fn record_sse_event(
        &self,
        session_id: Option<&SessionId>,
        entry: TraceEntry,
    ) {
        if let Some(session_id) = session_id {
            let mut session_queue = self
                .sessions
                .entry(session_id.clone())
                .or_insert_with(VecDeque::new);
            if session_queue.len() >= self.max_entries {
                session_queue.pop_front();
            }
            session_queue.push_back(entry.clone());
        }

        if let Ok(mut global) = self.global.lock() {
            if global.len() >= self.max_entries {
                global.pop_front();
            }
            global.push_back(entry);
        }

        if let Some(session_id) = session_id {
            let entries = self.snapshot_session(session_id);
            let live_log = views::partials::LiveLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let network_log = views::partials::NetworkLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(live_log));
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(network_log));
        }
    }

    pub fn snapshot_request(
        &self,
        request_id: &RequestId,
    ) -> Vec<TraceEntry> {
        self.requests
            .get(request_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_session(
        &self,
        session_id: &SessionId,
    ) -> Vec<TraceEntry> {
        self.sessions
            .get(session_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn clear_session(
        &self,
        session_id: &SessionId,
    ) {
        self.sessions.remove(session_id);
    }

    pub fn snapshot_global(&self) -> Vec<TraceEntry> {
        self.global
            .lock()
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[bon]
impl TraceLogStore {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_sse))] sse: sse::Registry,
        #[builder(default = 50, setters(name = with_max_entries))] max_entries: usize,
    ) -> Self {
        Self::new(sse, max_entries)
    }
}

pub struct TraceLogLayer {
    store: TraceLogStore,
}

impl TraceLogLayer {
    pub fn new(store: TraceLogStore) -> Self {
        Self { store }
    }
}

pub struct DiagnosticTraceLogLayer {
    store: TraceLogStore,
}

impl DiagnosticTraceLogLayer {
    pub fn new(store: TraceLogStore) -> Self {
        Self { store }
    }
}

impl<S> Layer<S> for TraceLogLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let context = request::current_context();
        let Some(request_id) = context
            .as_ref()
            .and_then(|value| value.request_id.as_ref())
            .cloned()
        else {
            return;
        };

        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let level = *event.metadata().level();
        let target = event.metadata().target();
        let message = visitor
            .message
            .unwrap_or_else(|| LogMessageText::new(event.metadata().name()));
        let target_kind = LogTargetKind::from_str(target);
        let message_kind = LogMessageKind::from_str(&message.to_string());
        if should_skip_event(&target_kind, &message_kind) {
            return;
        }
        if target_kind.is_diagnostic() {
            return;
        }
        let has_db = visitor
            .fields
            .iter()
            .any(|(name, _)| matches!(FieldName::from_str(&name.to_string()), FieldName::Known(FieldNameKnown::DbStatement)));
        let is_demo = target_kind.is_demo();
        let is_info = matches!(level, Level::INFO | Level::WARN | Level::ERROR);
        let is_sse = target_kind.is_demo_sse();

        if is_sse || !(is_info || has_db || is_demo) {
            return;
        }

        let entry = TraceEntry::builder()
            .timestamp(now_timestamp_short())
            .level(LogLevelText::new(level.to_string()))
            .target(LogTargetText::new(target.to_string()))
            .message(LogMessageText::new(message.to_string()))
            .fields(visitor.fields)
            .build();

        let session_id = context
            .as_ref()
            .and_then(|value| value.session_id.as_ref());
        self.store
            .record_with_session(&request_id, session_id, entry);
    }
}

impl<S> Layer<S> for DiagnosticTraceLogLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let context = request::current_context();
        let Some(request_id) = context
            .as_ref()
            .and_then(|value| value.request_id.as_ref())
            .cloned()
        else {
            return;
        };

        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let level = *event.metadata().level();
        let target = event.metadata().target();
        let message = visitor
            .message
            .unwrap_or_else(|| LogMessageText::new(event.metadata().name()));
        let target_kind = LogTargetKind::from_str(target);
        let message_kind = LogMessageKind::from_str(&message.to_string());

        let is_request_start = matches!(
            target_kind,
            LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic)
        );
        let is_request_completed = matches!(
            (target_kind, message_kind),
            (
                LogTargetKind::Known(LogTargetKnown::HttpRouterLayers),
                LogMessageKind::Known(LogMessageKnown::RequestCompleted)
            )
        );

        if !(is_request_start || is_request_completed) {
            return;
        }

        let entry = TraceEntry::builder()
            .timestamp(now_timestamp_short())
            .level(LogLevelText::new(level.to_string()))
            .target(LogTargetText::new(target.to_string()))
            .message(LogMessageText::new(message.to_string()))
            .fields(visitor.fields)
            .build();

        let session_id = context
            .as_ref()
            .and_then(|value| value.session_id.as_ref());
        self.store
            .record_with_session(&request_id, session_id, entry);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum LogTargetKnown {
    #[strum(serialize = "demo.request")]
    DemoRequest,
    #[strum(serialize = "demo.request.diagnostic")]
    DemoRequestDiagnostic,
    #[strum(serialize = "demo.sse")]
    DemoSse,
    #[strum(serialize = "demo.chat")]
    DemoChat,
    #[strum(serialize = "http::router::layers")]
    HttpRouterLayers,
}

#[derive(Clone, Debug)]
pub enum LogTargetKind {
    Known(LogTargetKnown),
    Other(LogTargetText),
}

impl LogTargetKind {
    pub fn from_str(value: &str) -> Self {
        LogTargetKnown::from_str(value)
            .map(Self::Known)
            .unwrap_or_else(|_| Self::Other(LogTargetText::new(value)))
    }

    pub fn is_demo(&self) -> bool {
        matches!(
            self,
            Self::Known(LogTargetKnown::DemoRequest)
                | Self::Known(LogTargetKnown::DemoSse)
                | Self::Known(LogTargetKnown::DemoChat)
        )
    }

    pub fn is_diagnostic(&self) -> bool {
        matches!(self, Self::Known(LogTargetKnown::DemoRequestDiagnostic))
    }

    pub fn is_demo_sse(&self) -> bool {
        matches!(self, Self::Known(LogTargetKnown::DemoSse))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum LogMessageKnown {
    #[strum(serialize = "request.end")]
    RequestEnd,
    #[strum(serialize = "request completed")]
    RequestCompleted,
    #[strum(serialize = "chat.message.incoming")]
    ChatMessageIncoming,
    #[strum(serialize = "chat message broadcast")]
    ChatMessageBroadcast,
}

#[derive(Clone, Debug)]
pub enum LogMessageKind {
    Known(LogMessageKnown),
    Other(LogMessageText),
}

impl LogMessageKind {
    pub fn from_str(value: &str) -> Self {
        LogMessageKnown::from_str(value)
            .map(Self::Known)
            .unwrap_or_else(|_| Self::Other(LogMessageText::new(value)))
    }
}

fn should_skip_event(target: &LogTargetKind, message: &LogMessageKind) -> bool {
    matches!(
        (target, message),
        (
            LogTargetKind::Known(LogTargetKnown::HttpRouterLayers),
            LogMessageKind::Known(LogMessageKnown::RequestCompleted)
        )
    )
}

#[derive(Default)]
struct FieldVisitor {
    fields: Vec<(LogFieldName, LogFieldValue)>,
    message: Option<LogMessageText>,
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(
        &mut self,
        field: &tracing::field::Field,
        value: &dyn core::fmt::Debug,
    ) {
        let value = format!("{value:?}");
        match FieldName::from_str(field.name()) {
            FieldName::Known(FieldNameKnown::Message) => {
                self.message = Some(LogMessageText::new(value));
            }
            _ => {
                self.fields
                    .push((
                        LogFieldName::new(field.name()),
                        LogFieldValue::new(value),
                    ));
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
enum FieldNameKnown {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "db_statement")]
    DbStatement,
}

#[derive(Clone, Debug)]
enum FieldName {
    Known(FieldNameKnown),
    Other,
}

impl FieldName {
    fn from_str(value: &str) -> Self {
        FieldNameKnown::from_str(value)
            .map(Self::Known)
            .unwrap_or(Self::Other)
    }
}

pub async fn audit_middleware(
    Extension(state): Extension<crate::State>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let started_at = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let request_id = request::current_context()
        .and_then(|value| value.request_id)
        .unwrap_or_else(RequestId::unknown);
    let session_id = request::current_context()
        .and_then(|value| value.session_id);
    let user_id = request::current_context()
        .and_then(|value| value.user_id);

    tracing::info!(
        target: "demo.request.diagnostic",
        message = "request.start",
        method = %method,
        path = %path,
        request_id = %request_id
    );

    let response = next.run(req).await;
    let latency_ms = started_at.elapsed().as_millis().to_string();
    let sender = match Route::from_path(path.as_str()) {
        Some(Route::ChatMessages) => ChatSender::You,
        Some(Route::ChatMessagesDemo) => ChatSender::Demo,
        _ => ChatSender::Unknown,
    };
    let sent_at = now_timestamp_short();

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_ref(),
        TraceEntry::builder()
            .timestamp(now_timestamp_short())
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new("demo.request"))
            .message(LogMessageText::new("request.end"))
            .fields(vec![
                (
                    LogFieldName::new("method"),
                    LogFieldValue::new(method),
                ),
                (LogFieldName::new("path"), LogFieldValue::new(path)),
                (
                    LogFieldName::new("status"),
                    LogFieldValue::new(response.status().as_u16().to_string()),
                ),
                (
                    LogFieldName::new("latency_ms"),
                    LogFieldValue::new(latency_ms),
                ),
                (
                    LogFieldName::new("request_id"),
                    LogFieldValue::new(request_id.to_string()),
                ),
                (
                    LogFieldName::new("session_id"),
                    session_id
                        .clone()
                        .map(|value| LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(LogFieldValue::missing),
                ),
                (
                    LogFieldName::new("user_id"),
                    user_id
                        .clone()
                        .map(|value| LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(LogFieldValue::missing),
                ),
                (
                    LogFieldName::new("sender"),
                    LogFieldValue::new(sender.as_str()),
                ),
                (
                    LogFieldName::new("sent_at"),
                    LogFieldValue::new(sent_at.to_string()),
                ),
            ])
            .build(),
    );

    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(target: "demo.request", "response error");
    }

    response
}

#[derive(Clone, Copy, Debug)]
enum ChatSender {
    You,
    Demo,
    Unknown,
}

impl ChatSender {
    fn as_str(self) -> &'static str {
        match self {
            ChatSender::You => "you",
            ChatSender::Demo => "demo",
            ChatSender::Unknown => "-",
        }
    }
}

pub fn now_timestamp_short() -> TimestampText {
    let raw = jiff::Timestamp::now().to_string();
    format_timestamp(TimestampText::new(raw))
}

fn format_timestamp(raw: TimestampText) -> TimestampText {
    let raw_value = raw.to_string();
    let mut parts = raw_value.split('T');
    let Some(date) = parts.next() else {
        return raw;
    };
    let Some(time) = parts.next() else {
        return raw;
    };
    let time = time.trim_end_matches('Z');
    let time = time.split('.').next().unwrap_or(time);
    TimestampText::new(format!("{date} {time}"))
}
