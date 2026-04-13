use super::*;

fn message() -> chat::Message {
    chat::Message::builder()
        .message_id(Text::from("abc123"))
        .author(Text::from("Demo Bot"))
        .timestamp(Text::from("2026-03-11 10:00"))
        .body(Text::from("hello"))
        .status(chat::Status::Visible)
        .build()
}

#[test]
fn renders_surface_contract() {
    let markup = Surface::builder()
        .room_id(Text::from("room-1"))
        .messages(vec![message()])
        .variant(Variant::Lab)
        .build()
        .render()
        .into_string();

    assert!(markup.contains("data-chat-surface"));
    assert!(markup.contains("data-chat-room-id=\"room-1\""));
    assert!(markup.contains("class=\"u-surface-card\""));
    assert!(markup.contains("data-chat-surface-variant=\"lab\""));
    assert!(markup.contains("chatDraftBody"));
    assert!(markup.contains("chatDemoDraftBody"));
    assert!(!markup.contains("\"roomId\""));
    assert!(!markup.contains("\"sseConnected\""));
    assert!(markup.contains("data-chat-connection-row"));
    assert_eq!(markup.matches("<div data-chat-panel>").count(), 2);
    assert!(markup.contains("/static/chat-demo.js"));
}

#[test]
fn demo_only_mode_renders_read_only_you_panel() {
    let markup = Surface::builder()
        .room_id(Text::from("room-1"))
        .messages(vec![message()])
        .mode(Mode::DemoOnly)
        .build()
        .render()
        .into_string();

    assert!(markup.contains("Read-only as you."));
    assert!(markup.contains("Send as demo"));
}
