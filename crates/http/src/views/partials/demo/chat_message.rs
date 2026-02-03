use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct ChatMessage {
    pub message_id: String,
    pub author: String,
    pub timestamp: String,
    pub body: String,
    pub status: String,
}

impl Render for ChatMessage {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li id=(format!("chat-message-{}", self.message_id)) class="chat-message" {
                div class="meta" {
                    strong { (&self.author) }
                    span class="timestamp" { (&self.timestamp) }
                    span class="status" { (&self.status) }
                }
                p { (&self.body) }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct ChatMessages {
    pub messages: Vec<ChatMessage>,
}

impl Render for ChatMessages {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class="chat-messages" {
                @if self.messages.is_empty() {
                    li class="muted" { "No messages yet." }
                } @else {
                    @for message in &self.messages {
                        (message.render())
                    }
                }
            }
        }
    }
}
