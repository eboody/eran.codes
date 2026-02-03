use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{LogRow, Pill};

#[derive(Builder)]
pub struct TraceLog<'a> {
    pub entries: &'a [TraceEntry],
}

impl Render for TraceLog<'_> {
    fn render(&self) -> maud::Markup {
        if self.entries.is_empty() {
            return maud::html! {
                div class="demo-result trace-log muted" { "No trace entries recorded yet." }
            };
        }

        maud::html! {
            div class="demo-result trace-log" {
                p { strong { "Trace log" } }
                ul class="live-log-entries" {
                    @for entry in self.entries {
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
        .map(|(_, value)| value.clone())
        .filter(|value| value != "-")
}
