use super::*;

fn trace_entry(message: &str) -> TraceEntry {
    TraceEntry::builder()
        .timestamp(TimestampText::new("2026-03-25 00:00:00"))
        .level(LogLevelText::new("INFO"))
        .target(LogTargetText::new("demo.request"))
        .message(LogMessageText::new(message))
        .fields(Vec::new())
        .build()
}

#[test]
fn prunes_oldest_request_and_session_keys_at_store_bound() {
    let store = Store::builder()
        .with_sse(crate::sse::Registry::new())
        .with_max_entries(2)
        .with_emit_sse(false)
        .build();
    let request_a = RequestId::new("req-a");
    let request_b = RequestId::new("req-b");
    let request_c = RequestId::new("req-c");
    let session_a = SessionId::new("session-a");
    let session_b = SessionId::new("session-b");
    let session_c = SessionId::new("session-c");

    store.record_with_session(&request_a, Some(&session_a), trace_entry("request-a"));
    store.record_with_session(&request_b, Some(&session_b), trace_entry("request-b"));
    store.record_with_session(&request_c, Some(&session_c), trace_entry("request-c"));

    assert!(store.snapshot_request(&request_a).is_empty());
    assert_eq!(store.snapshot_request(&request_b).len(), 1);
    assert_eq!(store.snapshot_request(&request_c).len(), 1);

    assert!(store.snapshot_session(&session_a).is_empty());
    assert_eq!(store.snapshot_session(&session_b).len(), 1);
    assert_eq!(store.snapshot_session(&session_c).len(), 1);

    let global = store.snapshot_global();
    assert_eq!(global.len(), 2);
    assert_eq!(global[0].message.to_string(), "request-b");
    assert_eq!(global[1].message.to_string(), "request-c");
}

#[test]
fn record_sse_event_prunes_oldest_session_keys() {
    let store = Store::builder()
        .with_sse(crate::sse::Registry::new())
        .with_max_entries(2)
        .with_emit_sse(false)
        .build();
    let session_a = SessionId::new("session-a");
    let session_b = SessionId::new("session-b");
    let session_c = SessionId::new("session-c");

    store.record_sse_event(Some(&session_a), trace_entry("sse-a"));
    store.record_sse_event(Some(&session_b), trace_entry("sse-b"));
    store.record_sse_event(Some(&session_c), trace_entry("sse-c"));

    assert!(store.snapshot_session(&session_a).is_empty());
    assert_eq!(store.snapshot_session(&session_b).len(), 1);
    assert_eq!(store.snapshot_session(&session_c).len(), 1);
}
