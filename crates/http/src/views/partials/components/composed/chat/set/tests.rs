use super::*;
use crate::types::Text;
use crate::views::partials;

fn panel() -> chat::Panel {
    chat::Panel::composer(
        chat::Window::builder()
            .with_title(Text::from("Demo"))
            .with_connected_signal(Text::from("$sseConnected"))
            .side(chat::Side::Left)
            .messages(Vec::new())
            .build(),
        chat::Composer::builder()
            .action(Text::from("/chat/messages/demo"))
            .input_label(Text::from("Message as demo user"))
            .input_name(Text::from("body"))
            .input_id(Text::from("chat-input-demo"))
            .input_signal(Text::from("chatDemoDraftBody"))
            .placeholder(Text::from("Send as demo user..."))
            .submit(
                partials::button::Button::builder()
                    .label(Text::from("Send as demo"))
                    .variant(partials::button::Variant::Secondary)
                    .role(partials::button::Role::submit())
                    .build(),
            )
            .build(),
    )
}

#[test]
fn renders_connection_and_multiple_panels() {
    let markup = Set::builder()
        .panels(vec![panel(), panel()])
        .with_connection(
            chat::Connection::builder()
                .connected_signal(Text::from("$sseConnected"))
                .build(),
        )
        .build()
        .render()
        .into_string();

    assert_eq!(markup.matches("<div data-chat-panel>").count(), 2);
    assert!(markup.contains("data-chat-connection-row"));
    assert!(markup.contains("data-chat-columns"));
}
