use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};

pub fn field_text<'a>(
    entry: &'a store::TraceEntry,
    key: LogFieldKey,
) -> Option<&'a Text> {
    entry.field_text(key)
}

pub fn non_empty_field_text<'a>(
    entry: &'a store::TraceEntry,
    key: LogFieldKey,
) -> Option<&'a Text> {
    field_text(entry, key).filter(|value| !value.to_string().is_empty())
}

pub fn short_request_id(value: &Text) -> Text {
    crate::trace_log::demo_chat::short_hyphenated_text(value)
}
