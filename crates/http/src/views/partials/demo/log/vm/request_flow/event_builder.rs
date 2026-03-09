use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::{Pill, logs};
use crate::views::partials::demo::log;

pub(super) fn build_flow_event(
    kind: log::vm::request_flow::kind::FlowEventKind,
    entry: &TraceEntry,
) -> logs::composed::FlowEvent {
    let (summary, stage_label, pills) = match kind {
        log::vm::request_flow::kind::FlowEventKind::RequestEnd => {
            request_end_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::RequestStart => {
            request_start_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatIncoming => {
            chat_incoming_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::ChatBroadcast => {
            chat_broadcast_event(entry)
        }
        log::vm::request_flow::kind::FlowEventKind::Sse => (
            Text::from(format!("SSE event: {}", entry.message)),
            Text::from("sse"),
            log::vm::request_flow::pills::field_pills(entry),
        ),
        log::vm::request_flow::kind::FlowEventKind::Backend => backend_event(entry),
    };

    logs::composed::FlowEvent {
        timestamp: Text::from(entry.timestamp.clone()),
        stage_label,
        summary,
        pills,
    }
}

fn backend_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let summary = db_backend_summary(entry)
        .unwrap_or_else(|| Text::from(format!("{}: {}", entry.target, entry.message)));

    (
        summary,
        Text::from("backend"),
        log::vm::request_flow::pills::field_pills(entry),
    )
}

fn db_backend_summary(entry: &TraceEntry) -> Option<Text> {
    if entry.target.to_string() != "demo.db" {
        return None;
    }

    let label = match unquote(entry.message.to_string().as_str()) {
        "db query" => "DB query",
        "db query complete" => "DB query complete",
        _ => return None,
    };

    let statement = log::vm::field_text(entry, LogFieldKey::DbStatement)
        .map(|value| normalize_whitespace(&value.to_string()))
        .unwrap_or_default();

    if statement.is_empty() {
        return Some(Text::from(label));
    }

    Some(Text::from(format!("{label}: {statement}")))
}

fn unquote(value: &str) -> &str {
    let trimmed = value.trim();
    trimmed
        .strip_prefix('"')
        .and_then(|candidate| candidate.strip_suffix('"'))
        .unwrap_or(trimmed)
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn request_end_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);
    let status = log::vm::request_flow::pills::status_or_dash(entry);

    let mut pills = vec![
        Pill::method(method.clone()),
        Pill::path(path.clone()),
        Pill::status(status.clone()),
    ];
    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[(LogFieldKey::LatencyMs, "latency_ms")],
    );

    (
        Text::from(format!("HTTP {method} {path} -> {status}")),
        Text::from("request"),
        pills,
    )
}

fn request_start_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let method = log::vm::request_flow::pills::method_or_unknown(entry);
    let path = log::vm::request_flow::pills::path_or_root(entry);

    (
        Text::from(format!("HTTP {method} {path} started")),
        Text::from("request"),
        vec![Pill::method(method), Pill::path(path)],
    )
}

fn chat_incoming_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let sender = log::vm::field_text(entry, LogFieldKey::Sender)
        .unwrap_or_else(|| Text::from("unknown"));
    let mut pills = vec![Pill::fields(format!("sender={sender}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Receiver, "receiver"),
            (LogFieldKey::UserId, "user_id"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
        ],
    );

    (
        Text::from(format!("Backend accepted chat message from {sender}")),
        Text::from("backend"),
        pills,
    )
}

fn chat_broadcast_event(entry: &TraceEntry) -> (Text, Text, Vec<Pill>) {
    let selector = log::vm::field_text(entry, LogFieldKey::Selector)
        .unwrap_or_else(|| Text::from("[unknown-selector]"));
    let mut pills = vec![Pill::fields(format!("selector={selector}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Mode, "mode"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
            (LogFieldKey::Sender, "sender"),
            (LogFieldKey::Receiver, "receiver"),
        ],
    );

    (
        Text::from(format!("SSE broadcast to {selector}")),
        Text::from("sse"),
        pills,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(
        target: &str,
        message: &str,
        fields: Vec<(&str, &str)>,
    ) -> TraceEntry {
        TraceEntry::builder()
            .timestamp(TimestampText::new("2026-03-08 22:15:27"))
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
    fn backend_demo_db_event_summary_includes_normalized_statement() {
        let event = build_flow_event(
            log::vm::request_flow::kind::FlowEventKind::Backend,
            &entry(
                "demo.db",
                "\"db query\"",
                vec![
                    (
                        "db_statement",
                        "SELECT id, room_id\nFROM chat_rooms\nWHERE id = $1",
                    ),
                    ("db_bind_1", "2e44a2af"),
                ],
            ),
        );

        assert_eq!(
            event.summary.to_string(),
            "DB query: SELECT id, room_id FROM chat_rooms WHERE id = $1"
        );
        assert!(event
            .pills
            .iter()
            .any(|pill| pill.text.to_string() == "$1=2e44a2af"));
    }

    #[test]
    fn backend_demo_db_event_summary_preserves_multiple_placeholders() {
        let event = build_flow_event(
            log::vm::request_flow::kind::FlowEventKind::Backend,
            &entry(
                "demo.db",
                "\"db query\"",
                vec![
                    (
                        "db_statement",
                        "SELECT id FROM chat_rooms WHERE id = $1 AND created_by = $2",
                    ),
                    ("db_bind_1", "room-1"),
                    ("db_bind_2", "owner-1"),
                ],
            ),
        );

        assert_eq!(
            event.summary.to_string(),
            "DB query: SELECT id FROM chat_rooms WHERE id = $1 AND created_by = $2"
        );
    }

    #[test]
    fn backend_non_db_event_summary_uses_default_format() {
        let event = build_flow_event(
            log::vm::request_flow::kind::FlowEventKind::Backend,
            &entry("app::chat::service", "moderation check passed", vec![]),
        );

        assert_eq!(
            event.summary.to_string(),
            "app::chat::service: moderation check passed"
        );
    }
}
