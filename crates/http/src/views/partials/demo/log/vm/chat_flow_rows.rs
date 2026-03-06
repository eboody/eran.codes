use std::str::FromStr;

use maud::{Markup, Render};

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::{BadgeKind, Pill};
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
        FlowDirection::Incoming => Pill::fields("incoming").render(),
        FlowDirection::Outgoing => Pill::fields("outgoing").render(),
        FlowDirection::Unknown => Pill::fields("unknown").render(),
    }
}

fn sender_pill(entry: &TraceEntry) -> Markup {
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (Text::from("You"), BadgeKind::You),
        ChatSender::Demo => (Text::from("Demo"), BadgeKind::Demo),
        ChatSender::Unknown => (Text::from("User"), BadgeKind::Secondary),
    };
    Pill::badge(label, kind).render()
}

fn receiver_pill(entry: &TraceEntry) -> Markup {
    match field_text(entry, LogFieldKey::Receiver) {
        Some(receiver) => Pill::fields(format!("to:{}", receiver)).render(),
        None => Pill::fields("to:unknown").render(),
    }
}

fn user_pill(entry: &TraceEntry) -> Markup {
    let Some(user_id) = field_text(entry, LogFieldKey::UserId) else {
        return Pill::fields("user:unknown").render();
    };
    let user_text = user_id.to_string();
    let short_id = user_text.split('-').next().unwrap_or(user_text.as_str());
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (format!("You ({short_id})"), BadgeKind::You),
        ChatSender::Demo => (format!("Demo ({short_id})"), BadgeKind::Demo),
        ChatSender::Unknown => (format!("User ({short_id})"), BadgeKind::Secondary),
    };
    Pill::badge(label, kind).render()
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
