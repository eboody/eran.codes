#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;

use crate::views::partials::components::chat;

#[derive(Clone, Debug, Builder)]
pub struct Window {
    #[builder(setters(name = with_title))]
    pub title: Option<Text>,
    #[builder(setters(name = with_connected_signal))]
    pub connected_signal: Option<Text>,
    pub side: chat::Side,
    pub messages: Vec<chat::Message>,
}

impl Render for Window {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-window {
                @if self.title.is_some() || self.connected_signal.is_some() {
                    header {
                        @if let Some(title) = &self.title {
                            span data-chat-role { (&title) }
                        }
                        @if let Some(connected_signal) = &self.connected_signal {
                            span
                                data-chat-room-state="live"
                                data-show=(connected_signal)
                                style="display:none;"
                            {
                                "Live"
                            }
                            span data-chat-room-state="offline" data-show=(format!("!{}", connected_signal)) {
                                "Offline"
                            }
                        }
                    }
                }
                div data-chat-feed {
                    ul data-chat-messages data-chat-side=(self.side.as_ref()) {
                        @for message in &self.messages {
                            (message.render())
                        }
                    }
                }
            }
        }
    }
}
