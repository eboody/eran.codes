use bon::Builder;

use crate::paths::Route;
use crate::views::page::UserNav;
use crate::views::partials::{ChatDemoSection, CtaRow, DemoResultPlaceholder, DemoSection, HighlightsSection, HomeHero, SectionHeader};

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
                    .title("Support systems inside the chat demo".to_string())
                    .content(maud::html! {
                        p {
                            "The chat capstone relies on the same foundations as the earlier demos. "
                            "Use these controls to inspect the underlying systems without leaving the page."
                        }
                        div class="grid demos" {
                            article {
                                h3 { "Identity + session durability" }
                                p class="muted" {
                                    "Auth, sessions, and persistence are the base layer for chat."
                                }
                                (CtaRow::builder()
                                    .items(vec![
                                        maud::html! { a class="button" href=(Route::Register.as_str()) { "Start demo" } },
                                        maud::html! { a class="button secondary" href=(Route::Login.as_str()) { "Sign in" } },
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialAuthStatus.as_str())) {
                                                "Check auth status"
                                            }
                                        },
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialSessionStatus.as_str())) {
                                                "Show session details"
                                            }
                                        },
                                    ])
                                    .build()
                                    .render())
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
                                (CtaRow::builder()
                                    .items(vec![
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialDbCheck.with_query("email=demo@example.com"))) {
                                                "Check demo@example.com"
                                            }
                                        },
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialDbCheck.with_query("email=missing@example.com"))) {
                                                "Check missing@example.com"
                                            }
                                        },
                                    ])
                                    .build()
                                    .render())
                                (DemoResultPlaceholder::builder()
                                    .target_id("db-check-target".to_string())
                                    .message("Run a DB lookup to see the query and trace output.".to_string())
                                    .build()
                                    .render())
                            }
                            article {
                                h3 { "Architecture boundaries + error strategy" }
                                p class="muted" { "Follow a single request through each boundary." }
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
                                (CtaRow::builder()
                                    .items(vec![
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=valid"))) {
                                                "Validate sample input"
                                            }
                                        },
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialBoundaryCheck.with_query("case=invalid"))) {
                                                "Validate invalid input"
                                            }
                                        },
                                        maud::html! { a class="button secondary" href=(Route::ErrorTest.as_str()) { "Trigger full-page error" } },
                                        maud::html! {
                                            button class="button secondary" data-on:click=(format!("@get('{}')", Route::ErrorTest.as_str())) {
                                                "Trigger Datastar error"
                                            }
                                        },
                                    ])
                                    .build()
                                    .render())
                                (DemoResultPlaceholder::builder()
                                    .target_id("boundary-target".to_string())
                                    .message("Run a validation check to see domain constraints in action.".to_string())
                                    .build()
                                    .render())
                            }
                            article data-signals=(format!("{{surrealMessage: '{}', originalSurrealMessage: '{}', surrealStatus: 'idle'}}", "Ready.", "Ready.")) {
                                h3 { "Observability + realtime delivery" }
                                p class="muted" { "Every request is wrapped in structured spans and SSE fanout." }
                                button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialRequestMeta.as_str())) {
                                    "Fetch request metadata"
                                }
                                (DemoResultPlaceholder::builder()
                                    .target_id("request-meta-target".to_string())
                                    .message("Click “Fetch request metadata” to load request ids and timing.".to_string())
                                    .build()
                                    .render())
                                div class="grid" {
                                    div {
                                        h4 { "SSE ping" }
                                        div id="ping-target" {
                                            p { "No pings yet." }
                                        }
                                        button data-on:click=(format!("@get('{}')", Route::PartialPing.as_str())) { "Ping" }
                                    }
                                    div {
                                        h4 { "Datastar signals" }
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
                    })
                    .build()
                    .render())

                @if let Some(chat_demo) = &self.chat_demo {
                    (chat_demo.render())
                } @else {
                    section id=(ChatDemoSection::ANCHOR_ID) class="chat-panel" {
                        (SectionHeader::builder()
                            .title("Live chat room".to_string())
                            .subtitle("Sign in to send messages and see the chat room.".to_string())
                            .action(maud::html! {
                                a class="button secondary" href=(Route::Login.as_str()) { "Sign in" }
                            })
                            .build()
                            .render())
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
