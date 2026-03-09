use super::request_flows;

use crate::trace_log::TraceEntry;
use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
    SseTabId, TimestampText,
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

    let flows = request_flows(&entries, 20, None);

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
fn skips_backend_only_entries_when_request_envelope_is_missing() {
    let entries = vec![
        entry(
            "12:00:00",
            "app::auth::service",
            "session refreshed",
            vec![("request_id", "2e44a2af"), ("status", "200")],
        ),
        entry(
            "12:00:01",
            "app::auth::service",
            "ignored without request id",
            vec![],
        ),
    ];

    let flows = request_flows(&entries, 20, None);

    assert!(flows.is_empty());
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

    let flows = request_flows(&entries, 20, None);

    assert_eq!(flows.len(), 2);
    assert_ne!(flows[0].id.to_string(), flows[1].id.to_string());
    assert!(
        flows
            .iter()
            .all(|flow| flow.id.to_string().starts_with("orphan-"))
    );
    assert!(
        flows
            .iter()
            .all(|flow| flow.title.to_string() == "Request (orphan)")
    );
}

#[test]
fn filters_request_flows_to_active_tab_id() {
    let entries = vec![
        entry(
            "12:00:00",
            "demo.request",
            "request.end",
            vec![
                ("request_id", "req-tab-a"),
                ("method", "POST"),
                ("path", "/demo/chat/messages"),
                ("status", "202"),
                ("sse_tab_id", "tab-a"),
            ],
        ),
        entry(
            "12:00:01",
            "demo.request",
            "request.end",
            vec![
                ("request_id", "req-tab-b"),
                ("method", "POST"),
                ("path", "/demo/chat/messages/demo"),
                ("status", "202"),
                ("sse_tab_id", "tab-b"),
            ],
        ),
    ];

    let flows = request_flows(&entries, 20, Some(&SseTabId::new("tab-a")));

    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].id.to_string(), "req-tab-a");
}
