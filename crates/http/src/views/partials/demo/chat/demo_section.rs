use bon::Builder;
use maud::Render;
use serde_json::json;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::{SectionHeader, SectionHeaderMetaText, button, chat};

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub interactivity: chat::Mode,
}

impl DemoSection {
    pub const ANCHOR_ID: &'static str = "chat-demo";
}

impl Render for DemoSection {
    fn render(&self) -> maud::Markup {
        let subtitle = match self.interactivity {
            chat::Mode::Interactive => Text::from(
                "Send messages as yourself or the demo user and watch SSE fanout.",
            ),
            chat::Mode::DemoOnly => {
                Text::from("Try live posts as the demo user. Sign in to send as yourself.")
            }
        };
        maud::html! {
            section
                id=(Self::ANCHOR_ID)
                data-lab-chat-surface
                data-chat-surface
                data-signals=(json!({
                    "roomId": self.room_id.to_string(),
                    "body": "",
                    "botBody": "",
                    "sseConnected": false
                }).to_string()) {
                (chat::surface_styles())
                (SectionHeader::builder()
                    .title(Text::from("Live chat room"))
                    .subtitle(subtitle)
                    .action(match self.interactivity {
                        chat::Mode::Interactive => button::Button::builder()
                            .label(Text::from("Moderation queue"))
                            .variant(button::Variant::Secondary)
                            .role(button::Role::link(Route::ChatModeration.as_str()))
                            .build(),
                        chat::Mode::DemoOnly => button::Button::builder()
                            .label(Text::from("Sign in to interact"))
                            .variant(button::Variant::Secondary)
                            .role(button::Role::link(Route::Login.as_str()))
                            .build(),
                    })
                    .meta(SectionHeaderMetaText::builder()
                        .text(Text::from(format!("Room: {}", self.room_name)))
                        .build())
                    .build())
                (chat::Connection::builder()
                    .connected_signal(Text::from("$sseConnected"))
                    .build())
                (chat::panel::Set::builder()
                    .messages(self.messages.clone())
                    .interactivity(self.interactivity)
                    .build())
                script src="/static/chat-demo.js" {}
            }
        }
    }
}
