use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<partials::components::chat::Message>,
    #[builder(default)]
    pub mode: partials::components::chat::Mode,
}

impl DemoSection {
    pub const ANCHOR_ID: &'static str = "chat-demo";
}

impl Render for DemoSection {
    fn render(&self) -> maud::Markup {
        let subtitle = match self.mode {
            partials::components::chat::Mode::Interactive => Text::from(
                "Send messages as yourself or the demo user and watch SSE fanout.",
            ),
            partials::components::chat::Mode::DemoOnly => {
                Text::from("Try live posts as the demo user. Sign in to send as yourself.")
            }
        };
        maud::html! {
            section id=(Self::ANCHOR_ID) {
                (partials::SectionHeader::builder()
                    .title(Text::from("Live chat room"))
                    .subtitle(subtitle)
                    .action(match self.mode {
                        partials::components::chat::Mode::Interactive => partials::button::Button::builder()
                            .label(Text::from("Moderation queue"))
                            .variant(partials::button::Variant::Secondary)
                            .role(partials::button::Role::link(Route::ChatModeration.as_str()))
                            .build(),
                        partials::components::chat::Mode::DemoOnly => partials::button::Button::builder()
                            .label(Text::from("Sign in to interact"))
                            .variant(partials::button::Variant::Secondary)
                            .role(partials::button::Role::link(Route::Login.as_str()))
                            .build(),
                    })
                    .meta(partials::SectionHeaderMetaText::builder()
                        .text(Text::from(format!("Room: {}", self.room_name)))
                        .build())
                    .build())
                (partials::components::chat::Surface::builder()
                    .room_id(self.room_id.clone())
                    .messages(self.messages.clone())
                    .mode(self.mode)
                    .variant(partials::components::chat::Variant::Lab)
                    .build())
            }
        }
    }
}
