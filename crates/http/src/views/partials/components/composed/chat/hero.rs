use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::components::{
    SectionHeader, SectionHeaderLevel, button,
};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-card);
  align-items: start;
  margin-top: 0;
}

me [data-chat-hero-copy] {
  min-width: 0;
}

me [data-chat-hero-copy] > [data-section-header] {
  margin-bottom: 0;
}

me [data-chat-hero-copy] > [data-section-header] h1 {
  font-size: clamp(1.9rem, 1.45rem + 2vw, 2.7rem);
  line-height: 1.04;
}

me [data-chat-hero-copy] > [data-section-header] .u-muted {
  max-width: 54ch;
}

me [data-chat-hero-card] {
  --inset-card-border: var(--ui-border-soft);
  display: grid;
  gap: var(--space-2);
  inline-size: min(100%, 21rem);
  padding: 1rem;
}

me [data-chat-hero-card-title] {
  margin: 0;
  font-size: 0.74rem;
  font-weight: 700;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-chat-hero-card] p {
  margin: 0;
}

me [data-chat-hero-card] .u-muted {
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

@media (min-width: 900px) {
  me {
    grid-template-columns: minmax(0, 1.25fr) minmax(18rem, 0.75fr);
  }

  me [data-chat-hero-card] {
    justify-self: end;
  }
}

@media (max-width: 768px) {
  me {
    gap: var(--space-3);
  }

  me [data-chat-hero-card] .button {
    width: 100%;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct Hero {
    pub room_name: Text,
    pub room_id: Text,
}

impl Render for Hero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="u-surface-card" data-chat-hero {
                (css())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_shared_chat_hero_shell() {
        let markup = Hero::builder()
            .room_name(Text::from("Lobby"))
            .room_id(Text::from("room-1"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("class=\"u-surface-card\""));
        assert!(markup.contains("data-chat-hero"));
        assert!(markup.contains("<h1>Live chat room</h1>"));
        assert!(markup.contains("Room id: room-1"));
        assert!(markup.contains("Moderation queue"));
    }
}
