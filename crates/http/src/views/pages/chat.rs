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
            main class="container" {
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
                    (crate::views::partials::ChatWindow::builder()
                        .messages(self.messages.clone())
                        .build()
                        .render())
                }

                section class="chat-input" {
                    form method="post"
                        action="/demo/chat/messages"
                        data-signals=(format!("{{roomId: '{}', body: ''}}", self.room_id))
                        data-on:submit="@post('/demo/chat/messages'); $body = ''"
                    {
                        label {
                            "Message"
                            input type="text"
                                name="body"
                                placeholder="Say something..."
                                data-bind:body
                                required;
                        }
                        button type="submit" { "Send" }
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
