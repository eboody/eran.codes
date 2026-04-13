use crate::views::partials::demo::log;

#[test]
fn builds_request_flow_with_network_and_backend_events() {
    let entries = vec![
        super::support::request_start(
            "12:00:00",
            "req-aaa-111",
            "POST",
            "/demo/chat/messages",
            Vec::new(),
        ),
        super::support::request_end(
            "12:00:01",
            "req-aaa-111",
            "POST",
            "/demo/chat/messages",
            vec![("status", "202"), ("latency_ms", "8")],
        ),
        super::support::entry(
            "12:00:02",
            "app::chat::service",
            "moderation check passed",
            vec![("request_id", "req-aaa-111"), ("user_id", "user-1")],
        ),
        super::support::entry(
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
        super::support::entry(
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
        super::support::request_end(
            "12:00:05",
            "req-bbb-222",
            "GET",
            "/partials/request-burst-probe",
            vec![("status", "204")],
        ),
    ];

    let flows = log::vm::request_flows(&entries, 20, None);

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
        super::support::entry(
            "12:00:00",
            "app::auth::service",
            "session refreshed",
            vec![("request_id", "2e44a2af"), ("status", "200")],
        ),
        super::support::entry(
            "12:00:01",
            "app::auth::service",
            "ignored without request id",
            vec![],
        ),
    ];

    let flows = log::vm::request_flows(&entries, 20, None);

    assert!(flows.is_empty());
}

#[test]
fn orphan_events_get_stable_non_colliding_flow_ids() {
    let entries = vec![
        super::support::entry(
            "12:00:01",
            "demo.chat",
            "chat.message.incoming",
            vec![("sender", "demo")],
        ),
        super::support::entry(
            "12:00:02",
            "demo.chat",
            "chat.message.incoming",
            vec![("sender", "you")],
        ),
    ];

    let flows = log::vm::request_flows(&entries, 20, None);

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
fn blank_request_id_falls_back_to_orphan_identity() {
    let entries = vec![super::support::request_end(
        "12:00:01",
        "   ",
        "POST",
        "/demo/chat/messages",
        vec![("status", "202")],
    )];

    let flows = log::vm::request_flows(&entries, 20, None);

    assert_eq!(flows.len(), 1);
    assert!(flows[0].id.to_string().starts_with("orphan-"));
    assert_eq!(flows[0].display_id.to_string(), "orphan");
    assert_eq!(flows[0].title.to_string(), "POST /demo/chat/messages");
}

#[test]
fn blank_request_id_does_not_promote_other_targets_to_backend_flows() {
    let entries = vec![super::support::entry(
        "12:00:00",
        "app::auth::service",
        "session refreshed",
        vec![("request_id", " "), ("status", "200")],
    )];

    let flows = log::vm::request_flows(&entries, 20, None);

    assert!(flows.is_empty());
}
