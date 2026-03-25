use maud::{Markup, Render};

use crate::trace_log::{
    demo_chat::{Direction as FlowDirection, Sender as ChatSender},
    store,
};
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;

pub fn chat_flow_rows(entries: &[&store::TraceEntry]) -> Vec<Vec<Markup>> {
    entries
        .iter()
        .rev()
        .take(20)
        .map(|entry| {
            vec![
                maud::html! { (&entry.timestamp) },
                maud::html! { (direction_pill(entry)) },
                maud::html! { (sender_pill(entry)) },
                maud::html! { (receiver_pill(entry)) },
                maud::html! { (user_pill(entry)) },
                maud::html! { (entry.field_text(LogFieldKey::Body).cloned().unwrap_or_else(|| Text::from("-"))) },
            ]
        })
        .collect()
}

fn direction_pill(entry: &store::TraceEntry) -> Markup {
    match FlowDirection::from_entry(entry) {
        FlowDirection::Incoming => components::Pill::fields("incoming").render(),
        FlowDirection::Outgoing => components::Pill::fields("outgoing").render(),
        FlowDirection::Unknown => components::Pill::fields("unknown").render(),
    }
}

fn sender_pill(entry: &store::TraceEntry) -> Markup {
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (Text::from("You"), components::BadgeKind::You),
        ChatSender::Demo => (Text::from("Demo"), components::BadgeKind::Demo),
        ChatSender::Unknown => (Text::from("User"), components::BadgeKind::Secondary),
    };
    components::Pill::badge(label, kind).render()
}

fn receiver_pill(entry: &store::TraceEntry) -> Markup {
    match entry.field_text(LogFieldKey::Receiver) {
        Some(receiver) => components::Pill::fields(format!("to:{}", receiver)).render(),
        None => components::Pill::fields("to:unknown").render(),
    }
}

fn user_pill(entry: &store::TraceEntry) -> Markup {
    let sender = ChatSender::from_entry(entry);
    super::redaction::chat_user_pill(sender, entry.field_text(LogFieldKey::UserId)).render()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(timestamp: &str, fields: Vec<(&str, &str)>) -> store::TraceEntry {
        store::TraceEntry::builder()
            .timestamp(TimestampText::new(timestamp))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new("demo.chat"))
            .message(LogMessageText::new("chat.message.incoming"))
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
    fn rows_are_newest_first() {
        let a = entry(
            "12:00:01",
            vec![("direction", "incoming"), ("sender", "you")],
        );
        let b = entry(
            "12:00:02",
            vec![("direction", "outgoing"), ("sender", "demo")],
        );
        let rows = chat_flow_rows(&[&a, &b]);

        assert_eq!(rows.len(), 2);
        let first_row = rows[0]
            .iter()
            .map(|cell| cell.clone().into_string())
            .collect::<String>();
        assert!(first_row.contains("12:00:02"));
    }

    #[test]
    fn unknown_sender_and_direction_fallbacks_are_rendered() {
        let row = entry("12:00:01", vec![("receiver", "clients")]);
        let rows = chat_flow_rows(&[&row]);

        let rendered = rows[0]
            .iter()
            .map(|cell| cell.clone().into_string())
            .collect::<String>();
        assert!(rendered.contains("unknown"));
        assert!(rendered.contains("to:clients"));
        assert!(rendered.contains("user:unknown"));
    }

    #[test]
    fn user_ids_are_redacted_from_chat_flow_rows() {
        let row = entry(
            "12:00:01",
            vec![
                ("sender", "you"),
                ("receiver", "server"),
                ("user_id", "abc-def"),
            ],
        );
        let rows = chat_flow_rows(&[&row]);

        let rendered = rows[0]
            .iter()
            .map(|cell| cell.clone().into_string())
            .collect::<String>();
        assert!(rendered.contains("You (redacted)"));
        assert!(!rendered.contains("abc-def"));
    }
}
