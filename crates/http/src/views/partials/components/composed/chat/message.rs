#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Visible,
    Pending,
    Removed,
}

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Left,
    Right,
}

impl From<domain::chat::message::Status> for Status {
    fn from(value: domain::chat::message::Status) -> Self {
        match value {
            domain::chat::message::Status::Visible => Self::Visible,
            domain::chat::message::Status::Pending => Self::Pending,
            domain::chat::message::Status::Removed => Self::Removed,
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Message {
    pub message_id: Text,
    pub author: Text,
    pub timestamp: Text,
    pub body: Text,
    pub status: Status,
}

impl Render for Message {
    fn render(&self) -> maud::Markup {
        let author = self.author.to_string();
        let avatar = author
            .chars()
            .next()
            .map(|character| character.to_uppercase().collect::<String>())
            .unwrap_or_else(|| "?".to_string());

        maud::html! {
            li id=(format!("chat-message-{}", self.message_id)) data-chat-message {
                span data-chat-avatar aria-hidden="true" { (avatar) }
                div data-chat-bubble {
                    div data-chat-meta {
                        strong { (author) }
                        span data-chat-timestamp { (&self.timestamp) }
                        span data-chat-status data-chat-status-kind=(self.status.as_ref()) {
                            (self.status.as_ref())
                        }
                    }
                    p data-chat-body { (&self.body) }
                }
            }
        }
    }
}
