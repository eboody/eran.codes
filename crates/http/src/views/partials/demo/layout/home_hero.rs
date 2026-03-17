use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  --control-font-size: var(--text-size-body-lg);
  --control-radius: var(--radius-pill);

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
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-widest);
  text-transform: uppercase;
  color: var(--accent-warm);
}

me p {
  margin: 0;
  max-width: 55ch;
  font-size: var(--text-size-lead);
  color: color-mix(in srgb, var(--text-body) 90%, var(--text-muted) 10%);
}

me h1 {
  margin: 0;
  max-width: 11ch;
  font-size: var(--text-size-display-lg);
  line-height: var(--text-line-tightest);
  text-wrap: balance;
}

me [data-home-hero-tags] {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
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
  border-radius: var(--ui-radius-md);
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
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wider);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-home-hero-card] .u-muted {
  color: color-mix(in srgb, var(--text-muted) 94%, var(--text-body) 6%);
}

me [data-home-hero-copy] [data-button-row] {
  width: fit-content;
  margin-top: var(--space-1);
  padding-top: var(--space-3);
  border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
}

me [data-home-hero-card] [data-button-row] {
  width: 100%;
  margin: var(--space-2) 0 0 0;
  padding: var(--space-2);
  border: 1px solid color-mix(in srgb, var(--accent-warm) 16%, var(--border-subtle));
  border-radius: var(--ui-radius-md);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, white 26%, transparent),
      transparent 60%
    ),
    color-mix(in srgb, var(--surface-field) 76%, var(--surface-raised));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me [data-home-hero-card] [data-button-row] > * {
  flex: 1 1 10rem;
}

me [data-home-hero-card] [data-button-row] :where(a.button, button, .button) {
  width: 100%;
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

  me [data-home-hero-card] [data-button-row] {
    border-color: color-mix(in srgb, var(--accent-warm) 24%, var(--border-default));
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 46%),
      color-mix(in srgb, var(--surface-field) 92%, black 8%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}

@media (max-width: 48rem) {
  me {
    padding: var(--space-card);
    gap: var(--space-5);
  }

  me [data-home-hero-copy] {
    gap: var(--space-3);
    padding-inline-end: 0;
  }

  me h1 {
    font-size: var(--text-size-display-md);
    max-width: 9ch;
  }

  me p {
    font-size: var(--text-size-body-lg);
  }

  me [data-home-hero-card] {
    inline-size: 100%;
  }

  me [data-home-hero-copy] [data-button-row] {
    width: 100%;
    padding-top: var(--space-2);
  }

  me [data-home-hero-copy] [data-button-row] > * {
    flex: 1 1 12rem;
  }

  me [data-home-hero-copy] [data-button-row] :where(a.button, button, .button) {
    width: 100%;
  }
}

@media (max-width: 36rem) {
  me h1 {
    max-width: 8ch;
  }

  me [data-home-hero-copy] [data-button-row] > * {
    flex-basis: 100%;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<page::UserNav>,
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
                        (partials::components::Pill::builder().text(Text::from("axum-login")).build())
                        (partials::components::Pill::builder().text(Text::from("tower-sessions")).build())
                        (partials::components::Pill::builder().text(Text::from("sqlx + postgres")).build())
                        (partials::components::Pill::builder().text(Text::from("datastar + sse")).build())
                        (partials::components::Pill::builder().text(Text::from("argon2")).build())
                    }
                    (partials::button::Row::builder()
                        .items(vec![
                            partials::button::Button::builder()
                                .label(Text::from("Open live demo"))
                                .variant(partials::button::Variant::Primary)
                                .role(partials::button::Role::link("#chat-demo"))
                                .build(),
                            partials::button::Button::builder()
                                .label(Text::from("Review engineering quality"))
                                .variant(partials::button::Variant::Secondary)
                                .role(partials::button::Role::link("#engineering-quality"))
                                .build(),
                        ])
                        .build())
                }
                aside data-home-hero-card {
                    h3 { "Session status" }
                    @if let Some(user) = &self.user {
                        p { "Signed in as " strong { (&user.username) } "." }
                        p class="u-muted" { (&user.email) }
                        (partials::button::Row::builder()
                            .items(vec![
                                partials::button::Button::builder()
                                    .label(Text::from("Open account"))
                                    .variant(partials::button::Variant::Primary)
                                    .role(partials::button::Role::link(Route::Protected.as_str()))
                                    .build(),
                            ])
                            .build())
                    } @else {
                        p { "No active session." }
                        p class="u-muted" { "Create an account to see session-backed auth." }
                        (partials::button::Row::builder()
                            .items(vec![
                                partials::button::Button::builder()
                                    .label(Text::from("Create account"))
                                    .variant(partials::button::Variant::Primary)
                                    .role(partials::button::Role::link(Route::Register.as_str()))
                                    .build(),
                                partials::button::Button::builder()
                                    .label(Text::from("Sign in"))
                                    .variant(partials::button::Variant::Secondary)
                                    .role(partials::button::Role::link(Route::Login.as_str()))
                                    .build(),
                            ])
                            .build())
                    }
                }
            }
        }
    }
}
