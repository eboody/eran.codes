use super::build_flow_event;
use crate::trace_log::TraceEntry;
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, TimestampText,
};
use crate::views::partials::demo::log;

fn entry(target: &str, message: &str, fields: Vec<(&str, &str)>) -> TraceEntry {
    TraceEntry::builder()
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
        .any(|pill| pill.text.to_string() == "$1=2e44a2af"));
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
