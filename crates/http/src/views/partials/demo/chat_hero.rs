use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct ChatHero {
    pub room_name: Text,
    pub room_id: Text,
}

impl Render for ChatHero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="hero" {
                div {
                    h1 { "Live chat room" }
                    p { "Enterprise demo: persistence, rate limits, moderation, and SSE fanout." }
                }
                aside class="hero-card" {
                    h3 { "Room" }
                    p { (self.room_name.to_string()) }
                    p class="muted" { "Room id: " (self.room_id.to_string()) }
                    a class="button secondary" href=(Route::ChatModeration.as_str()) {
                        "Moderation queue"
                    }
                }
            }
        }
    }
}
