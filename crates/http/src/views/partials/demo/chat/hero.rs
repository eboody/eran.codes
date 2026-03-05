use bon::Builder;
use maud::Render;
use maud_extensions::css;

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
            ({
                css! {
                    [data-chat-hero] {
                      display: grid;
                      gap: 1.6rem;
                      align-items: center;
                      margin-top: 1.2rem;
                      margin-bottom: 1.6rem;
                      padding: 1.35rem;
                      border-radius: 18px;
                      border: 1px solid var(--portfolio-surface-border);
                      background:
                        linear-gradient(
                          140deg,
                          color-mix(in srgb, var(--portfolio-accent-a) 18%, transparent),
                          transparent 58%
                        ),
                        var(--portfolio-surface);
                      box-shadow: var(--portfolio-shadow);
                    }
                    @media (min-width: 900px) {
                      [data-chat-hero] {
                        grid-template-columns: 1.25fr 0.75fr;
                      }
                    }
                    [data-chat-hero] h1 {
                      margin-bottom: 0.25rem;
                      font-size: clamp(1.9rem, 1.45rem + 2vw, 2.7rem);
                      line-height: 1.1;
                    }
                    [data-chat-hero] p {
                      margin: 0.3rem 0 0;
                    }
                    [data-chat-hero-card] {
                      background: color-mix(in srgb, var(--ui-surface-card) 90%, transparent);
                      padding: 1rem;
                      border-radius: var(--ui-radius-md);
                      border: 1px solid var(--ui-border-soft);
                    }
                    [data-chat-hero-card] h3 {
                      margin-bottom: 0.45rem;
                    }
                    [data-chat-hero-card] [data-muted] {
                      color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
                    }
                    @media (max-width: 768px) {
                      [data-chat-hero] {
                        padding: 1rem;
                        border-radius: 16px;
                        gap: 1rem;
                      }
                    }
                }
            })
        }
    }
}
