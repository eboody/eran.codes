use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::views::page::{
    UserNav, PORTFOLIO_CONTACT_URL, PORTFOLIO_GITHUB_URL, PORTFOLIO_LINKEDIN_URL,
    PORTFOLIO_RESUME_URL,
};
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
                    h1 { "Eran Codes: Portfolio + App Platform" }
                    p {
                        "I build production-grade Rust systems with typed boundaries, durable auth sessions, and observable real-time flows."
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
                            maud::html! { a class="button secondary" href="#professionalism-practice" { "See implementation details" } },
                        ])
                        .build()
                        .render())
                    div class="hero-conversion-links" {
                        a href=(PORTFOLIO_RESUME_URL) { "Resume" }
                        a href=(PORTFOLIO_GITHUB_URL) target="_blank" rel="noopener noreferrer" { "GitHub" }
                        a href=(PORTFOLIO_LINKEDIN_URL) target="_blank" rel="noopener noreferrer" { "LinkedIn" }
                        a href=(PORTFOLIO_CONTACT_URL) { "Contact" }
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
