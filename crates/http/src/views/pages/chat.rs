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
                            .role(crate::views::partials::ChatPanelRole::You)
                            .messages(self.messages.clone())
                            .build()
                            .render())
                        (crate::views::partials::ChatPanel::builder()
                            .role(crate::views::partials::ChatPanelRole::Demo)
                            .messages(self.messages.clone())
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
