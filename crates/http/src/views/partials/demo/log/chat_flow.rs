use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use std::str::FromStr;

use crate::types::{LogFieldKey, LogFieldName, Text};
use strum_macros::{Display, EnumString};
use crate::views::partials::components::{
    BadgeKind, DataTable, EmptyState, FieldValue, LogPanel, Pill, TableVariant,
};

#[derive(Clone, Debug, Builder)]
pub struct ChatFlow<'a> {
    pub entries: &'a [&'a TraceEntry],
}

impl Render for ChatFlow<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            EmptyState::builder()
                .message(Text::from("No chat messages yet. Send a message to see request/response flow."))
                .build()
                .render()
        } else {
            let rows = self
                .entries
                .iter()
                .rev()
                .take(20)
                .map(|entry| {
                    vec![
                        maud::html! { (&entry.timestamp) },
                        maud::html! { (direction_pill(entry).render()) },
                        maud::html! { (sender_pill(entry).render()) },
                        maud::html! { (receiver_pill(entry).render()) },
                        maud::html! { (user_pill(entry).render()) },
                        maud::html! { (field_value_text(entry, &LogFieldName::from(LogFieldKey::Body)).unwrap_or_else(|| Text::from("-"))) },
                    ]
                })
                .collect::<Vec<_>>();
            DataTable::builder()
                .headers(vec![
                    Text::from("Time"),
                    Text::from("Direction"),
                    Text::from("Sender"),
                    Text::from("Receiver"),
                    Text::from("User"),
                    Text::from("Body"),
                ])
                .rows(rows)
                .variant(TableVariant::ChatFlow)
                .build()
                .render()
        };

        LogPanel::builder()
            .title(Text::from("Chat message flow"))
            .body(body)
            .build()
            .render()
    }
}

fn direction_pill(entry: &TraceEntry) -> Pill {
    match FlowDirection::from_entry(entry) {
        FlowDirection::Incoming => Pill::fields("incoming"),
        FlowDirection::Outgoing => Pill::fields("outgoing"),
        FlowDirection::Unknown => Pill::fields("unknown"),
    }
}

fn sender_pill(entry: &TraceEntry) -> Pill {
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (Text::from("You"), BadgeKind::You),
        ChatSender::Demo => (Text::from("Demo"), BadgeKind::Demo),
        ChatSender::Unknown => (Text::from("User"), BadgeKind::Secondary),
    };
    Pill::badge(label, kind)
}

fn receiver_pill(entry: &TraceEntry) -> Pill {
    let receiver: Option<Text> =
        field_value(entry, &LogFieldName::from(LogFieldKey::Receiver)).into_option();
    match receiver {
        Some(receiver) => Pill::fields(format!("to:{}", receiver)),
        None => Pill::fields("to:unknown"),
    }
}

fn user_pill(entry: &TraceEntry) -> Pill {
    let user_id: Option<Text> =
        field_value(entry, &LogFieldName::from(LogFieldKey::UserId)).into_option();
    let Some(user_id) = user_id else {
        return Pill::fields("user:unknown");
    };
    let user_text = user_id.to_string();
    let short_id = user_text.split('-').next().unwrap_or(user_text.as_str());
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => (format!("You ({short_id})"), BadgeKind::You),
        ChatSender::Demo => (format!("Demo ({short_id})"), BadgeKind::Demo),
        ChatSender::Unknown => (format!("User ({short_id})"), BadgeKind::Secondary),
    };
    Pill::badge(label, kind)
}

#[derive(Clone, Copy, Debug)]
enum ChatSender {
    You,
    Demo,
    Unknown,
}

impl ChatSender {
    fn from_entry(entry: &TraceEntry) -> Self {
        let sender = field_value_text(entry, &LogFieldName::from(LogFieldKey::Sender));
        let Some(sender) = sender else {
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
        let direction = field_value_text(entry, &LogFieldName::from(LogFieldKey::Direction));
        let Some(direction) = direction else {
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

fn field_value(entry: &TraceEntry, name: &LogFieldName) -> FieldValue {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_log_value(Some(value)))
        .unwrap_or(FieldValue::Missing)
}

fn field_value_text(entry: &TraceEntry, name: &LogFieldName) -> Option<Text> {
    field_value(entry, name).into_option()
}
