use crate::trace_log::store;
use crate::types::{LogFieldKey, LogFieldName, LogFieldValue, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log::vm::entry_fields::{
    non_empty_field_text, short_request_id,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum GroupKey {
    Request(Text),
    Unknown,
}

impl GroupKey {
    pub(super) fn from_entry(entry: &store::TraceEntry) -> Self {
        match request_id(entry) {
            Some(request_id) => Self::Request(request_id.clone()),
            None => Self::Unknown,
        }
    }

    fn request_label(&self) -> Text {
        match self {
            Self::Request(request_id) => short_request_id(request_id),
            Self::Unknown => Text::from("unknown"),
        }
    }
}

pub(super) struct GroupDraft<'a> {
    key: GroupKey,
    entries: Vec<&'a store::TraceEntry>,
}

impl<'a> GroupDraft<'a> {
    pub(super) fn new(entry: &'a store::TraceEntry) -> Self {
        Self {
            key: GroupKey::from_entry(entry),
            entries: vec![entry],
        }
    }

    pub(super) fn push(&mut self, entry: &'a store::TraceEntry) {
        self.entries.push(entry);
    }

    pub(super) fn into_group(self) -> components::logs::composed::Group {
        let rows = self.entries.iter().map(|entry| event_row(entry)).collect();

        components::logs::composed::Group::builder()
            .request_pill(components::Pill::fields(format!(
                "request_id={}",
                self.key.request_label()
            )))
            .count_label(Text::from(format!("{} events", self.entries.len())))
            .rows(rows)
            .build()
    }
}

fn event_row(entry: &store::TraceEntry) -> components::logs::primitives::EventRow {
    components::logs::primitives::EventRow::builder()
        .timestamp(Text::from(entry.timestamp.clone()))
        .message(Text::from(entry.message.clone()))
        .pills(build_pills(entry))
        .build()
}

fn build_pills(entry: &store::TraceEntry) -> Vec<components::Pill> {
    let mut pills = vec![components::Pill::level(entry.level.clone())];

    if let Some(status) = non_empty_field_text(entry, LogFieldKey::Status) {
        pills.push(components::Pill::status(status.clone()));
    }
    if let Some(method) = non_empty_field_text(entry, LogFieldKey::Method) {
        pills.push(components::Pill::method(method.clone()));
    }
    if let Some(path) = non_empty_field_text(entry, LogFieldKey::Path) {
        pills.push(components::Pill::path(path.clone()));
    }

    pills.push(components::Pill::target(entry.target.clone()));
    pills.extend(compact_fields(entry));
    pills
}

fn compact_fields(entry: &store::TraceEntry) -> Vec<components::Pill> {
    entry
        .fields
        .iter()
        .filter_map(|(name, value)| public_support_pill(name, value))
        .take(2)
        .collect()
}

fn public_support_pill(name: &LogFieldName, value: &LogFieldValue) -> Option<components::Pill> {
    let text = match value {
        LogFieldValue::Text(text) if !text.to_string().is_empty() => text,
        _ => return None,
    };

    let rendered = match LogFieldKey::try_from(name).ok()? {
        LogFieldKey::LatencyMs => format!("latency_ms={text}"),
        LogFieldKey::Sender => format!("source={text}"),
        _ => return None,
    };

    Some(components::Pill::fields(rendered))
}

fn request_id(entry: &store::TraceEntry) -> Option<&Text> {
    non_empty_field_text(entry, LogFieldKey::RequestId)
}
