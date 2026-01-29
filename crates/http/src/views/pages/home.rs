use bon::Builder;

use crate::paths::Route;
use crate::views::page::UserNav;
use crate::views::partials::{DemoResultPlaceholder, DemoSection, HighlightsSection, HomeHero};

#[derive(Builder)]
pub struct Home {
    pub user: Option<UserNav>,
}

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                (HomeHero::builder()
                    .maybe_user(self.user.clone())
                    .build()
                    .render())

                (HighlightsSection::builder()
                    .categories(vec![
                        crate::views::partials::HighlightCategory::AuthSessions,
                        crate::views::partials::HighlightCategory::Realtime,
                        crate::views::partials::HighlightCategory::ArchitectureTracing,
                    ])
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 1: Auth flow walkthrough".to_string())
                    .content(maud::html! {
                        p {
                            "Follow the full flow: "
                            strong { (Route::Register.as_str()) }
                            " → "
                            strong { (Route::Login.as_str()) }
                            " → "
                            strong { (Route::Protected.as_str()) }
                            "."
                        }
                        ul {
                            li { "Redirected access is enforced on protected routes." }
                            li { "Sessions are stored in Postgres via tower-sessions." }
                            li { "Session cookies are signed, HttpOnly, SameSite Lax." }
                            li { "Passwords are hashed with Argon2 in a credentials table." }
                        }
                        div class="cta-row" {
                            a class="button" href=(Route::Register.as_str()) { "Start demo" }
                            a class="button secondary" href=(Route::Login.as_str()) { "Sign in" }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialAuthStatus.as_str())) {
                                "Check auth status"
                            }
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("auth-status-target".to_string())
                            .message("Click “Check auth status” to load live session info.".to_string())
                            .build()
                            .render())
                    })
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 2: Persistent session resilience".to_string())
                    .content(maud::html! {
                        p {
                            "Log in, restart the server, and stay authenticated. "
                            "Sessions are stored in Postgres and expire on inactivity."
                        }
                        ol {
                            li { "Sign in or register to create a session." }
                            li { "Restart the server process." }
                            li { "Reload " strong { (Route::Protected.as_str()) } " and remain signed in." }
                        }
                        p class="muted" {
                            "Session records live in the "
                            strong { "tower_sessions.session" }
                            " table and are cleaned up automatically."
                        }
                        button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialSessionStatus.as_str())) {
                            "Show session details"
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("session-status-target".to_string())
                            .message("Click “Show session details” to load the session id and expiry.".to_string())
                            .build()
                            .render())
                        div class="cta-row" {
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialDbCheck.with_query("email=demo@example.com"))) {
                                "Check demo@example.com"
                            }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialDbCheck.with_query("email=missing@example.com"))) {
                                "Check missing@example.com"
                            }
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("db-check-target".to_string())
                            .message("Run a DB lookup to see the query and trace output.".to_string())
                            .build()
                            .render())
                    })
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 3: Architecture boundary map".to_string())
                    .content(maud::html! {
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
                        div class="cta-row" {
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=valid"))) {
                                "Validate sample input"
                            }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=invalid"))) {
                                "Validate invalid input"
                            }
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("boundary-target".to_string())
                            .message("Run a validation check to see domain constraints in action.".to_string())
                            .build()
                            .render())
                    })
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 4: Error handling showcase".to_string())
                    .content(maud::html! {
                        p { "Errors are mapped centrally into user-facing responses." }
                        ul {
                            li { "Consistent status + message rendering." }
                            li { "HTML page fallback for standard requests." }
                            li { "Datastar-aware partials for interactive flows." }
                        }
                        div class="cta-row" {
                            a class="button secondary" href=(Route::ErrorTest.as_str()) { "Trigger full-page error" }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::ErrorTest.as_str())) {
                                "Trigger Datastar error"
                            }
                        }
                        p class="muted" {
                            "Datastar errors appear in the alert banner above."
                        }
                    })
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 5: Tracing and observability".to_string())
                    .content(maud::html! {
                        p { "Each request is wrapped in structured spans with key context fields." }
                        ul {
                            li { "request_id, session_id, user_id, route, and latency_ms." }
                            li { "User id is injected from the auth middleware." }
                            li { "Structured logs support JSON or pretty output." }
                        }
                        p class="muted" {
                            "Configure with "
                            code { "RUST_LOG" }
                            " and "
                            code { "LOG_FORMAT=json" }
                            " for structured output."
                        }
                        button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialRequestMeta.as_str())) {
                            "Fetch request metadata"
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("request-meta-target".to_string())
                            .message("Click “Fetch request metadata” to load request ids and timing.".to_string())
                            .build()
                            .render())
                    })
                    .build()
                    .render())

                (DemoSection::builder()
                    .title("Demo 6: SSE and Datastar patches".to_string())
                    .content(maud::html! {
                        p { "Live updates stream over a single SSE connection per visitor." }
                        ul {
                            li { "Datastar patches update signals and fragments in place." }
                            li { "Session-scoped SSE handle keyed by signed cookie." }
                            li { "Client keeps one EventSource at /events." }
                        }
                        p class="muted" {
                            "Use the ping and signal demos to observe live updates."
                        }
                        div class="grid demos" data-signals=(format!("{{surrealMessage: '{}', originalSurrealMessage: '{}', surrealStatus: 'idle'}}", "Ready.", "Ready.")) {
                            article {
                                h3 { "SSE ping" }
                                div id="ping-target" {
                                    p { "No pings yet." }
                                }
                                button data-on:click=(format!("@get('{}')", Route::PartialPing.as_str())) { "Ping" }
                            }
                            article {
                                h3 { "Datastar signals" }
                                p data-text="$surrealMessage" {}
                                small data-text="$surrealStatus" {}
                                div class="grid" {
                                    button
                                        data-on:click="$surrealMessage = 'Front-end says hi!'; setTimeout(() => { $surrealMessage = $originalSurrealMessage; }, 1000)"
                                    { "Front-end update" }
                                    button data-on:click=(format!("@get('{}')", Route::PartialSurrealGuarded.as_str())) {
                                        "Backend guarded"
                                    }
                                    button data-on:click=(format!("@get('{}')", Route::PartialSurrealCancel.as_str())) {
                                        "Backend cancel"
                                    }
                                }
                            }
                        }
                    })
                    .build()
                    .render())

                section {
                    h2 { "Live backend log (SSE)" }
                    p class="muted" {
                        "Actions above stream real request, trace, and DB events into this log via SSE."
                    }
                    (DemoResultPlaceholder::builder()
                        .target_id("live-log-target".to_string())
                        .message("No events yet. Trigger a demo action to start streaming.".to_string())
                        .build()
                        .render())
                }

                section {
                    h2 { "Live network log (SSE)" }
                    p class="muted" {
                        "Server-side request timings emulate a network tab view."
                    }
                    (DemoResultPlaceholder::builder()
                        .target_id("network-log-target".to_string())
                        .message("No requests yet. Trigger a demo action to populate this table.".to_string())
                        .build()
                        .render())
                }

                (DemoSection::builder()
                    .title("Demo 9: Live chat room".to_string())
                    .content(maud::html! {
                        p { "Enterprise chat flow with persistence, moderation, and SSE fanout." }
                        ul {
                            li { "Messages are stored in Postgres and reloaded on entry." }
                            li { "Rate limiting + moderation queue are enforced in the app layer." }
                            li { "SSE broadcasts updates to all connected visitors." }
                        }
                        div class="cta-row" {
                            a class="button" href=(Route::Chat.as_str()) { "Open chat demo" }
                        }
                    })
                    .build()
                    .render())

            }
        };

        crate::views::page::Layout::builder()
            .title("Home")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
