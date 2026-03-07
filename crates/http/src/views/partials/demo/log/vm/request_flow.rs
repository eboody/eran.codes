use std::collections::HashMap;

use crate::trace_log::{
    LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown, TraceEntry,
};
use crate::types::LogFieldKey;
use crate::types::Text;
use crate::views::partials::components::{Pill, logs};

use super::{field_text, short_request_id};

pub fn request_flows(entries: &[TraceEntry], max_flows: usize) -> Vec<logs::composed::Flow> {
    let mut order: Vec<String> = Vec::new();
    let mut flow_map: HashMap<String, FlowAggregate> = HashMap::new();

    for (index, entry) in entries.iter().enumerate() {
        let Some(kind) = flow_event_kind(entry) else {
            continue;
        };

        let flow_id = flow_id(entry, index);
        let key = flow_id.to_string();
        let display_id = field_text(entry, LogFieldKey::RequestId)
            .map(|value| short_request_id(&value))
            .unwrap_or_else(|| Text::from("orphan"));

        if !flow_map.contains_key(&key) {
            order.push(key.clone());
            flow_map.insert(
                key.clone(),
                FlowAggregate {
                    id: flow_id.clone(),
                    detail_id: Text::from(format!("network-flow-{}", slugify(&key))),
                    display_id,
                    latest_timestamp: Text::from(entry.timestamp.clone()),
                    latest_index: index,
                    method: None,
                    path: None,
                    status: None,
                    events: Vec::new(),
                },
            );
        }

        if let Some(aggregate) = flow_map.get_mut(&key) {
            aggregate.latest_timestamp = Text::from(entry.timestamp.clone());
            aggregate.latest_index = index;
            aggregate.events.push(build_flow_event(kind, entry));
            hydrate_request_fields(aggregate, entry, kind);
        }
    }

    let mut flows: Vec<FlowAggregate> = order
        .into_iter()
        .filter_map(|key| flow_map.remove(&key))
        .collect();
    flows.sort_by(|left, right| right.latest_index.cmp(&left.latest_index));

    flows
        .into_iter()
        .take(max_flows)
        .map(|flow| {
            let title = flow_title(&flow);
            logs::composed::Flow {
                id: flow.id,
                detail_id: flow.detail_id,
                display_id: flow.display_id,
                title,
                latest_timestamp: flow.latest_timestamp,
                status: flow.status,
                events: flow.events,
            }
        })
        .collect()
}

struct FlowAggregate {
    id: Text,
    detail_id: Text,
    display_id: Text,
    latest_timestamp: Text,
    latest_index: usize,
    method: Option<Text>,
    path: Option<Text>,
    status: Option<Text>,
    events: Vec<logs::composed::FlowEvent>,
}

#[derive(Clone, Copy, Debug)]
enum FlowEventKind {
    RequestStart,
    RequestEnd,
    ChatIncoming,
    ChatBroadcast,
    Sse,
    Backend,
}

fn flow_event_kind(entry: &TraceEntry) -> Option<FlowEventKind> {
    let target_kind = LogTargetKind::parse(&entry.target.to_string());
    let message_kind = LogMessageKind::parse(&entry.message.to_string());
    match (target_kind, message_kind) {
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequest),
            LogMessageKind::Known(LogMessageKnown::RequestEnd),
        ) => Some(FlowEventKind::RequestEnd),
        (
            LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic),
            LogMessageKind::Known(LogMessageKnown::RequestStart),
        ) => Some(FlowEventKind::RequestStart),
        (
            LogTargetKind::Known(LogTargetKnown::DemoChat),
            LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming),
        ) => Some(FlowEventKind::ChatIncoming),
        (
            LogTargetKind::Known(LogTargetKnown::DemoSse),
            LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast),
        ) => Some(FlowEventKind::ChatBroadcast),
        (LogTargetKind::Known(LogTargetKnown::DemoSse), _) => Some(FlowEventKind::Sse),
        (LogTargetKind::Known(LogTargetKnown::DemoRequest), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoRequestDiagnostic), _)
        | (LogTargetKind::Known(LogTargetKnown::DemoChat), _) => {
            Some(FlowEventKind::Backend)
        }
        (LogTargetKind::Other(_), _)
            if field_text(entry, LogFieldKey::RequestId).is_some() =>
        {
            Some(FlowEventKind::Backend)
        }
        _ => None,
    }
}

fn hydrate_request_fields(
    aggregate: &mut FlowAggregate,
    entry: &TraceEntry,
    kind: FlowEventKind,
) {
    if matches!(
        kind,
        FlowEventKind::RequestEnd | FlowEventKind::RequestStart
    ) {
        if aggregate.method.is_none() {
            aggregate.method = field_text(entry, LogFieldKey::Method);
        }
        if aggregate.path.is_none() {
            aggregate.path = field_text(entry, LogFieldKey::Path);
        }
    }

    if aggregate.status.is_none() {
        aggregate.status = field_text(entry, LogFieldKey::Status);
    }
}

fn build_flow_event(kind: FlowEventKind, entry: &TraceEntry) -> logs::composed::FlowEvent {
    let (summary, stage_label, pills) = match kind {
        FlowEventKind::RequestEnd => {
            let method = field_text(entry, LogFieldKey::Method)
                .unwrap_or_else(|| Text::from("UNKNOWN"));
            let path =
                field_text(entry, LogFieldKey::Path).unwrap_or_else(|| Text::from("/"));
            let status =
                field_text(entry, LogFieldKey::Status).unwrap_or_else(|| Text::from("-"));

            let mut pills = vec![Pill::method(method.clone()), Pill::path(path.clone())];
            pills.push(Pill::status(status.clone()));
            if let Some(latency) = field_text(entry, LogFieldKey::LatencyMs) {
                pills.push(Pill::fields(format!("latency_ms={latency}")));
            }

            (
                Text::from(format!("HTTP {method} {path} -> {status}")),
                Text::from("request"),
                pills,
            )
        }
        FlowEventKind::RequestStart => {
            let method = field_text(entry, LogFieldKey::Method)
                .unwrap_or_else(|| Text::from("UNKNOWN"));
            let path =
                field_text(entry, LogFieldKey::Path).unwrap_or_else(|| Text::from("/"));
            (
                Text::from(format!("HTTP {method} {path} started")),
                Text::from("request"),
                vec![Pill::method(method), Pill::path(path)],
            )
        }
        FlowEventKind::ChatIncoming => {
            let sender = field_text(entry, LogFieldKey::Sender)
                .unwrap_or_else(|| Text::from("unknown"));
            let mut pills = vec![Pill::fields(format!("sender={sender}"))];
            if let Some(receiver) = field_text(entry, LogFieldKey::Receiver) {
                pills.push(Pill::fields(format!("receiver={receiver}")));
            }
            if let Some(user_id) = field_text(entry, LogFieldKey::UserId) {
                pills.push(Pill::fields(format!("user_id={user_id}")));
            }
            if let Some(payload_bytes) = field_text(entry, LogFieldKey::PayloadBytes) {
                pills.push(Pill::fields(format!("payload_bytes={payload_bytes}")));
            }

            (
                Text::from(format!("Backend accepted chat message from {sender}")),
                Text::from("backend"),
                pills,
            )
        }
        FlowEventKind::ChatBroadcast => {
            let selector = field_text(entry, LogFieldKey::Selector)
                .unwrap_or_else(|| Text::from("[unknown-selector]"));
            let mut pills = vec![Pill::fields(format!("selector={selector}"))];
            if let Some(mode) = field_text(entry, LogFieldKey::Mode) {
                pills.push(Pill::fields(format!("mode={mode}")));
            }
            if let Some(payload_bytes) = field_text(entry, LogFieldKey::PayloadBytes) {
                pills.push(Pill::fields(format!("payload_bytes={payload_bytes}")));
            }
            if let Some(sender) = field_text(entry, LogFieldKey::Sender) {
                pills.push(Pill::fields(format!("sender={sender}")));
            }
            if let Some(receiver) = field_text(entry, LogFieldKey::Receiver) {
                pills.push(Pill::fields(format!("receiver={receiver}")));
            }

            (
                Text::from(format!("SSE broadcast to {selector}")),
                Text::from("sse"),
                pills,
            )
        }
        FlowEventKind::Sse => (
            Text::from(format!("SSE event: {}", entry.message)),
            Text::from("sse"),
            field_pills(entry),
        ),
        FlowEventKind::Backend => (
            Text::from(format!("{}: {}", entry.target, entry.message)),
            Text::from("backend"),
            field_pills(entry),
        ),
    };

    logs::composed::FlowEvent {
        timestamp: Text::from(entry.timestamp.clone()),
        stage_label,
        summary,
        pills,
    }
}

fn field_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    if let Some(method) = field_text(entry, LogFieldKey::Method) {
        pills.push(Pill::method(method));
    }
    if let Some(path) = field_text(entry, LogFieldKey::Path) {
        pills.push(Pill::path(path));
    }
    if let Some(status) = field_text(entry, LogFieldKey::Status) {
        pills.push(Pill::status(status));
    }
    if let Some(latency) = field_text(entry, LogFieldKey::LatencyMs) {
        pills.push(Pill::fields(format!("latency_ms={latency}")));
    }
    if let Some(sender) = field_text(entry, LogFieldKey::Sender) {
        pills.push(Pill::fields(format!("sender={sender}")));
    }
    if let Some(receiver) = field_text(entry, LogFieldKey::Receiver) {
        pills.push(Pill::fields(format!("receiver={receiver}")));
    }
    if let Some(user_id) = field_text(entry, LogFieldKey::UserId) {
        pills.push(Pill::fields(format!("user_id={user_id}")));
    }
    if let Some(selector) = field_text(entry, LogFieldKey::Selector) {
        pills.push(Pill::fields(format!("selector={selector}")));
    }
    if let Some(mode) = field_text(entry, LogFieldKey::Mode) {
        pills.push(Pill::fields(format!("mode={mode}")));
    }
    if let Some(payload_bytes) = field_text(entry, LogFieldKey::PayloadBytes) {
        pills.push(Pill::fields(format!("payload_bytes={payload_bytes}")));
    }
    if pills.is_empty() {
        pills.push(Pill::target(entry.target.clone()));
    }
    pills
}

fn flow_title(flow: &FlowAggregate) -> Text {
    match (&flow.method, &flow.path) {
        (Some(method), Some(path)) => Text::from(format!("{method} {path}")),
        (Some(method), None) => Text::from(format!("{method} request")),
        _ => Text::from(format!("Flow {}", flow.display_id)),
    }
}

fn flow_id(entry: &TraceEntry, index: usize) -> Text {
    field_text(entry, LogFieldKey::RequestId).unwrap_or_else(|| {
        Text::from(format!(
            "orphan-{}-{index}",
            entry.timestamp.to_string().replace(':', "")
        ))
    })
}

fn slugify(value: &str) -> String {
    let mut slug = String::with_capacity(value.len());
    let mut last_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }
    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "flow".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(
        timestamp: &str,
        target: &str,
        message: &str,
        fields: Vec<(&str, &str)>,
    ) -> TraceEntry {
        TraceEntry::builder()
            .timestamp(TimestampText::new(timestamp))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new(target))
            .message(LogMessageText::new(message))
            .fields(
                fields
                    .into_iter()
                    .map(|(name, value)| {
                        (LogFieldName::new(name), LogFieldValue::new(value))
                    })
                    .collect(),
            )
            .build()
    }

    #[test]
    fn builds_request_flow_with_network_and_backend_events() {
        let entries = vec![
            entry(
                "12:00:00",
                "demo.request.diagnostic",
                "request.start",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                ],
            ),
            entry(
                "12:00:01",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                    ("status", "202"),
                    ("latency_ms", "8"),
                ],
            ),
            entry(
                "12:00:02",
                "app::chat::service",
                "moderation check passed",
                vec![("request_id", "req-aaa-111"), ("user_id", "user-1")],
            ),
            entry(
                "12:00:03",
                "demo.chat",
                "chat.message.incoming",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("sender", "you"),
                    ("receiver", "server"),
                    ("user_id", "user-1"),
                ],
            ),
            entry(
                "12:00:04",
                "demo.sse",
                "chat message broadcast",
                vec![
                    ("request_id", "req-aaa-111"),
                    ("selector", "[data-chat-messages]"),
                    ("mode", "prepend"),
                    ("payload_bytes", "240"),
                ],
            ),
            entry(
                "12:00:05",
                "demo.request",
                "request.end",
                vec![
                    ("request_id", "req-bbb-222"),
                    ("method", "GET"),
                    ("path", "/partials/request-burst-probe"),
                    ("status", "204"),
                ],
            ),
        ];

        let flows = request_flows(&entries, 20);

        assert_eq!(flows.len(), 2);
        assert_eq!(flows[0].id.to_string(), "req-bbb-222");
        assert_eq!(flows[1].id.to_string(), "req-aaa-111");
        assert_eq!(flows[1].events.len(), 5);
        assert!(
            flows[1]
                .events
                .iter()
                .any(|event| event.summary.to_string().contains("started"))
        );
        assert!(flows[1].events.iter().any(|event| {
            event
                .summary
                .to_string()
                .contains("moderation check passed")
        }));
    }

    #[test]
    fn includes_backend_entries_with_request_id_for_non_demo_targets() {
        let entries = vec![
            entry(
                "12:00:00",
                "app::auth::service",
                "session refreshed",
                vec![("request_id", "req-xyz-999"), ("status", "200")],
            ),
            entry(
                "12:00:01",
                "app::auth::service",
                "ignored without request id",
                vec![],
            ),
        ];

        let flows = request_flows(&entries, 20);

        assert_eq!(flows.len(), 1);
        assert_eq!(flows[0].id.to_string(), "req-xyz-999");
        assert_eq!(flows[0].events.len(), 1);
        assert!(
            flows[0].events[0]
                .summary
                .to_string()
                .contains("session refreshed")
        );
    }

    #[test]
    fn orphan_events_get_stable_non_colliding_flow_ids() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.chat",
                "chat.message.incoming",
                vec![("sender", "demo")],
            ),
            entry(
                "12:00:02",
                "demo.chat",
                "chat.message.incoming",
                vec![("sender", "you")],
            ),
        ];

        let flows = request_flows(&entries, 20);

        assert_eq!(flows.len(), 2);
        assert_ne!(flows[0].id.to_string(), flows[1].id.to_string());
        assert!(
            flows
                .iter()
                .all(|flow| flow.id.to_string().starts_with("orphan-"))
        );
    }
}
