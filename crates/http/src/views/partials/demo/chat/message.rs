use crate::types::Text;
use bon::Builder;
use maud::Render;

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
        let status = self.status.as_ref();
        let avatar = author
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_else(|| "?".to_string());
        maud::html! {
            li id=(format!("chat-message-{}", self.message_id)) data-chat-message {
                span data-chat-avatar aria-hidden="true" { (avatar) }
                div data-chat-bubble {
                    div data-chat-meta {
                        strong { (author) }
                        span data-chat-timestamp { (&self.timestamp) }
                        span data-chat-status data-chat-status-kind=(status) {
                            (status)
                        }
                    }
                    p data-chat-body { (&self.body) }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Messages {
    pub messages: Vec<Message>,
    pub side: Side,
}

impl Render for Messages {
    fn render(&self) -> maud::Markup {
        let side = self.side.as_ref();
        maud::html! {
            div data-chat-feed {
                ul data-chat-messages data-chat-side=(side) {
                    @for message in &self.messages {
                        (message.render())
                    }
                }
            }
        }
    }
}
