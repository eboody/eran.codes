use super::*;

#[test]
fn renders_shared_chat_hero_shell() {
    let markup = Hero::builder()
        .room_name(Text::from("Lobby"))
        .room_id(Text::from("room-1"))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("class=\"u-surface-card\""));
    assert!(markup.contains("data-chat-hero"));
    assert!(markup.contains("<h1>Live chat room</h1>"));
    assert!(markup.contains("Room id: room-1"));
    assert!(markup.contains("Moderation queue"));
}
