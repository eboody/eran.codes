use super::*;

#[test]
fn renders_pending_status_and_message_id() {
    let markup = Message::builder()
        .message_id(Text::from("abc123"))
        .author(Text::from("Demo Bot"))
        .timestamp(Text::from("2026-03-11 10:00"))
        .body(Text::from("hello"))
        .status(Status::Pending)
        .build()
        .render()
        .into_string();

    assert!(markup.contains("id=\"chat-message-abc123\""));
    assert!(markup.contains("data-chat-status-kind=\"pending\""));
    assert!(markup.contains(">pending<"));
}
