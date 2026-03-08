use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::chat;

#[derive(Clone, Debug, Builder)]
pub struct Window {
    pub title: Option<Text>,
    pub side: chat::message::Side,
    pub messages: Vec<chat::Message>,
}

impl Render for Window {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-window {
                @if let Some(title) = &self.title {
                    header {
                        span data-chat-role { (title) }
                        span data-chat-room-state="live" data-show="$sseConnected" style="display:none;" {
                            "Live"
                        }
                        span data-chat-room-state="offline" data-show="!$sseConnected" {
                            "Offline"
                        }
                    }
                }
                (chat::message::Messages::builder()
                    .side(self.side)
                    .messages(self.messages.clone())
                    .build())
            }
        }
    }
}
