use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, LogFieldValue, Text};
use crate::views::partials::components::Pill;
use crate::views::partials::demo::log;

pub(super) fn method_or_unknown(entry: &TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Method)
        .unwrap_or_else(|| Text::from("UNKNOWN"))
}

pub(super) fn path_or_root(entry: &TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Path).unwrap_or_else(|| Text::from("/"))
}

pub(super) fn status_or_dash(entry: &TraceEntry) -> Text {
    log::vm::field_text(entry, LogFieldKey::Status).unwrap_or_else(|| Text::from("-"))
}

pub(super) fn field_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();

    if let Some(method) = log::vm::field_text(entry, LogFieldKey::Method) {
        pills.push(Pill::method(method));
    }
    if let Some(path) = log::vm::field_text(entry, LogFieldKey::Path) {
        pills.push(Pill::path(path));
    }
    if let Some(status) = log::vm::field_text(entry, LogFieldKey::Status) {
        pills.push(Pill::status(status));
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
        pills.push(Pill::target(entry.target.clone()));
    }

    pills
}

pub(super) fn push_fields_as_pills(
    pills: &mut Vec<Pill>,
    entry: &TraceEntry,
    fields: &[(LogFieldKey, &'static str)],
) {
    for (key, name) in fields {
        if let Some(value) = log::vm::field_text(entry, key.clone()) {
            pills.push(Pill::fields(format!("{name}={value}")));
        }
    }
}

fn push_db_bind_pills(pills: &mut Vec<Pill>, entry: &TraceEntry) {
    for (index, value) in db_bind_values(entry) {
        pills.push(Pill::fields(format!("${index}={value}")));
    }
}

pub(super) fn db_bind_values(entry: &TraceEntry) -> Vec<(usize, Text)> {
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
