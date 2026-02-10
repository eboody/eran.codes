use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{EmptyState, FieldValue, LogPanel, LogRow, Pill};

#[derive(Builder)]
pub struct TraceLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for TraceLog<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            EmptyState::builder()
                .message("No trace entries recorded yet.".to_string())
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
                                    (Pill::fields("request_id=unknown".to_string()).render())
                                }
                                span class="muted" { (format!("{} events", group.entries.len())) }
                            }
                            ul class="live-log-entries" {
                                @for entry in group.entries {
                                    (LogRow::builder()
                                        .timestamp(entry.timestamp.clone())
                                        .message(entry.message.clone())
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
                .title("Trace log".to_string())
                .body(body)
                .build()
                .render())
        }
    }
}

struct LogGroup<'a> {
    request_id: Option<String>,
    entries: Vec<&'a TraceEntry>,
}

fn group_by_request<'a, I>(entries: I) -> Vec<LogGroup<'a>>
where
    I: IntoIterator<Item = &'a TraceEntry>,
{
    let mut groups: Vec<LogGroup<'a>> = Vec::new();
    let mut order: Vec<Option<String>> = Vec::new();
    let mut map: std::collections::HashMap<Option<String>, Vec<&'a TraceEntry>> =
        std::collections::HashMap::new();
    for entry in entries {
        let request_id = field_value(entry, "request_id");
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

fn short_request_id(value: &str) -> String {
    value.split('-').next().unwrap_or(value).to_string()
}

fn build_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(Pill::level(entry.level.clone()));
    if let Some(status) = field_value(entry, "status") {
        pills.push(Pill::status(status.clone()));
    }
    if let Some(method) = field_value(entry, "method") {
        pills.push(Pill::method(method.clone()));
    }
    if let Some(path) = field_value(entry, "path") {
        pills.push(Pill::path(path));
    }
    pills.push(Pill::target(entry.target.clone()));
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
        match name.as_str() {
            "method" | "path" | "status" => continue,
            _ => extras.push(format!("{}={}", name, value)),
        }
    }
    if !extras.is_empty() {
        let extra = extras.into_iter().take(2).collect::<Vec<_>>().join(" · ");
        parts.push(Pill::fields(extra));
    }
    parts
}

fn field_value(entry: &TraceEntry, name: &str) -> Option<String> {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_str(value))
        .and_then(|value| value.into_option())
}
