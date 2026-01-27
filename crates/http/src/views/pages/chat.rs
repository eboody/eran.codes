use bon::Builder;
use maud::Render;

use crate::views::page::{Layout, UserNav};
use crate::views::partials::ChatMessage;

#[derive(Builder)]
pub struct Chat {
    pub room_id: String,
    pub room_name: String,
    pub messages: Vec<ChatMessage>,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
}

impl Render for Chat {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container"
                data-signals=(format!("{{roomId: '{}', body: '', botBody: '', sseConnected: false}}", self.room_id)) {
                header class="hero" {
                    div {
                        h1 { "Live chat room" }
                        p { "Enterprise demo: persistence, rate limits, moderation, and SSE fanout." }
                    }
                    aside class="hero-card" {
                        h3 { "Room" }
                        p { (&self.room_name) }
                        p class="muted" { "Room id: " (&self.room_id) }
                        a class="button secondary" href="/demo/chat/moderation" { "Moderation queue" }
                    }
                }

                section class="chat-panel" {
                    div class="pill-row" {
                        span class="pill secondary" data-show="$sseConnected" { "SSE connected" }
                        span class="pill muted" data-show="!$sseConnected" { "SSE disconnected" }
                    }
                    div class="chat-columns" {
                        (crate::views::partials::ChatPanel::builder()
                            .title("You".to_string())
                            .messages(self.messages.clone())
                            .action("/demo/chat/messages".to_string())
                            .input_label("Message as you".to_string())
                            .placeholder("Say something...".to_string())
                            .input_name("body".to_string())
                            .input_signal("body".to_string())
                            .button_label("Send".to_string())
                            .build()
                            .render())
                        (crate::views::partials::ChatPanel::builder()
                            .title("Demo user".to_string())
                            .messages(self.messages.clone())
                            .action("/demo/chat/messages/demo".to_string())
                            .input_label("Message as demo user".to_string())
                            .placeholder("Send as demo user...".to_string())
                            .input_name("body".to_string())
                            .input_signal("botBody".to_string())
                            .button_label("Send as demo".to_string())
                            .maybe_button_class(Some("secondary".to_string()))
                            .build()
                            .render())
                    }
                }
            }
        };

        Layout::builder()
            .title("Chat room")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
