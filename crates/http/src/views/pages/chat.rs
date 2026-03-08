use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::page::{Layout, UserNav};
use crate::views::partials::chat;

#[derive(Builder)]
pub struct Chat {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<chat::Message>,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
}

impl Render for Chat {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main
                class="container"
                data-signals=({
                    format!(
                        "{{roomId: '{}', body: '', botBody: '', sseConnected: false}}",
                        self.room_id,
                    )
                })
            {
                ({
                    chat::Hero::builder()
                        .room_name(self.room_name.clone())
                        .room_id(self.room_id.clone())
                        .build()
                })

                section data-chat-surface {
                    ({
                        chat::Connection::builder()
                            .connected_signal(Text::from("$sseConnected"))
                            .build()
                    })
                    (chat::PanelSet::builder()
                        .messages(self.messages.clone())
                        .build())
                    script src="/static/chat-demo.js" {}
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
