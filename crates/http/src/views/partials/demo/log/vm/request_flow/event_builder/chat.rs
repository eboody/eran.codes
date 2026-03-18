use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log;

pub(super) fn chat_incoming_event(
    entry: &store::TraceEntry,
) -> (Text, Text, Vec<components::Pill>) {
    let sender = log::vm::field_text(entry, LogFieldKey::Sender)
        .unwrap_or_else(|| Text::from("unknown"));
    let mut pills = vec![components::Pill::fields(format!("sender={sender}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Receiver, "receiver"),
            (LogFieldKey::UserId, "user_id"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
        ],
    );

    (
        Text::from(format!("Backend accepted chat message from {sender}")),
        Text::from("backend"),
        pills,
    )
}

pub(super) fn chat_broadcast_event(
    entry: &store::TraceEntry,
) -> (Text, Text, Vec<components::Pill>) {
    let selector = log::vm::field_text(entry, LogFieldKey::Selector)
        .unwrap_or_else(|| Text::from("[unknown-selector]"));
    let mut pills = vec![components::Pill::fields(format!("selector={selector}"))];

    log::vm::request_flow::pills::push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::Mode, "mode"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
            (LogFieldKey::Sender, "sender"),
            (LogFieldKey::Receiver, "receiver"),
        ],
    );

    (
        Text::from(format!("SSE broadcast to {selector}")),
        Text::from("sse"),
        pills,
    )
}
