use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct Hero {
    pub room_name: Text,
    pub room_id: Text,
}

impl Render for Hero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header data-chat-hero {
                div {
                    h1 { "Live chat room" }
                    p { "Enterprise demo: persistence, rate limits, moderation, and SSE fanout." }
                }
                aside data-chat-hero-card {
                    h3 { "Room" }
                    p { (&self.room_name) }
                    p data-muted { "Room id: " (&self.room_id) }
                    a class="button secondary" href=(Route::ChatModeration) { "Moderation queue" }
                }
            }
        }
    }
}
