use bon::Builder;
use maud::Render;

use crate::trace_log::TraceEntry;
use crate::views::partials::{BadgeKind, EmptyState, LogPanel, LogRow, Pill};

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
                ul class="live-log-entries" {
                    @for entry in self.entries.iter().rev().take(20) {
                        (LogRow::builder()
                            .timestamp(entry.timestamp.clone())
                            .message(field_value(entry, "body"))
                            .pills(chat_pills(entry))
                            .build()
                            .render())
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

fn chat_pills(entry: &TraceEntry) -> Vec<Pill> {
    let mut pills = Vec::new();
    pills.push(Pill::level(entry.level.clone()));
    pills.push(direction_pill(entry));
    pills.push(sender_pill(entry));
    pills.push(receiver_pill(entry));
    pills.push(user_pill(entry));
    pills
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
