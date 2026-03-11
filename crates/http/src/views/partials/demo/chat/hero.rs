use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::button;

crate::views::scoped::inline_css!(
    r#"
me {
  overflow: visible;
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

me .button {
  position: relative;
  z-index: 0;
}

me .button:hover,
me .button:focus-visible {
  z-index: 1;
}

me h1 {
  margin-bottom: 0.25rem;
  font-size: clamp(1.9rem, 1.45rem + 2vw, 2.7rem);
  line-height: 1.1;
}

me p {
  margin: 0.3rem 0 0;
}

me [data-chat-hero-card] {
  overflow: visible;
  background: color-mix(in srgb, var(--ui-surface-card) 90%, transparent);
  inline-size: min(100%, 21rem);
  padding: 1rem;
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--ui-border-soft);
}

me [data-chat-hero-card] h3 {
  margin-bottom: 0.45rem;
}

me [data-chat-hero-card] .u-muted {
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

@media (min-width: 900px) {
  me {
    grid-template-columns: 1.25fr 0.75fr;
  }

  me [data-chat-hero-card] {
    justify-self: end;
  }
}

@media (max-width: 768px) {
  me {
    padding: 1rem;
    border-radius: 16px;
    gap: 1rem;
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
            header data-chat-hero {
                (css())
                div {
                    h1 { "Live chat room" }
                    p { "Enterprise demo: persistence, rate limits, moderation, and SSE fanout." }
                }
                aside data-chat-hero-card {
                    h3 { "Room" }
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
