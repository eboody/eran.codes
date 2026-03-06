use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::components::Pill;
use crate::views::partials::CtaRow;

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header id="home-hero" class="ui-home-hero" {
                div {
                    h1 { "Production Rust Systems, Demonstrated Live" }
                    p {
                        "I build secure, observable backend systems with typed boundaries. This portfolio runs on the same real auth, Postgres, and SSE stack I ship in production-style projects."
                    }
                    div class="ui-home-hero-tags" {
                        (Pill::builder().text(Text::from("axum-login")).build())
                        (Pill::builder().text(Text::from("tower-sessions")).build())
                        (Pill::builder().text(Text::from("sqlx + postgres")).build())
                        (Pill::builder().text(Text::from("datastar + sse")).build())
                        (Pill::builder().text(Text::from("argon2")).build())
                    }
                    (CtaRow::builder()
                        .items(vec![
                            maud::html! { a class="button" href="#chat-demo" { "Open live demo" } },
                            maud::html! { a class="button secondary" href="#engineering-quality" { "Review engineering quality" } },
                        ])
                        .build())
                }
                aside class="ui-home-hero-card" {
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
        }
    }
}
