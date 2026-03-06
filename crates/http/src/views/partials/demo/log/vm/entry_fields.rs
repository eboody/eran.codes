use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, LogFieldName, LogFieldValue, Text};

pub fn field_text(entry: &TraceEntry, key: LogFieldKey) -> Option<Text> {
    let name = LogFieldName::from(key);
    entry
        .fields
        .iter()
        .find(|(field, _)| field == &name)
        .and_then(|(_, value)| match value {
            LogFieldValue::Text(text) => Some(text.clone()),
            LogFieldValue::Missing => None,
        })
}

pub fn short_request_id(value: &Text) -> Text {
    let value = value.to_string();
    let short = value.split('-').next().unwrap_or(value.as_str()).to_string();
    Text::from(short)
}
