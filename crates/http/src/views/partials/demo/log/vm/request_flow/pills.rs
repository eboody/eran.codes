use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;

pub(super) fn method_or_unknown(entry: &store::TraceEntry) -> Text {
    entry
        .field_text(LogFieldKey::Method)
        .cloned()
        .unwrap_or_else(|| Text::from("UNKNOWN"))
}

pub(super) fn path_or_root(entry: &store::TraceEntry) -> Text {
    entry
        .field_text(LogFieldKey::Path)
        .cloned()
        .unwrap_or_else(|| Text::from("/"))
}

pub(super) fn status_or_dash(entry: &store::TraceEntry) -> Text {
    entry
        .field_text(LogFieldKey::Status)
        .cloned()
        .unwrap_or_else(|| Text::from("-"))
}

pub(super) fn field_pills(entry: &store::TraceEntry) -> Vec<components::Pill> {
    let mut pills = Vec::new();

    if let Some(method) = entry.field_text(LogFieldKey::Method) {
        pills.push(components::Pill::method(method.clone()));
    }
    if let Some(path) = entry.field_text(LogFieldKey::Path) {
        pills.push(components::Pill::path(path.clone()));
    }
    if let Some(status) = entry.field_text(LogFieldKey::Status) {
        pills.push(components::Pill::status(status.clone()));
    }

    push_fields_as_pills(
        &mut pills,
        entry,
        &[
            (LogFieldKey::LatencyMs, "latency_ms"),
            (LogFieldKey::Sender, "sender"),
            (LogFieldKey::Receiver, "receiver"),
            (LogFieldKey::UserId, "user_id"),
            (LogFieldKey::Selector, "selector"),
            (LogFieldKey::Mode, "mode"),
            (LogFieldKey::PayloadBytes, "payload_bytes"),
        ],
    );
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
        if let Some(value) = entry.field_text(*key) {
            pills.push(components::Pill::fields(format!("{name}={value}")));
        }
    }
}

fn push_db_bind_pills(pills: &mut Vec<components::Pill>, entry: &store::TraceEntry) {
    for (index, value) in db_bind_values(entry) {
        pills.push(components::Pill::fields(format!("${index}={value}")));
    }
}

pub(super) fn db_bind_values(entry: &store::TraceEntry) -> Vec<(usize, Text)> {
    entry.db_bind_values()
}
