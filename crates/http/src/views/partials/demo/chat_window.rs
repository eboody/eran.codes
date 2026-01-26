use bon::Builder;
use maud::Render;

use crate::views::partials::ChatMessages;

#[derive(Clone, Debug, Builder)]
pub struct ChatWindow {
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

impl Render for ChatWindow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id="chat-window" {
                (ChatMessages::builder()
                    .messages(self.messages.clone())
                    .build()
                    .render())
            }
        }
    }
}
