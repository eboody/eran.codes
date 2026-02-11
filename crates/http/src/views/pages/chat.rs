use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::page::{Layout, UserNav};
use crate::views::partials::ChatMessage;

#[derive(Builder)]
pub struct Chat {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<ChatMessage>,
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
                    crate::views::partials::ChatHero::builder()
                        .room_name(self.room_name.clone())
                        .room_id(self.room_id.clone())
                        .build()
                        .render()
                })

                section class="chat-panel" {
                    ({
                        crate::views::partials::ChatConnection::builder()
                            .connected_signal(Text::from("$sseConnected"))
                            .build()
                            .render()
                    })
                    div class="chat-columns" {
                        ({
                            crate::views::partials::ChatPanel::builder()
                                .role(crate::views::partials::ChatPanelRole::You)
                                .messages(self.messages.clone())
                                .build()
                                .render()
                        })
                        ({
                            crate::views::partials::ChatPanel::builder()
                                .role(crate::views::partials::ChatPanelRole::Demo)
                                .messages(self.messages.clone())
                                .build()
                                .render()
                        })
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
