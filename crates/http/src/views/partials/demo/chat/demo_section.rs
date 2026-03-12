use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::{
    SectionHeader, SectionHeaderMetaText, button, components,
};

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<components::chat::Message>,
    #[builder(default)]
    pub mode: components::chat::Mode,
}

impl DemoSection {
    pub const ANCHOR_ID: &'static str = "chat-demo";
}

impl Render for DemoSection {
    fn render(&self) -> maud::Markup {
        let subtitle = match self.mode {
            components::chat::Mode::Interactive => Text::from(
                "Send messages as yourself or the demo user and watch SSE fanout.",
            ),
            components::chat::Mode::DemoOnly => {
                Text::from("Try live posts as the demo user. Sign in to send as yourself.")
            }
        };
        maud::html! {
            section id=(Self::ANCHOR_ID) {
                (SectionHeader::builder()
                    .title(Text::from("Live chat room"))
                    .subtitle(subtitle)
                    .action(match self.mode {
                        components::chat::Mode::Interactive => button::Button::builder()
                            .label(Text::from("Moderation queue"))
                            .variant(button::Variant::Secondary)
                            .role(button::Role::link(Route::ChatModeration.as_str()))
                            .build(),
                        components::chat::Mode::DemoOnly => button::Button::builder()
                            .label(Text::from("Sign in to interact"))
                            .variant(button::Variant::Secondary)
                            .role(button::Role::link(Route::Login.as_str()))
                            .build(),
                    })
                    .meta(SectionHeaderMetaText::builder()
                        .text(Text::from(format!("Room: {}", self.room_name)))
                        .build())
                    .build())
                (components::chat::Surface::builder()
                    .room_id(self.room_id.clone())
                    .messages(self.messages.clone())
                    .mode(self.mode)
                    .variant(components::chat::Variant::Lab)
                    .build())
            }
        }
    }
}
