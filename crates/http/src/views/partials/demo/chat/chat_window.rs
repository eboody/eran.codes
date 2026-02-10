use bon::Builder;
use maud::Render;

use crate::views::partials::ChatMessages;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct ChatWindow {
    pub title: Option<Text>,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

impl Render for ChatWindow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="chat-window" {
                @if let Some(title) = &self.title {
                    header {
                        span class="role" { (title.to_string()) }
                    }
                }
                (ChatMessages::builder()
                    .messages(self.messages.clone())
                    .build()
                    .render())
            }
        }
    }
}
