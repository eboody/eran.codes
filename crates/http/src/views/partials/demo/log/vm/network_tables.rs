use maud::{Markup, Render};

use crate::trace_log::TraceEntry;
use crate::trace_log::{LogMessageKind, LogMessageKnown, LogTargetKind, LogTargetKnown};
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::Pill;

use super::field_text;

pub fn request_rows(entries: &[TraceEntry]) -> Vec<Vec<Markup>> {
    entries
        .iter()
        .filter(|entry| {
            matches!(
                LogTargetKind::parse(&entry.target.to_string()),
                LogTargetKind::Known(LogTargetKnown::DemoRequest)
            ) && matches!(
                LogMessageKind::parse(&entry.message.to_string()),
                LogMessageKind::Known(LogMessageKnown::RequestEnd)
            )
        })
        .rev()
        .take(20)
        .map(|entry| {
            vec![
                maud::html! { (&entry.timestamp) },
                maud::html! { (status_pill(entry)) },
                maud::html! { (method_pill(entry)) },
                maud::html! { (path_pill(entry)) },
                maud::html! { (source_pill(entry)) },
                maud::html! { (latency_pill(entry).unwrap_or_default()) },
            ]
        })
        .collect()
}

pub fn sse_rows(entries: &[TraceEntry]) -> Vec<Vec<Markup>> {
    entries
        .iter()
        .filter(|entry| {
            matches!(
                LogTargetKind::parse(&entry.target.to_string()),
                LogTargetKind::Known(LogTargetKnown::DemoSse)
            )
        })
        .rev()
        .take(20)
        .map(|entry| {
            vec![
                maud::html! { (&entry.timestamp) },
                maud::html! { (&entry.message) },
                maud::html! { (field_text(entry, LogFieldKey::Selector).unwrap_or_else(|| Text::from("-"))) },
                maud::html! { (field_text(entry, LogFieldKey::Mode).unwrap_or_else(|| Text::from("-"))) },
                maud::html! { (field_text(entry, LogFieldKey::PayloadBytes).unwrap_or_else(|| Text::from("-"))) },
            ]
        })
        .collect()
}

pub fn chat_entries(entries: &[TraceEntry]) -> Vec<&TraceEntry> {
    entries
        .iter()
        .filter(|entry| {
            let target_kind = LogTargetKind::parse(&entry.target.to_string());
            let message_kind = LogMessageKind::parse(&entry.message.to_string());
            matches!(
                (target_kind, message_kind),
                (
                    LogTargetKind::Known(LogTargetKnown::DemoChat),
                    LogMessageKind::Known(LogMessageKnown::ChatMessageIncoming)
                ) | (
                    LogTargetKind::Known(LogTargetKnown::DemoSse),
                    LogMessageKind::Known(LogMessageKnown::ChatMessageBroadcast)
                )
            )
        })
        .collect()
}

fn method_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Method) {
        Some(method) => Pill::method(method).render(),
        None => Pill::fields("-").render(),
    }
}

fn path_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Path) {
        Some(path) => Pill::path(path).render(),
        None => Pill::fields("-").render(),
    }
}

fn status_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Status) {
        Some(status) => Pill::status(status).render(),
        None => Pill::fields("-").render(),
    }
}

fn latency_pill(entry: &TraceEntry) -> Option<Markup> {
    field_text(entry, LogFieldKey::LatencyMs)
        .map(|value| Pill::fields(format!("latency_ms={}", value)).render())
}

fn source_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Sender) {
        Some(sender) => Pill::fields(format!("source={}", sender)).render(),
        None => Pill::fields("source=unknown").render(),
    }
}
