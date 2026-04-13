use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log;
use crate::views::partials::demo::log::vm::entry_fields::non_empty_field_text;

pub(super) fn method_or_unknown(entry: &store::TraceEntry) -> Text {
    non_empty_field_text(entry, LogFieldKey::Method)
        .cloned()
        .unwrap_or_else(|| Text::from("UNKNOWN"))
}

pub(super) fn path_or_root(entry: &store::TraceEntry) -> Text {
    non_empty_field_text(entry, LogFieldKey::Path)
        .cloned()
        .unwrap_or_else(|| Text::from("/"))
}

pub(super) fn status_or_dash(entry: &store::TraceEntry) -> Text {
    non_empty_field_text(entry, LogFieldKey::Status)
        .cloned()
        .unwrap_or_else(|| Text::from("-"))
}

pub(super) fn field_pills(entry: &store::TraceEntry) -> Vec<components::Pill> {
    let mut pills = Vec::new();

    if let Some(method) = non_empty_field_text(entry, LogFieldKey::Method) {
        pills.push(components::Pill::method(method.clone()));
    }
    if let Some(path) = non_empty_field_text(entry, LogFieldKey::Path) {
        pills.push(components::Pill::path(path.clone()));
    }
    if let Some(status) = non_empty_field_text(entry, LogFieldKey::Status) {
        pills.push(components::Pill::status(status.clone()));
    }

    push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::LatencyMs, "latency_ms"),
            (LogFieldKey::Sender, "sender"),
            (LogFieldKey::Receiver, "receiver"),
            (LogFieldKey::Selector, "selector"),
            (LogFieldKey::Mode, "mode"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
        ],
    );
    if non_empty_field_text(entry, LogFieldKey::UserId).is_some() {
        pills.push(log::vm::redaction::authenticated_user_pill());
    }
    push_db_bind_pills(&mut pills, entry);

    if pills.is_empty() {
        pills.push(components::Pill::target(entry.target.clone()));
    }

    pills
}

pub(super) fn push_fields_as_pills(
    pills: &mut Vec<components::Pill>,
    entry: &store::TraceEntry,
    fields: &[(LogFieldKey, &'static str)],
) {
    for (key, name) in fields {
        if let Some(value) = non_empty_field_text(entry, *key) {
            pills.push(components::Pill::fields(format!("{name}={value}")));
        }
    }
}

fn push_db_bind_pills(pills: &mut Vec<components::Pill>, entry: &store::TraceEntry) {
    for (index, _) in db_bind_values(entry) {
        pills.push(log::vm::redaction::redacted_bind_pill(index));
    }
}

pub(super) fn db_bind_values(entry: &store::TraceEntry) -> Vec<(usize, Text)> {
    entry.db_bind_values()
}
