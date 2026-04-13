use super::build_flow_event;
use crate::trace_log::store;
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, TimestampText,
};
use crate::views::partials::demo::log;

fn entry(target: &str, message: &str, fields: Vec<(&str, &str)>) -> store::TraceEntry {
    store::TraceEntry::builder()
        .timestamp(TimestampText::new("2026-03-08 22:15:27"))
        .level(LogLevelText::new("INFO"))
        .target(LogTargetText::new(target))
        .message(LogMessageText::new(message))
        .fields(
            fields
                .into_iter()
                .map(|(name, value)| (LogFieldName::new(name), LogFieldValue::new(value)))
                .collect(),
        )
        .build()
}

#[test]
fn backend_demo_db_event_summary_includes_normalized_statement() {
    let event = build_flow_event(
        log::vm::request_flow::kind::FlowEvent::Backend,
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
        .any(|pill| pill.text.to_string() == "$1=(redacted)"));
}

#[test]
fn backend_demo_db_event_summary_preserves_multiple_placeholders() {
    let event = build_flow_event(
        log::vm::request_flow::kind::FlowEvent::Backend,
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
        log::vm::request_flow::kind::FlowEvent::Backend,
        &entry("app::chat::service", "moderation check passed", vec![]),
    );

    assert_eq!(
        event.summary.to_string(),
        "app::chat::service: moderation check passed"
    );
}

#[test]
fn chat_events_treat_blank_fields_as_missing() {
    let incoming = build_flow_event(
        log::vm::request_flow::kind::FlowEvent::ChatIncoming,
        &entry(
            "demo.chat",
            "chat.message.incoming",
            vec![
                ("sender", "   "),
                ("receiver", ""),
                ("payload_bytes", " "),
                ("user_id", " "),
            ],
        ),
    );

    assert_eq!(
        incoming.summary.to_string(),
        "Backend accepted chat message from unknown"
    );
    let incoming_pills = incoming
        .pills
        .iter()
        .map(|pill| pill.text.to_string())
        .collect::<Vec<_>>();
    assert_eq!(incoming_pills, vec!["sender=unknown"]);

    let broadcast = build_flow_event(
        log::vm::request_flow::kind::FlowEvent::ChatBroadcast,
        &entry(
            "demo.sse",
            "chat message broadcast",
            vec![
                ("selector", " "),
                ("mode", ""),
                ("payload_bytes", " "),
                ("sender", ""),
                ("receiver", " "),
            ],
        ),
    );

    assert_eq!(
        broadcast.summary.to_string(),
        "SSE broadcast to [unknown-selector]"
    );
    let broadcast_pills = broadcast
        .pills
        .iter()
        .map(|pill| pill.text.to_string())
        .collect::<Vec<_>>();
    assert_eq!(broadcast_pills, vec!["selector=[unknown-selector]"]);
}
