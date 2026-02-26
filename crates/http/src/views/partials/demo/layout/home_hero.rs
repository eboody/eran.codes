use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::views::page::UserNav;
use crate::views::partials::CtaRow;

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="hero" {
                div {
                    h1 { "Production Rust Systems, Demonstrated Live" }
                    p {
                        "I build secure, observable backend systems with typed boundaries. This portfolio runs on the same real auth, Postgres, and SSE stack I ship in production-style projects."
                    }
                    div class="hero-tags" {
                        span class="pill" { "axum-login" }
                        span class="pill" { "tower-sessions" }
                        span class="pill" { "sqlx + postgres" }
                        span class="pill" { "datastar + sse" }
                        span class="pill" { "argon2" }
                    }
                    (CtaRow::builder()
                        .items(vec![
                            maud::html! { a class="button" href="#chat-demo" { "Open live demo" } },
                            maud::html! { a class="button secondary" href="#professionalism-practice" { "Review architecture" } },
                        ])
                        .build()
                        .render())
                }
                aside class="hero-card" {
                    h3 { "Session status" }
                    @if let Some(user) = &self.user {
                        p { "Signed in as " strong { (&user.username) } "." }
                        p class="muted" { (&user.email) }
                        a class="button" href=(Route::Protected) { "Open account" }
                    } @else {
                        p { "No active session." }
                        p class="muted" { "Create an account to see session-backed auth." }
                        (CtaRow::builder()
                            .items(vec![
                                maud::html! { a class="button" href=(Route::Register) { "Create account" } },
                                maud::html! { a class="button secondary" href=(Route::Login) { "Sign in" } },
                            ])
                            .build()
                            .render())
                    }
                }
            }
        }
    }
}
