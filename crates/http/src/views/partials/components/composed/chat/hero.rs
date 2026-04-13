mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::components::{SectionHeader, SectionHeaderLevel, button};

#[derive(Clone, Debug, Builder)]
pub struct Hero {
    pub room_name: Text,
    pub room_id: Text,
}

impl Render for Hero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="u-surface-card" data-chat-hero {
                (styles::render())
                div data-chat-hero-copy {
                    (SectionHeader::builder()
                        .title(Text::from("Live chat room"))
                        .subtitle(Text::from(
                            "Enterprise demo: persistence, rate limits, moderation, and SSE fanout.",
                        ))
                        .level(SectionHeaderLevel::H1)
                        .build())
                }
                aside class="u-inset-card" data-chat-hero-card {
                    p data-chat-hero-card-title { "Room" }
                    p { (&self.room_name) }
                    p class="u-muted" { "Room id: " (&self.room_id) }
                    (button::Button::builder()
                        .label(Text::from("Moderation queue"))
                        .variant(button::Variant::Secondary)
                        .role(button::Role::link(Route::ChatModeration.as_str()))
                        .build())
                }
            }
        }
    }
}
