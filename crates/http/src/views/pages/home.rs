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
                    h2 { "Live demos" }
                    div class="grid demos" {
                        article {
                            h3 { "SSE ping" }
                            div id="ping-target" {
                                p { "No pings yet." }
                            }
                            button data-on:click="@get('/partials/ping')" { "Ping" }
                        }
                        article data-signals="{surrealMessage: 'Ready.', originalSurrealMessage: 'Ready.', surrealStatus: 'idle'}" {
                            h3 { "Datastar signals" }
                            p data-text="$surrealMessage" {}
                            small data-text="$surrealStatus" {}
                            div class="grid" {
                                button
                                    data-on:click="$surrealMessage = 'Front-end says hi!'; setTimeout(() => { $surrealMessage = $originalSurrealMessage; }, 1000)"
                                { "Front-end update" }
                                button data-on:click="@get('/partials/surreal-message-guarded')" {
                                    "Backend guarded"
                                }
                                button data-on:click="@get('/partials/surreal-message-cancel')" {
                                    "Backend cancel"
                                }
                            }
                        }
                        article {
                            h3 { "Error surface" }
                            p class="muted" { "Centralized error handling with HTML fallbacks." }
                            button class="secondary" data-on:click="@get('/error-test')" {
                                "Trigger error"
                            }
                        }
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
