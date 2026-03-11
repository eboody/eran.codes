use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::components::Pill;
use crate::views::partials::button;

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  align-items: start;
  padding: clamp(1.55rem, 1.15rem + 1.2vw, 2.45rem);
  border-radius: var(--radius-shell);
  border: 1px solid color-mix(in srgb, var(--accent-signal) 12%, var(--border-default));
  background:
    linear-gradient(
      135deg,
      color-mix(in srgb, var(--accent-signal) 11%, transparent),
      transparent 56%
    ),
    linear-gradient(
      180deg,
      color-mix(in srgb, white 34%, transparent),
      transparent 42%
    ),
    var(--surface-shell);
  box-shadow: var(--shadow-shell);
  view-transition-name: lab-hero;
}

me [data-home-hero-copy] {
  display: grid;
  gap: var(--space-4);
  align-content: start;
  min-width: 0;
  padding-inline-end: clamp(0rem, 0.3rem + 0.8vw, 1rem);
}

me [data-home-hero-kicker] {
  margin: 0;
  font-size: 0.74rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: var(--accent-warm);
}

me p {
  margin: 0;
  max-width: 55ch;
  font-size: clamp(1rem, 0.95rem + 0.25vw, 1.1rem);
  color: color-mix(in srgb, var(--text-body) 90%, var(--text-muted) 10%);
}

me h1 {
  margin: 0;
  max-width: 11ch;
  font-size: clamp(2.7rem, 1.9rem + 3vw, 4.2rem);
  line-height: 0.92;
  text-wrap: balance;
}

me [data-home-hero-tags] {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem 0.55rem;
  margin-top: 0;
}

me [data-home-hero-tags] .ui-pill {
  color: var(--text-body);
  background: color-mix(in srgb, var(--surface-field) 78%, transparent);
}

me [data-home-hero-card] {
  display: grid;
  gap: var(--space-3);
  grid-template-rows: auto auto auto 1fr auto;
  align-content: start;
  inline-size: min(100%, 22rem);
  padding: clamp(1.2rem, 0.95rem + 0.8vw, 1.75rem);
  border-radius: calc(var(--radius-card) - 2px);
  border: 1px solid color-mix(in srgb, var(--accent-warm) 14%, var(--border-default));
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-warm-soft) 36%, transparent),
      transparent 56%
    ),
    var(--surface-raised);
  box-shadow: inset 0 1px 0 var(--surface-edge-default);
}

me [data-home-hero-card] h3 {
  margin: 0;
  font-size: 0.74rem;
  font-weight: 700;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-home-hero-card] .u-muted {
  color: color-mix(in srgb, var(--text-muted) 94%, var(--text-body) 6%);
}

me [data-button-row] {
  width: fit-content;
  margin-top: var(--space-1);
  padding-top: var(--space-3);
  border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
}

@media (min-width: 62rem) {
  me {
    grid-template-columns: minmax(0, 1.45fr) minmax(18rem, 0.8fr);
  }

  me [data-home-hero-card] {
    justify-self: end;
  }
}

@media (prefers-color-scheme: dark) {
  me {
    border-color: color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
    background:
      radial-gradient(
        circle at 16% 0%,
        color-mix(in srgb, var(--accent-signal) 18%, transparent),
        transparent 44%
      ),
      linear-gradient(180deg, var(--surface-wash-top), transparent 38%),
      color-mix(in srgb, var(--surface-shell) 94%, black 6%);
  }

  me [data-home-hero-card] {
    border-color: color-mix(in srgb, var(--accent-warm) 22%, var(--border-default));
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 36%),
      radial-gradient(
        circle at 100% 0%,
        color-mix(in srgb, var(--accent-warm) 14%, transparent),
        transparent 62%
      ),
      color-mix(in srgb, var(--surface-raised) 92%, black 8%);
    box-shadow:
      inset 0 1px 0 var(--surface-edge-default),
      0 12px 20px color-mix(in srgb, black 24%, transparent);
  }
}

@media (max-width: 48rem) {
  me {
    padding: var(--space-card);
    gap: var(--size-5);
  }

  me [data-home-hero-copy] {
    gap: var(--space-3);
    padding-inline-end: 0;
  }

  me h1 {
    max-width: 9ch;
    font-size: clamp(2.15rem, 1.55rem + 3.4vw, 3rem);
  }

  me p {
    font-size: 0.98rem;
  }

  me [data-home-hero-card] {
    inline-size: 100%;
  }

  me [data-button-row] {
    width: 100%;
    padding-top: var(--space-2);
  }

  me [data-button-row] > * {
    flex: 1 1 12rem;
  }

  me [data-button-row] :where(a.button, button, .button) {
    width: 100%;
  }
}

@media (max-width: 36rem) {
  me h1 {
    max-width: 8ch;
  }

  me [data-button-row] > * {
    flex-basis: 100%;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header id="home-hero" data-home-hero {
                (css())
                div data-home-hero-copy {
                    p data-home-hero-kicker { "Live Lab" }
                    h1 { "Production Rust Systems, Demonstrated Live" }
                    p {
                        "I build secure, observable backend systems with typed boundaries. This portfolio runs on the same real auth, Postgres, and SSE stack I ship in production-style projects."
                    }
                    div data-home-hero-tags {
                        (Pill::builder().text(Text::from("axum-login")).build())
                        (Pill::builder().text(Text::from("tower-sessions")).build())
                        (Pill::builder().text(Text::from("sqlx + postgres")).build())
                        (Pill::builder().text(Text::from("datastar + sse")).build())
                        (Pill::builder().text(Text::from("argon2")).build())
                    }
                    (button::Row::builder()
                        .items(vec![
                            button::Button::builder()
                                .label(Text::from("Open live demo"))
                                .variant(button::Variant::Primary)
                                .role(button::Role::link("#chat-demo"))
                                .build(),
                            button::Button::builder()
                                .label(Text::from("Review engineering quality"))
                                .variant(button::Variant::Secondary)
                                .role(button::Role::link("#engineering-quality"))
                                .build(),
                        ])
                        .build())
                }
                aside data-home-hero-card {
                    h3 { "Session status" }
                    @if let Some(user) = &self.user {
                        p { "Signed in as " strong { (&user.username) } "." }
                        p class="u-muted" { (&user.email) }
                        (button::Button::builder()
                            .label(Text::from("Open account"))
                            .variant(button::Variant::Primary)
                            .role(button::Role::link(Route::Protected.as_str()))
                            .build())
                    } @else {
                        p { "No active session." }
                        p class="u-muted" { "Create an account to see session-backed auth." }
                        (button::Row::builder()
                            .items(vec![
                                button::Button::builder()
                                    .label(Text::from("Create account"))
                                    .variant(button::Variant::Primary)
                                    .role(button::Role::link(Route::Register.as_str()))
                                    .build(),
                                button::Button::builder()
                                    .label(Text::from("Sign in"))
                                    .variant(button::Variant::Secondary)
                                    .role(button::Role::link(Route::Login.as_str()))
                                    .build(),
                            ])
                            .build())
                    }
                }
            }
        }
    }
}
