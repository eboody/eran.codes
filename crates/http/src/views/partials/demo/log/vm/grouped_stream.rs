use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;

use super::short_request_id;

pub fn build_grouped_feed<'a, I>(entries: I) -> components::logs::composed::GroupedFeed
where
    I: IntoIterator<Item = &'a store::TraceEntry>,
{
    let groups = group_by_request(entries)
        .into_iter()
        .map(|group| {
            let request_label = group
                .request_id
                .as_ref()
                .map(short_request_id)
                .unwrap_or_else(|| Text::from("unknown"));
            let rows = group
                .entries
                .iter()
                .map(|entry| {
                    components::logs::primitives::EventRow::builder()
                        .timestamp(Text::from(entry.timestamp.clone()))
                        .message(Text::from(entry.message.clone()))
                        .pills(build_pills(entry))
                        .build()
                })
                .collect();
            components::logs::composed::Group::builder()
                .request_pill(components::Pill::fields(format!("request_id={}", request_label)))
                .count_label(Text::from(format!("{} events", group.entries.len())))
                .rows(rows)
                .build()
        })
        .collect();

    components::logs::composed::GroupedFeed::builder()
        .children(groups)
        .build()
}

struct LogGroup<'a> {
    request_id: Option<Text>,
    entries: Vec<&'a store::TraceEntry>,
}

fn group_by_request<'a, I>(entries: I) -> Vec<LogGroup<'a>>
where
    I: IntoIterator<Item = &'a store::TraceEntry>,
{
    let mut groups: Vec<LogGroup<'a>> = Vec::new();
    let mut order: Vec<Option<Text>> = Vec::new();
    let mut map: std::collections::HashMap<Option<Text>, Vec<&'a store::TraceEntry>> =
        std::collections::HashMap::new();

    for entry in entries {
        let request_id = entry.field_text(LogFieldKey::RequestId).cloned();
        if !map.contains_key(&request_id) {
            order.push(request_id.clone());
        }
        map.entry(request_id).or_default().push(entry);
    }

    for key in order {
        if let Some(entries) = map.remove(&key) {
            groups.push(LogGroup {
                request_id: key,
                entries,
            });
        }
    }

    groups
}

fn build_pills(entry: &store::TraceEntry) -> Vec<components::Pill> {
    let mut pills = Vec::new();
    pills.push(components::Pill::level(entry.level.clone()));
    if let Some(status) = entry.field_text(LogFieldKey::Status) {
        pills.push(components::Pill::status(status.clone()));
    }
    if let Some(method) = entry.field_text(LogFieldKey::Method) {
        pills.push(components::Pill::method(method.clone()));
    }
    if let Some(path) = entry.field_text(LogFieldKey::Path) {
        pills.push(components::Pill::path(path.clone()));
    }
    pills.push(components::Pill::target(entry.target.clone()));
    pills.extend(compact_fields(entry));
    pills
}

fn compact_fields(entry: &store::TraceEntry) -> Vec<components::Pill> {
    if entry.fields.is_empty() {
        return Vec::new();
    }

    entry
        .fields
        .iter()
        .filter_map(|(name, value)| {
            let field_kind = LogFieldKey::try_from(name).ok();
            if matches!(
                field_kind,
                Some(LogFieldKey::Method | LogFieldKey::Path | LogFieldKey::Status)
            ) {
                return None;
            }
            let value = Text::from(value.to_string());
            Some(components::Pill::fields(format!("{}={}", name, value)))
        })
        .take(2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace_log::store;
    use crate::types::{LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, TimestampText};
    use maud::Render;

    fn entry(request_id: &str, message: &str) -> store::TraceEntry {
        store::TraceEntry::builder()
            .timestamp(TimestampText::new("12:00:00"))
            .level(LogLevelText::new("info"))
            .target(LogTargetText::new("demo.request"))
            .message(LogMessageText::new(message))
            .fields(vec![(
                LogFieldName::new("request_id"),
                LogFieldValue::new(request_id),
            )])
            .build()
    }

    #[test]
    fn builds_grouped_feed_from_request_ids() {
        let entries = [entry("abc-1", "a"), entry("abc-1", "b"), entry("def-2", "c")];
        let markup = build_grouped_feed(entries.iter()).render().into_string();

        assert!(markup.contains("request_id=abc"));
        assert!(markup.contains("request_id=def"));
        assert!(markup.contains("2 events"));
        assert!(markup.contains("1 events"));
    }
}
