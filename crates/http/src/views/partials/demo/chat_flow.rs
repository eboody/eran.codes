use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{BadgeKind, EmptyState, LogPanel, Pill};

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
            maud::html! {
                table class="chat-flow-table" {
                    thead {
                        tr {
                            th { "Time" }
                            th { "Direction" }
                            th { "Sender" }
                            th { "Receiver" }
                            th { "User" }
                            th { "Body" }
                        }
                    }
                    tbody {
                        @for entry in self.entries.iter().rev().take(20) {
                            tr {
                                td { (entry.timestamp.clone()) }
                                td { (direction_pill(entry).render()) }
                                td { (sender_pill(entry).render()) }
                                td { (receiver_pill(entry).render()) }
                                td { (user_pill(entry).render()) }
                                td { (field_value(entry, "body")) }
                            }
                        }
                    }
                }
            }
        };

        LogPanel::builder()
            .title("Chat message flow".to_string())
            .body(body)
            .build()
            .render()
    }
}

fn direction_pill(entry: &TraceEntry) -> Pill {
    match field_value(entry, "direction").as_str() {
        "incoming" => Pill::fields("incoming".to_string()),
        "outgoing" => Pill::fields("outgoing".to_string()),
        _ => Pill::fields("unknown".to_string()),
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
    let receiver = field_value(entry, "receiver");
    Pill::fields(format!("to:{receiver}"))
}

fn user_pill(entry: &TraceEntry) -> Pill {
    let user_id = field_value(entry, "user_id");
    if user_id == "-" {
        return Pill::fields("user:unknown".to_string());
    }
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
        let sender = field_value(entry, "sender");
        match sender.as_str() {
            "you" => Self::You,
            "demo" => Self::Demo,
            _ => Self::Unknown,
        }
    }
}

fn field_value(entry: &TraceEntry, name: &str) -> String {
    entry
        .fields
        .iter()
        .find(|(field, _)| field == name)
        .map(|(_, value)| value.clone())
        .unwrap_or_else(|| "-".to_string())
}
