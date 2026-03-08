use bon::Builder;
use maud::Render;

use crate::views::partials::chat;

#[derive(Clone, Debug, Builder)]
pub struct PanelSet {
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub interactivity: chat::Mode,
}

impl Render for PanelSet {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-columns {
                (chat::Panel::builder()
                    .role(chat::Role::You)
                    .messages(self.messages.clone())
                    .interactivity(self.interactivity)
                    .build())
                (chat::Panel::builder()
                    .role(chat::Role::Demo)
                    .messages(self.messages.clone())
                    .interactivity(self.interactivity)
                    .build())
            }
        }
    }
}
