use super::*;

#[test]
fn renders_right_sided_window_with_live_offline_state() {
    let markup = Window::builder()
        .with_title(Text::from("You"))
        .with_connected_signal(Text::from("$sseConnected"))
        .side(chat::Side::Right)
        .messages(Vec::new())
        .build()
        .render()
        .into_string();

    assert!(markup.contains("data-chat-side=\"right\""));
    assert!(markup.contains("data-chat-room-state=\"live\""));
    assert!(markup.contains("data-show=\"$sseConnected\""));
    assert!(markup.contains("data-show=\"!$sseConnected\""));
}
