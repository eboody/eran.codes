use std::str::FromStr;

use bon::Builder;
use maud::Render;
use strum_macros::{Display, EnumString};

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldName, Text};
use crate::views::partials::components::{
    EmptyState, FieldValue, LogPanel, LogRow, Pill,
};

#[derive(Builder)]
pub struct TraceLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for TraceLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            EmptyState::builder()
                .message(Text::from("No trace entries recorded yet."))
                .build()
                .render()
        } else {
            let grouped = group_by_request(self.entries.iter());
            maud::html! {
                div class="log-groups" {
                    @for group in grouped {
                        div class="log-group" {
                            div class="log-group-header" {
                                @if let Some(request_id) = &group.request_id {
                                    (Pill::fields(format!("request_id={}", short_request_id(request_id))).render())
                                } @else {
                                    (Pill::fields("request_id=unknown").render())
                                }
                                span class="muted" { (format!("{} events", group.entries.len())) }
                            }
                            ul class="live-log-entries" {
                                @for entry in group.entries {
                                    (LogRow::builder()
                                        .timestamp(Text::from(entry.timestamp.to_string()))
                                        .message(Text::from(entry.message.to_string()))
                                        .pills(build_pills(entry))
                                        .build()
                                        .render())
                                }
                            }
                        }
                    }
                }
            }
        };

        maud::html! {
            (LogPanel::builder()
                .title(Text::from("Trace log"))
                .body(body)
                .build()
                .render())
        }
    }
}

struct LogGroup<'a> {
    request_id: Option<Text>,
    entries: Vec<&'a TraceEntry>,
}

fn group_by_request<'a, I>(entries: I) -> Vec<LogGroup<'a>>
where
    I: IntoIterator<Item = &'a TraceEntry>,
{
    let mut groups: Vec<LogGroup<'a>> = Vec::new();
    let mut order: Vec<Option<Text>> = Vec::new();
    let mut map: std::collections::HashMap<Option<Text>, Vec<&'a TraceEntry>> =
        std::collections::HashMap::new();
    for entry in entries {
        let request_id = field_value(entry, &LogFieldName::from("request_id"));
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

fn short_request_id(value: &Text) -> String {
    let value = value.to_string();
    value.split('-').next().unwrap_or(value.as_str()).to_string()
}

fn build_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(Pill::level(entry.level.to_string()));
    if let Some(status) = field_value(entry, &LogFieldName::from("status")) {
        pills.push(Pill::status(status.clone()));
    }
    if let Some(method) = field_value(entry, &LogFieldName::from("method")) {
        pills.push(Pill::method(method.clone()));
    }
    if let Some(path) = field_value(entry, &LogFieldName::from("path")) {
        pills.push(Pill::path(path));
    }
    pills.push(Pill::target(entry.target.to_string()));
    pills.extend(compact_fields(entry));
    pills
}

fn compact_fields(entry: &TraceEntry) -> Vec<Pill> {
    if entry.fields.is_empty() {
        return Vec::new();
    }
    let mut parts: Vec<Pill> = Vec::new();
    let mut extras: Vec<String> = Vec::new();
    for (name, value) in entry.fields.iter() {
        let field_kind = LogFieldKey::from_str(&name.to_string()).ok();
        if matches!(
            field_kind,
            Some(LogFieldKey::Method | LogFieldKey::Path | LogFieldKey::Status)
        ) {
            continue;
        }
        extras.push(format!("{}={}", name.to_string(), value.to_string()));
    }
    if !extras.is_empty() {
        let extra = extras.into_iter().take(2).collect::<Vec<_>>().join(" · ");
        parts.push(Pill::fields(extra));
    }
    parts
}

fn field_value(entry: &TraceEntry, name: &LogFieldName) -> Option<Text> {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_log_value(Some(value)))
        .and_then(|value| value.into_option())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
enum LogFieldKey {
    #[strum(serialize = "method")]
    Method,
    #[strum(serialize = "path")]
    Path,
    #[strum(serialize = "status")]
    Status,
}
