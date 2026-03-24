use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::{page, partials};

#[derive(Builder)]
pub struct Chat {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<partials::components::chat::Message>,
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for Chat {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            div class="u-page-stack" data-chat-page data-page-section {
                ({
                    partials::components::chat::Hero::builder()
                        .room_name(self.room_name.clone())
                        .room_id(self.room_id.clone())
                        .build()
                })

                (partials::components::chat::Surface::builder()
                    .room_id(self.room_id.clone())
                    .messages(self.messages.clone())
                    .mode(partials::components::chat::Mode::Interactive)
                    .build())
            }
        };
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title("Chat room")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn uses_shared_page_stack_contract() {
        let markup = Chat::builder()
            .room_id(Text::from("room-1"))
            .room_name(Text::from("Lobby"))
            .messages(Vec::new())
            .build()
            .render()
            .into_string();

        assert!(markup.contains("class=\"u-page-stack\""));
        assert!(markup.contains("data-chat-page"));
    }
}
