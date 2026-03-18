use maud::{Markup, Render};

use crate::trace_log::store;
use crate::trace_log::log::{message, target};
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;

use super::field_text;

pub fn request_rows(entries: &[store::TraceEntry]) -> Vec<Vec<Markup>> {
    entries
        .iter()
        .filter(|entry| {
            matches!(
                target::Kind::parse(&entry.target.to_string()),
                target::Kind::Known(target::Known::DemoRequest)
            ) && matches!(
                message::Kind::parse(&entry.message.to_string()),
                message::Kind::Known(message::Known::RequestEnd)
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

pub fn sse_rows(entries: &[store::TraceEntry]) -> Vec<Vec<Markup>> {
    entries
        .iter()
        .filter(|entry| {
            matches!(
                target::Kind::parse(&entry.target.to_string()),
                target::Kind::Known(target::Known::DemoSse)
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

pub fn chat_entries(entries: &[store::TraceEntry]) -> Vec<&store::TraceEntry> {
    entries
        .iter()
        .filter(|entry| {
            let target_kind = target::Kind::parse(&entry.target.to_string());
            let message_kind = message::Kind::parse(&entry.message.to_string());
            matches!(
                (target_kind, message_kind),
                (
                    target::Kind::Known(target::Known::DemoChat),
                    message::Kind::Known(message::Known::ChatMessageIncoming)
                ) | (
                    target::Kind::Known(target::Known::DemoSse),
                    message::Kind::Known(message::Known::ChatMessageBroadcast)
                )
            )
        })
        .collect()
}

fn method_pill(entry: &store::TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Method) {
        Some(method) => components::Pill::method(method).render(),
        None => components::Pill::fields("-").render(),
    }
}

fn path_pill(entry: &store::TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Path) {
        Some(path) => components::Pill::path(path).render(),
        None => components::Pill::fields("-").render(),
    }
}

fn status_pill(entry: &store::TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Status) {
        Some(status) => components::Pill::status(status).render(),
        None => components::Pill::fields("-").render(),
    }
}

fn latency_pill(entry: &store::TraceEntry) -> Option<Markup> {
    field_text(entry, LogFieldKey::LatencyMs)
        .map(|value| components::Pill::fields(format!("latency_ms={}", value)).render())
}

fn source_pill(entry: &store::TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Sender) {
        Some(sender) => components::Pill::fields(format!("source={}", sender)).render(),
        None => components::Pill::fields("source=unknown").render(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(
        timestamp: &str,
        target: &str,
        message: &str,
        fields: Vec<(&str, &str)>,
    ) -> store::TraceEntry {
        store::TraceEntry::builder()
            .timestamp(TimestampText::new(timestamp))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new(target))
            .message(LogMessageText::new(message))
            .fields(
                fields
                    .into_iter()
                    .map(|(name, value)| {
                        (LogFieldName::new(name), LogFieldValue::new(value))
                    })
                    .collect(),
            )
            .build()
    }

    #[test]
    fn request_rows_only_include_demo_request_ends() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.request",
                "request.end",
                vec![
                    ("status", "202"),
                    ("method", "POST"),
                    ("path", "/demo/chat/messages"),
                    ("sender", "you"),
                ],
            ),
            entry(
                "12:00:02",
                "demo.request",
                "request.start",
                vec![("status", "200")],
            ),
            entry(
                "12:00:03",
                "demo.sse",
                "chat message broadcast",
                vec![("selector", "[data-chat-messages]")],
            ),
        ];

        let rows = request_rows(&entries);

        assert_eq!(rows.len(), 1);
        let rendered = rows[0]
            .iter()
            .map(|cell| cell.clone().into_string())
            .collect::<String>();
        assert!(rendered.contains("POST"));
        assert!(rendered.contains("/demo/chat/messages"));
    }

    #[test]
    fn sse_rows_only_include_demo_sse_and_apply_fallbacks() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.sse",
                "chat message broadcast",
                vec![("selector", "[data-chat-messages]"), ("mode", "prepend")],
            ),
            entry(
                "12:00:02",
                "demo.chat",
                "chat.message.incoming",
                vec![("sender", "you")],
            ),
        ];

        let rows = sse_rows(&entries);

        assert_eq!(rows.len(), 1);
        let rendered = rows[0]
            .iter()
            .map(|cell| cell.clone().into_string())
            .collect::<String>();
        assert!(rendered.contains("[data-chat-messages]"));
        assert!(rendered.contains("prepend"));
        assert!(rendered.contains("-"));
    }

    #[test]
    fn chat_entries_include_only_incoming_and_broadcast_events() {
        let entries = vec![
            entry(
                "12:00:01",
                "demo.chat",
                "chat.message.incoming",
                vec![("sender", "you")],
            ),
            entry(
                "12:00:02",
                "demo.sse",
                "chat message broadcast",
                vec![("selector", "[data-chat-messages]")],
            ),
            entry(
                "12:00:03",
                "demo.request",
                "request.end",
                vec![("status", "202")],
            ),
        ];

        let rows = chat_entries(&entries);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].target.to_string(), "demo.chat");
        assert_eq!(rows[1].target.to_string(), "demo.sse");
    }
}
