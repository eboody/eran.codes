use bon::Builder;
use maud::Render;

use crate::views::partials::chat;

#[derive(Clone, Debug, Builder)]
pub struct Set {
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub interactivity: chat::Mode,
}

impl Render for Set {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-columns {
                (super::Panel::builder()
                    .role(super::Role::You)
                    .messages(self.messages.clone())
                    .interactivity(self.interactivity)
                    .build())
                (super::Panel::builder()
                    .role(super::Role::Demo)
                    .messages(self.messages.clone())
                    .interactivity(self.interactivity)
                    .build())
            }
        }
    }
}
