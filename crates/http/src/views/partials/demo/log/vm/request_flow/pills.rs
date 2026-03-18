use crate::trace_log::store;
use crate::types::{LogFieldKey, LogFieldValue, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log;

pub(super) fn method_or_unknown(entry: &store::TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Method)
        .unwrap_or_else(|| Text::from("UNKNOWN"))
}

pub(super) fn path_or_root(entry: &store::TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Path).unwrap_or_else(|| Text::from("/"))
}

pub(super) fn status_or_dash(entry: &store::TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Status).unwrap_or_else(|| Text::from("-"))
}

pub(super) fn field_pills(entry: &store::TraceEntry) -> Vec<components::Pill> {
    let mut pills = Vec::new();

    if let Some(method) = log::vm::field_text(entry, LogFieldKey::Method) {
        pills.push(components::Pill::method(method));
    }
    if let Some(path) = log::vm::field_text(entry, LogFieldKey::Path) {
        pills.push(components::Pill::path(path));
    }
    if let Some(status) = log::vm::field_text(entry, LogFieldKey::Status) {
        pills.push(components::Pill::status(status));
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
        if let Some(value) = log::vm::field_text(entry, key.clone()) {
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
    let mut values: Vec<(usize, Text)> = entry
        .fields
        .iter()
        .filter_map(|(name, value)| {
            let key = name.to_string();
            let index = key
                .strip_prefix("db_bind_")
                .and_then(|suffix| suffix.parse::<usize>().ok())?;
            let value = match value {
                LogFieldValue::Text(text) => text.clone(),
                LogFieldValue::Missing => return None,
            };
            Some((index, value))
        })
        .collect();
    values.sort_by_key(|(index, _)| *index);
    values
}
