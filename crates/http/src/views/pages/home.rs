use crate::views::page::UserNav;

pub struct Home {
    pub user: Option<UserNav>,
}

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
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
                            a class="button" href="/protected" { "Open account" }
                        } @else {
                            p { "No active session." }
                            p class="muted" { "Create an account to see session-backed auth." }
                            div class="cta-row" {
                                a class="button" href="/register" { "Create account" }
                                a class="button secondary" href="/login" { "Sign in" }
                            }
                        }
                    }
                }

                section {
                    h2 { "Implementation highlights" }
                    div class="grid highlights" {
                        article {
                            h3 { "Auth + sessions" }
                            ul {
                                li { "axum-login with SQLx-backed session store" }
                                li { "Signed session cookies (HTTP-only, SameSite Lax)" }
                                li { "Argon2 password hashing with credentials table" }
                                li { "Session cleanup task for expired records" }
                            }
                        }
                        article {
                            h3 { "Realtime UX" }
                            ul {
                                li { "Single SSE stream per visitor" }
                                li { "Datastar patches for signals + fragments" }
                                li { "Scoped CSS for safe inline styling" }
                                li { "Server-rendered HTML with progressive enhancement" }
                            }
                        }
                        article {
                            h3 { "Architecture + tracing" }
                            ul {
                                li { "Domain/app/infra/http boundaries enforced" }
                                li { "Request spans with request-id and user-id" }
                                li { "Centralized error rendering" }
                                li { "Config-driven wiring in the binary root" }
                            }
                        }
                    }
                }

                section {
                    h2 { "Demo 1: Auth flow walkthrough" }
                    article class="flow-card" {
                        p {
                            "Follow the full flow: "
                            strong { "/register" }
                            " → "
                            strong { "/login" }
                            " → "
                            strong { "/protected" }
                            "."
                        }
                        ul {
                            li { "Redirected access is enforced on protected routes." }
                            li { "Sessions are stored in Postgres via tower-sessions." }
                            li { "Session cookies are signed, HttpOnly, SameSite Lax." }
                            li { "Passwords are hashed with Argon2 in a credentials table." }
                        }
                        div class="cta-row" {
                            a class="button" href="/register" { "Start demo" }
                            a class="button secondary" href="/login" { "Sign in" }
                        }
                    }
                }

                section {
                    h2 { "Demo 2: Persistent session resilience" }
                    article class="flow-card" {
                        p {
                            "Log in, restart the server, and stay authenticated. "
                            "Sessions are stored in Postgres and expire on inactivity."
                        }
                        ol {
                            li { "Sign in or register to create a session." }
                            li { "Restart the server process." }
                            li { "Reload " strong { "/protected" } " and remain signed in." }
                        }
                        p class="muted" {
                            "Session records live in the "
                            strong { "tower_sessions.session" }
                            " table and are cleaned up automatically."
                        }
                    }
                }

                section {
                    h2 { "Demo 3: Architecture boundary map" }
                    article class="flow-card" {
                        p { "Follow a single request through each boundary." }
                        div class="flow-map" {
                            span class="step" { "http::dto::Register" }
                            span class="arrow" { "→" }
                            span class="step" { "app::user::RegisterUser" }
                            span class="arrow" { "→" }
                            span class="step" { "domain::user::{Username, Email}" }
                            span class="arrow" { "→" }
                            span class="step" { "infra::repo::SqlxUserRepository" }
                        }
                        p class="muted" {
                            "Domain types avoid serde/DB concerns; app orchestrates policy; infra owns SQL."
                        }
                    }
                }

                section {
                    h2 { "Demo 4: Error handling showcase" }
                    article class="flow-card" {
                        p { "Errors are mapped centrally into user-facing responses." }
                        ul {
                            li { "Consistent status + message rendering." }
                            li { "HTML page fallback for standard requests." }
                            li { "Datastar-aware partials for interactive flows." }
                        }
                        a class="button secondary" href="/error-test" { "Trigger error" }
                    }
                }

            }
        };

        crate::views::page::Layout {
            title: "Home",
            content,
            user: self.user.clone(),
        }
        .render()
    }
}
