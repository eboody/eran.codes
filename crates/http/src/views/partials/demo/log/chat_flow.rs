use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{BadgeKind, DataTable, EmptyState, FieldValue, LogPanel, Pill, TableVariant};

#[derive(Clone, Debug, Builder)]
pub struct ChatFlow<'a> {
    pub entries: &'a [&'a TraceEntry],
}

impl Render for ChatFlow<'_> {
    fn render(&self) -> maud::Markup {
        let body = if self.entries.is_empty() {
            EmptyState::builder()
                .message("No chat messages yet. Send a message to see request/response flow.".to_string())
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
                        maud::html! { (entry.timestamp.clone()) },
                        maud::html! { (direction_pill(entry).render()) },
                        maud::html! { (sender_pill(entry).render()) },
                        maud::html! { (receiver_pill(entry).render()) },
                        maud::html! { (user_pill(entry).render()) },
                        maud::html! { (field_value_text(entry, "body")) },
                    ]
                })
                .collect::<Vec<_>>();
            DataTable::builder()
                .headers(vec![
                    "Time".to_string(),
                    "Direction".to_string(),
                    "Sender".to_string(),
                    "Receiver".to_string(),
                    "User".to_string(),
                    "Body".to_string(),
                ])
                .rows(rows)
                .variant(TableVariant::ChatFlow)
                .build()
                .render()
        };

        LogPanel::builder()
            .title("Chat message flow".to_string())
            .body(body)
            .build()
            .render()
    }
}

fn direction_pill(entry: &TraceEntry) -> Pill {
    match FieldValue::from_str(&field_value_text(entry, "direction")) {
        FieldValue::Value(value) => match value.as_str() {
            "incoming" => Pill::fields("incoming".to_string()),
            "outgoing" => Pill::fields("outgoing".to_string()),
            _ => Pill::fields("unknown".to_string()),
        },
        FieldValue::Missing => Pill::fields("unknown".to_string()),
    }
}

fn sender_pill(entry: &TraceEntry) -> Pill {
    let sender = ChatSender::from_entry(entry);
    let (label, kind) = match sender {
        ChatSender::You => ("You".to_string(), BadgeKind::You),
        ChatSender::Demo => ("Demo".to_string(), BadgeKind::Demo),
        ChatSender::Unknown => ("User".to_string(), BadgeKind::Secondary),
    };
    Pill::badge(label, kind)
}

fn receiver_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "receiver").into_option() {
        Some(receiver) => Pill::fields(format!("to:{receiver}")),
        None => Pill::fields("to:unknown".to_string()),
    }
}

fn user_pill(entry: &TraceEntry) -> Pill {
    let user_id = field_value(entry, "user_id").into_option();
    let Some(user_id) = user_id else {
        return Pill::fields("user:unknown".to_string());
    };
    let short_id = user_id.split('-').next().unwrap_or(user_id.as_str());
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
        let sender = field_value_text(entry, "sender");
        match sender.as_str() {
            "you" => Self::You,
            "demo" => Self::Demo,
            _ => Self::Unknown,
        }
    }
}

fn field_value(entry: &TraceEntry, name: &str) -> FieldValue {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| FieldValue::from_str(value))
        .unwrap_or(FieldValue::Missing)
}

fn field_value_text(entry: &TraceEntry, name: &str) -> String {
    field_value(entry, name)
        .into_option()
        .unwrap_or_else(|| "-".to_string())
}
