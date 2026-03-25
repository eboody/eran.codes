use std::{str::FromStr, time::Instant};

use axum::{body::Body, extract::Extension, http, middleware::Next, response::Response};
use tracing::{Event, Level};
use tracing_subscriber::{layer::Layer as SubscriberLayer, registry::LookupSpan};

use super::demo_chat::Sender as ChatSender;
use super::log::{self, message, target};
use super::store::TraceEntry;
use super::{Store, now_timestamp_short};
use crate::request;
use crate::types::{
    LogFieldKey, LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
    RequestId,
};

pub struct Layer {
    store: Store,
}

impl Layer {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

pub struct DiagnosticLayer {
    store: Store,
}

impl DiagnosticLayer {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

impl<S> SubscriberLayer<S> for Layer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
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
        append_context_fields(&mut visitor.fields, context.as_ref(), &request_id);

        let level = *event.metadata().level();
        let target = event.metadata().target();
        let message = visitor
            .message
            .unwrap_or_else(|| LogMessageText::new(event.metadata().name()));
        let (target_kind, message_kind) = log::classify(target, &message.to_string());
        if log::should_skip_event(&target_kind, &message_kind) {
            return;
        }
        if target_kind.is_diagnostic() {
            return;
        }
        let has_db = visitor.fields.iter().any(|(name, _)| {
            matches!(LogFieldKey::try_from(name), Ok(LogFieldKey::DbStatement))
        });
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

        let session_id = context.as_ref().and_then(|value| value.session_id.as_ref());
        self.store
            .record_with_session(&request_id, session_id, entry);
    }
}

impl<S> SubscriberLayer<S> for DiagnosticLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
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
        append_context_fields(&mut visitor.fields, context.as_ref(), &request_id);

        let level = *event.metadata().level();
        let target = event.metadata().target();
        let message = visitor
            .message
            .unwrap_or_else(|| LogMessageText::new(event.metadata().name()));
        let (target_kind, message_kind) = log::classify(target, &message.to_string());

        let is_request_start = matches!(
            target_kind,
            target::Kind::Known(target::Known::DemoRequestDiagnostic)
        );
        let is_request_completed = matches!(
            (target_kind, message_kind),
            (
                target::Kind::Known(target::Known::HttpRouterLayers),
                message::Kind::Known(message::Known::RequestCompleted)
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

        let session_id = context.as_ref().and_then(|value| value.session_id.as_ref());
        self.store
            .record_with_session(&request_id, session_id, entry);
    }
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
        match LogFieldKey::from_str(field.name()) {
            Ok(LogFieldKey::Message) => {
                self.message = Some(LogMessageText::new(value));
            }
            _ => {
                self.fields
                    .push((LogFieldName::new(field.name()), LogFieldValue::new(value)));
            }
        }
    }
}

fn append_context_fields(
    fields: &mut Vec<(LogFieldName, LogFieldValue)>,
    context: Option<&request::Context>,
    request_id: &RequestId,
) {
    upsert_context_field(fields, LogFieldKey::RequestId, Some(request_id.to_string()));
    upsert_context_field(
        fields,
        LogFieldKey::SessionId,
        context
            .and_then(|value| value.session_id.as_ref())
            .map(ToString::to_string),
    );
    upsert_context_field(
        fields,
        LogFieldKey::UserId,
        context
            .and_then(|value| value.user_id.as_ref())
            .map(ToString::to_string),
    );
    upsert_context_field(
        fields,
        LogFieldKey::SseTabId,
        context
            .and_then(|value| value.sse_tab_id.as_ref())
            .map(ToString::to_string),
    );
}

fn upsert_context_field(
    fields: &mut Vec<(LogFieldName, LogFieldValue)>,
    key: LogFieldKey,
    value: Option<String>,
) {
    let name = LogFieldName::from(key);
    if fields.iter().any(|(field_name, _)| field_name == &name) {
        return;
    }
    let value = value
        .map(LogFieldValue::new)
        .unwrap_or_else(LogFieldValue::missing);
    fields.push((name, value));
}

pub async fn audit_middleware(
    Extension(state): Extension<crate::State>,
    req: http::Request<Body>,
    next: Next,
) -> Response {
    let started_at = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let skip_operational_trace = should_skip_operational_path(path.as_str());
    let request_id = request::current_context()
        .and_then(|value| value.request_id)
        .unwrap_or_else(RequestId::unknown);

    if !skip_operational_trace {
        tracing::info!(
            target: target::Known::DemoRequestDiagnostic.as_str(),
            message = message::Known::RequestStart.as_str(),
            method = %method,
            path = %path,
            request_id = %request_id
        );
    }

    let response = next.run(req).await;
    if skip_operational_trace {
        return response;
    }
    let context = request::current_context();
    let session_id = context.as_ref().and_then(|value| value.session_id.clone());
    let user_id = context.as_ref().and_then(|value| value.user_id.clone());
    let sse_tab_id = context.as_ref().and_then(|value| value.sse_tab_id.clone());
    let latency_ms = started_at.elapsed().as_millis().to_string();
    let sender = path
        .parse::<crate::paths::Route>()
        .ok()
        .and_then(|route| ChatSender::try_from(route).ok())
        .unwrap_or_default();
    let sent_at = now_timestamp_short();

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_ref(),
        TraceEntry::builder()
            .timestamp(now_timestamp_short())
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::from(target::Known::DemoRequest))
            .message(LogMessageText::from(message::Known::RequestEnd))
            .fields(vec![
                (
                    LogFieldName::from(LogFieldKey::Method),
                    LogFieldValue::new(method),
                ),
                (
                    LogFieldName::from(LogFieldKey::Path),
                    LogFieldValue::new(path),
                ),
                (
                    LogFieldName::from(LogFieldKey::Status),
                    LogFieldValue::new(response.status().as_u16().to_string()),
                ),
                (
                    LogFieldName::from(LogFieldKey::LatencyMs),
                    LogFieldValue::new(latency_ms),
                ),
                (
                    LogFieldName::from(LogFieldKey::RequestId),
                    LogFieldValue::new(request_id.to_string()),
                ),
                (
                    LogFieldName::from(LogFieldKey::SessionId),
                    session_id
                        .clone()
                        .map(|value| LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(LogFieldValue::missing),
                ),
                (
                    LogFieldName::from(LogFieldKey::UserId),
                    user_id
                        .clone()
                        .map(|value| LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(LogFieldValue::missing),
                ),
                (
                    LogFieldName::from(LogFieldKey::SseTabId),
                    sse_tab_id
                        .clone()
                        .map(|value| LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(LogFieldValue::missing),
                ),
                (
                    LogFieldName::from(LogFieldKey::Sender),
                    LogFieldValue::new(sender.as_ref()),
                ),
                (
                    LogFieldName::from(LogFieldKey::SentAt),
                    LogFieldValue::new(sent_at.to_string()),
                ),
            ])
            .build(),
    );

    if response.status() == http::StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(target: target::Known::DemoRequest.as_str(), "response error");
    }

    response
}

fn should_skip_operational_path(path: &str) -> bool {
    if path == "/" || path == "/api/operations/filter" || path == "/events" {
        return true;
    }
    if path == "/static" || path.starts_with("/static/") {
        return true;
    }
    path.contains("livereload")
}

#[cfg(test)]
mod tests {
    use crate::types::{SessionId, SseTabId, UserIdText};

    use super::*;

    #[test]
    fn append_context_fields_adds_request_session_user() {
        let mut fields = Vec::new();
        let request_id = RequestId::new("req-123");
        let context = request::Context {
            request_id: Some(request_id.clone()),
            session_id: Some(SessionId::new("session-abc")),
            sse_tab_id: Some(SseTabId::new("tab-1")),
            user_id: Some(UserIdText::new("user-xyz")),
            client_ip: None,
            user_agent: None,
            kind: request::Kind::Datastar,
        };

        append_context_fields(&mut fields, Some(&context), &request_id);

        assert!(fields.iter().any(|(name, value)| {
            name == &LogFieldName::from(LogFieldKey::RequestId)
                && value.to_string() == "req-123"
        }));
        assert!(fields.iter().any(|(name, value)| {
            name == &LogFieldName::from(LogFieldKey::SessionId)
                && value.to_string() == "session-abc"
        }));
        assert!(fields.iter().any(|(name, value)| {
            name == &LogFieldName::from(LogFieldKey::UserId)
                && value.to_string() == "user-xyz"
        }));
        assert!(fields.iter().any(|(name, value)| {
            name == &LogFieldName::from(LogFieldKey::SseTabId)
                && value.to_string() == "tab-1"
        }));
    }

    #[test]
    fn append_context_fields_does_not_duplicate_existing_request_id() {
        let mut fields = vec![(
            LogFieldName::from(LogFieldKey::RequestId),
            LogFieldValue::new("req-preexisting"),
        )];
        let request_id = RequestId::new("req-123");

        append_context_fields(&mut fields, None, &request_id);

        let request_id_fields = fields
            .iter()
            .filter(|(name, _)| name == &LogFieldName::from(LogFieldKey::RequestId))
            .count();
        assert_eq!(request_id_fields, 1);
        assert_eq!(fields[0].1.to_string(), "req-preexisting");
    }

    #[test]
    fn operational_path_skip_rejects_internal_and_static_routes() {
        assert!(should_skip_operational_path("/"));
        assert!(should_skip_operational_path("/api/operations/filter"));
        assert!(should_skip_operational_path("/events"));
        assert!(should_skip_operational_path("/static"));
        assert!(should_skip_operational_path("/static/app.css"));
        assert!(should_skip_operational_path("/__livereload"));
        assert!(should_skip_operational_path("/foo/livereload/socket"));
        assert!(!should_skip_operational_path("/demo/chat/messages"));
    }
}
