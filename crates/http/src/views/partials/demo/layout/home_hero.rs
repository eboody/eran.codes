use bon::Builder;
use maud::Render;
use maud_extensions::css;

use crate::paths::Route;
use crate::views::partials::components::Pill;
use crate::views::page::UserNav;
use crate::views::partials::CtaRow;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header data-home-hero {
                div {
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
                    (CtaRow::builder()
                        .items(vec![
                            maud::html! { a class="button" href="#chat-demo" { "Open live demo" } },
                            maud::html! { a class="button secondary" href="#professionalism-practice" { "Review architecture" } },
                        ])
                        .build())
                }
                aside data-home-hero-card {
                    h3 { "Session status" }
                    @if let Some(user) = &self.user {
                        p { "Signed in as " strong { (&user.username) } "." }
                        p data-muted { (&user.email) }
                        a class="button" href=(Route::Protected) { "Open account" }
                    } @else {
                        p { "No active session." }
                        p data-muted { "Create an account to see session-backed auth." }
                        (CtaRow::builder()
                            .items(vec![
                                maud::html! { a class="button" href=(Route::Register) { "Create account" } },
                                maud::html! { a class="button secondary" href=(Route::Login) { "Sign in" } },
                            ])
                            .build())
                    }
                }
            }
            ({
                css! {
                    me [data-home-hero] {
                      display: grid;
                      gap: 2.3rem;
                      align-items: center;
                      margin-top: 1.65rem;
                      margin-bottom: 2.35rem;
                      padding: 1.7rem;
                      border-radius: 20px;
                      border: 1px solid var(--portfolio-surface-border);
                      background:
                        linear-gradient(
                          135deg,
                          color-mix(in srgb, var(--portfolio-accent-a) 26%, transparent),
                          transparent 56%
                        ),
                        var(--portfolio-surface);
                      box-shadow: var(--portfolio-shadow);
                    }
                    @media (min-width: 900px) {
                      me [data-home-hero] {
                        grid-template-columns: 1.35fr 0.65fr;
                      }
                    }
                    me [data-home-hero] p {
                      margin-top: 0.35rem;
                      max-width: 52ch;
                      font-size: 1.03rem;
                      color: color-mix(in srgb, var(--ui-text) 90%, var(--ui-text-muted) 10%);
                    }
                    me [data-home-hero] h1 {
                      margin-bottom: 0.25rem;
                      font-size: clamp(2.1rem, 1.52rem + 2.6vw, 3.25rem);
                      line-height: 1.06;
                    }
                    me [data-home-hero-tags] {
                      display: flex;
                      flex-wrap: wrap;
                      gap: 0.4rem;
                      margin-top: 0.75rem;
                    }
                    me [data-home-hero-card] {
                      background: color-mix(in srgb, var(--ui-surface-card) 90%, transparent);
                      padding: 1.25rem;
                      border-radius: var(--ui-radius-md);
                      border: 1px solid var(--ui-border-soft);
                      box-shadow: inset 0 1px 0 color-mix(in srgb, white 20%, transparent);
                    }
                    me [data-home-hero-card] h3 {
                      margin-bottom: 0.6rem;
                    }
                    me [data-home-hero-card] [data-muted] {
                      color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
                    }
                    @media (max-width: 768px) {
                      me [data-home-hero] {
                        padding: 1.15rem;
                        border-radius: 18px;
                        margin-top: 1rem;
                        margin-bottom: 1.45rem;
                        gap: 1.1rem;
                      }
                      me [data-home-hero] h1 {
                        font-size: clamp(1.75rem, 1.35rem + 3.8vw, 2.4rem);
                      }
                      me [data-home-hero] p {
                        font-size: 0.97rem;
                      }
                    }
                }
            })
        }
    }
}
