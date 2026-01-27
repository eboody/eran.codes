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
                        div class="chat-stack" {
                            (crate::views::partials::ChatWindow::builder()
                                .maybe_title(Some("You".to_string()))
                                .messages(self.messages.clone())
                                .build()
                                .render())
                            form method="post"
                                action="/demo/chat/messages"
                                data-target=".chat-messages"
                                data-swap="append"
                                data-on:submit="@post('/demo/chat/messages'); $body = ''"
                            {
                                label {
                                    "Message as you"
                                    input type="text"
                                        name="body"
                                        placeholder="Say something..."
                                        data-bind:body
                                        required;
                                }
                                button type="submit" { "Send" }
                            }
                        }
                        div class="chat-stack" {
                            (crate::views::partials::ChatWindow::builder()
                                .maybe_title(Some("Demo user".to_string()))
                                .messages(self.messages.clone())
                                .build()
                                .render())
                            form method="post"
                                action="/demo/chat/messages/demo"
                                data-target=".chat-messages"
                                data-swap="append"
                                data-on:submit="@post('/demo/chat/messages/demo'); $botBody = ''"
                            {
                                label {
                                    "Message as demo user"
                                    input type="text"
                                        name="body"
                                        placeholder="Send as demo user..."
                                        data-bind:botBody
                                        required;
                                }
                                button type="submit" class="secondary" { "Send as demo" }
                            }
                        }
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
