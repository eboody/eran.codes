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
                    h1 { "eran.codes platform" }
                    p { "A production-ready Axum stack with signed sessions, SQLx persistence, and Datastar-powered UI." }
                    div class="hero-tags" {
                        span class="pill" { "axum-login" }
                        span class="pill" { "tower-sessions" }
                        span class="pill" { "sqlx + postgres" }
                        span class="pill" { "datastar + sse" }
                        span class="pill" { "argon2" }
                    }
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
