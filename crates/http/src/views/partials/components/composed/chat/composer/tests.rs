use super::*;

#[test]
fn submit_action_filters_to_draft_signal_and_sse_tab() {
    let composer = Composer::builder()
        .action(Text::from("/demo/chat/messages"))
        .input_label(Text::from("Message as you"))
        .input_name(Text::from("body"))
        .input_id(Text::from("chat-input-you"))
        .input_signal(Text::from("chatDraftBody"))
        .placeholder(Text::from("Say something..."))
        .submit(
            partials::button::Button::builder()
                .label(Text::from("Send"))
                .variant(partials::button::Variant::Primary)
                .role(partials::button::Role::submit())
                .build(),
        )
        .build();

    let markup = composer.render().into_string();

    assert!(
        markup.contains(
            "@post('/demo/chat/messages', {filterSignals: {include: /^(?:chatDraftBody|sseTabId)$/}})"
        )
    );
    assert!(markup.contains("$chatDraftBody = ''"));
}
