use bon::Builder;

use crate::paths::Route;
use crate::views::page::UserNav;
use crate::views::partials::{ChatDemoSection, DemoResultPlaceholder, DemoSection, HighlightsSection, HomeHero};

#[derive(Builder)]
pub struct Home {
    pub user: Option<UserNav>,
    pub chat_demo: Option<ChatDemoSection>,
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
                    .title("Demo A: Identity & Session Durability".to_string())
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
                            li { "Migrations define the schema and session tables." }
                        }
                        div class="cta-row" {
                            a class="button" href=(Route::Register.as_str()) { "Start demo" }
                            a class="button secondary" href=(Route::Login.as_str()) { "Sign in" }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialAuthStatus.as_str())) {
                                "Check auth status"
                            }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialSessionStatus.as_str())) {
                                "Show session details"
                            }
                        }
                        (DemoResultPlaceholder::builder()
                            .target_id("auth-status-target".to_string())
                            .message("Click “Check auth status” to load live session info.".to_string())
                            .build()
                            .render())
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
                    .title("Demo B: Architecture Boundaries + Error Strategy".to_string())
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
                        p class="muted" { "Domain types avoid serde/DB concerns; app orchestrates policy; infra owns SQL." }
                        p class="muted" { "Errors are mapped centrally into user-facing responses." }
                        div class="cta-row" {
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=valid"))) {
                                "Validate sample input"
                            }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=invalid"))) {
                                "Validate invalid input"
                            }
                            a class="button secondary" href=(Route::ErrorTest.as_str()) { "Trigger full-page error" }
                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::ErrorTest.as_str())) {
                                "Trigger Datastar error"
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
                    .title("Demo C: Observability + Realtime Delivery".to_string())
                    .content(maud::html! {
                        p { "Each request is wrapped in structured spans with key context fields." }
                        ul {
                            li { "request_id, session_id, user_id, route, and latency_ms." }
                            li { "User id is injected from the auth middleware." }
                            li { "Structured logs support JSON or pretty output." }
                            li { "Single SSE connection per visitor for live updates." }
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
                        p class="muted" { "Use the ping and signal demos to observe live updates." }
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
                    .title("Demo D: Live Chat System (Capstone)".to_string())
                    .content(maud::html! {
                        p { "Enterprise chat flow with persistence, moderation, and SSE fanout." }
                        ul {
                            li { "Messages are stored in Postgres and reloaded on entry." }
                            li { "Rate limiting + moderation queue are enforced in the app layer." }
                            li { "SSE broadcasts updates to all connected visitors." }
                        }
                        div class="cta-row" {
                            a class="button" href=(format!("#{}", ChatDemoSection::ANCHOR_ID)) { "Jump to chat" }
                        }
                    })
                    .build()
                    .render())

                @if let Some(chat_demo) = &self.chat_demo {
                    (chat_demo.render())
                } @else {
                    section id=(ChatDemoSection::ANCHOR_ID) class="chat-panel" {
                        header class="section-header" {
                            div {
                                h2 { "Live chat room" }
                                p class="muted" { "Sign in to send messages and see the chat room." }
                            }
                            a class="button secondary" href=(Route::Login.as_str()) { "Sign in" }
                        }
                    }
                }

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
