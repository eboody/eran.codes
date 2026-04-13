use super::*;
use crate::trace_log::store;
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, TimestampText,
};
use maud::Render;

fn entry(request_id: &str, message: &str) -> store::TraceEntry {
    store::TraceEntry::builder()
        .timestamp(TimestampText::new("12:00:00"))
        .level(LogLevelText::new("info"))
        .target(LogTargetText::new("demo.request"))
        .message(LogMessageText::new(message))
        .fields(vec![(
            LogFieldName::new("request_id"),
            LogFieldValue::new(request_id),
        )])
        .build()
}

#[test]
fn builds_grouped_feed_from_request_ids() {
    let entries = [entry("abc-1", "a"), entry("abc-1", "b"), entry("def-2", "c")];
    let markup = build_grouped_feed(entries.iter()).render().into_string();

    assert!(markup.contains("request_id=abc"));
    assert!(markup.contains("request_id=def"));
    assert!(markup.contains("2 events"));
    assert!(markup.contains("1 events"));
}

#[test]
fn grouped_feed_hides_private_context_fields_and_keeps_safe_operational_pills() {
    let entry = store::TraceEntry::builder()
        .timestamp(TimestampText::new("12:00:00"))
        .level(LogLevelText::new("info"))
        .target(LogTargetText::new("demo.request"))
        .message(LogMessageText::new("request.end"))
        .fields(vec![
            (
                LogFieldName::new("request_id"),
                LogFieldValue::new("req-abc-123"),
            ),
            (
                LogFieldName::new("session_id"),
                LogFieldValue::new("session-raw"),
            ),
            (LogFieldName::new("user_id"), LogFieldValue::new("user-raw")),
            (
                LogFieldName::new("sse_tab_id"),
                LogFieldValue::new("tab-raw"),
            ),
            (
                LogFieldName::new("latency_ms"),
                LogFieldValue::new("12"),
            ),
            (LogFieldName::new("sender"), LogFieldValue::new("lab")),
        ])
        .build();

    let markup = build_grouped_feed([&entry]).render().into_string();

    assert!(markup.contains("request_id=req"));
    assert!(markup.contains("latency_ms=12"));
    assert!(markup.contains("source=lab"));
    assert!(!markup.contains("session-raw"));
    assert!(!markup.contains("session_id="));
    assert!(!markup.contains("user-raw"));
    assert!(!markup.contains("user_id="));
    assert!(!markup.contains("tab-raw"));
    assert!(!markup.contains("sse_tab_id="));
}

#[test]
fn blank_and_missing_request_ids_share_unknown_group() {
    let blank = entry("   ", "blank");
    let missing = store::TraceEntry::builder()
        .timestamp(TimestampText::new("12:00:01"))
        .level(LogLevelText::new("info"))
        .target(LogTargetText::new("demo.request"))
        .message(LogMessageText::new("missing"))
        .fields(vec![])
        .build();

    let markup = build_grouped_feed([&blank, &missing]).render().into_string();

    assert!(markup.contains("request_id=unknown"));
    assert!(markup.contains("2 events"));
    assert_eq!(markup.matches("request_id=unknown").count(), 1);
}

#[test]
fn blank_support_fields_do_not_render_empty_operational_pills() {
    let entry = store::TraceEntry::builder()
        .timestamp(TimestampText::new("12:00:00"))
        .level(LogLevelText::new("info"))
        .target(LogTargetText::new("demo.request"))
        .message(LogMessageText::new("request.end"))
        .fields(vec![
            (
                LogFieldName::new("request_id"),
                LogFieldValue::new("req-abc-123"),
            ),
            (LogFieldName::new("status"), LogFieldValue::new(" ")),
            (LogFieldName::new("method"), LogFieldValue::new("")),
            (LogFieldName::new("path"), LogFieldValue::new(" ")),
            (LogFieldName::new("latency_ms"), LogFieldValue::new("")),
            (LogFieldName::new("sender"), LogFieldValue::new(" ")),
        ])
        .build();

    let markup = build_grouped_feed([&entry]).render().into_string();

    assert!(!markup.contains("source="));
    assert!(!markup.contains("latency_ms="));
    assert!(!markup.contains("ui-pill--status"));
    assert!(!markup.contains("ui-pill--method"));
    assert!(!markup.contains("ui-pill--path"));
}
