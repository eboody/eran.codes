use crate::types::SseTabId;
use crate::views::partials::demo::log;

#[test]
fn filters_request_flows_to_active_tab_id() {
    let entries = vec![
        super::support::request_end(
            "12:00:00",
            "req-tab-a",
            "POST",
            "/demo/chat/messages",
            vec![("status", "202"), ("sse_tab_id", "tab-a")],
        ),
        super::support::request_end(
            "12:00:01",
            "req-tab-b",
            "POST",
            "/demo/chat/messages/demo",
            vec![("status", "202"), ("sse_tab_id", "tab-b")],
        ),
    ];

    let flows = log::vm::request_flows(&entries, 20, Some(&SseTabId::new("tab-a")));

    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].id.to_string(), "req-tab-a");
}

#[test]
fn filters_request_start_only_flows_to_active_tab_id() {
    let entries = vec![
        super::support::request_start(
            "12:00:00",
            "req-tab-a",
            "GET",
            "/partials/sensitive-proof",
            vec![("sse_tab_id", "tab-a")],
        ),
        super::support::request_start(
            "12:00:01",
            "req-tab-b",
            "GET",
            "/partials/sensitive-proof",
            vec![("sse_tab_id", "tab-b")],
        ),
    ];

    let flows = log::vm::request_flows(&entries, 20, Some(&SseTabId::new("tab-a")));

    assert_eq!(flows.len(), 1);
    assert_eq!(flows[0].id.to_string(), "req-tab-a");
    assert_eq!(flows[0].title.to_string(), "GET /partials/sensitive-proof");
}
