use std::str::FromStr;

use maud::{Markup, Render};

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;
use strum_macros::{Display, EnumString};

use super::field_text;

pub fn chat_flow_rows(entries: &[&TraceEntry]) -> Vec<Vec<Markup>> {
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
                maud::html! { (field_text(entry, LogFieldKey::Body).unwrap_or_else(|| Text::from("-"))) },
            ]
        })
        .collect()
}

fn direction_pill(entry: &TraceEntry) -> Markup {
    match FlowDirection::from_entry(entry) {
        FlowDirection::Incoming => components::Pill::fields("incoming").render(),
        FlowDirection::Outgoing => components::Pill::fields("outgoing").render(),
        FlowDirection::Unknown => components::Pill::fields("unknown").render(),
    }
}

fn sender_pill(entry: &TraceEntry) -> Markup {
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (Text::from("You"), components::BadgeKind::You),
        ChatSender::Demo => (Text::from("Demo"), components::BadgeKind::Demo),
        ChatSender::Unknown => (Text::from("User"), components::BadgeKind::Secondary),
    };
    components::Pill::badge(label, kind).render()
}

fn receiver_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Receiver) {
        Some(receiver) => components::Pill::fields(format!("to:{}", receiver)).render(),
        None => components::Pill::fields("to:unknown").render(),
    }
}

fn user_pill(entry: &TraceEntry) -> Markup {
    let Some(user_id) = field_text(entry, LogFieldKey::UserId) else {
        return components::Pill::fields("user:unknown").render();
    };
    let user_text = user_id.to_string();
    let short_id = user_text.split('-').next().unwrap_or(user_text.as_str());
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (format!("You ({short_id})"), components::BadgeKind::You),
        ChatSender::Demo => (format!("Demo ({short_id})"), components::BadgeKind::Demo),
        ChatSender::Unknown => (format!("User ({short_id})"), components::BadgeKind::Secondary),
    };
    components::Pill::badge(label, kind).render()
}

#[derive(Clone, Copy, Debug)]
enum ChatSender {
    You,
    Demo,
    Unknown,
}

impl ChatSender {
    fn from_entry(entry: &TraceEntry) -> Self {
        let Some(sender) = field_text(entry, LogFieldKey::Sender) else {
            return Self::Unknown;
        };
        ChatSenderKnown::from_str(&sender.to_string())
            .map(Into::into)
            .unwrap_or(Self::Unknown)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
enum ChatSenderKnown {
    #[strum(serialize = "you")]
    You,
    #[strum(serialize = "demo")]
    Demo,
}

impl From<ChatSenderKnown> for ChatSender {
    fn from(kind: ChatSenderKnown) -> Self {
        match kind {
            ChatSenderKnown::You => Self::You,
            ChatSenderKnown::Demo => Self::Demo,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
enum FlowDirectionKnown {
    #[strum(serialize = "incoming")]
    Incoming,
    #[strum(serialize = "outgoing")]
    Outgoing,
}

#[derive(Clone, Copy, Debug)]
enum FlowDirection {
    Incoming,
    Outgoing,
    Unknown,
}

impl FlowDirection {
    fn from_entry(entry: &TraceEntry) -> Self {
        let Some(direction) = field_text(entry, LogFieldKey::Direction) else {
            return Self::Unknown;
        };
        FlowDirectionKnown::from_str(&direction.to_string())
            .map(Into::into)
            .unwrap_or(Self::Unknown)
    }
}

impl From<FlowDirectionKnown> for FlowDirection {
    fn from(kind: FlowDirectionKnown) -> Self {
        match kind {
            FlowDirectionKnown::Incoming => Self::Incoming,
            FlowDirectionKnown::Outgoing => Self::Outgoing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
        TimestampText,
    };

    fn entry(timestamp: &str, fields: Vec<(&str, &str)>) -> TraceEntry {
        TraceEntry::builder()
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
}
