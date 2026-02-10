use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{ChatDemoSection, CtaRow, DemoResultPlaceholder, DemoSection, FlowMap, HighlightsSection, HomeHero, SectionHeader, SupportCard};

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
                    .title(Text::from("Support systems inside the chat demo"))
                    .content(maud::html! {
                        p {
                            "The chat capstone relies on the same foundations as the earlier demos. "
                            "Use these controls to inspect the underlying systems without leaving the page."
                        }
                        div class="grid demos" {
                            (SupportCard::builder()
                                .title(Text::from("Identity + session durability"))
                                .description(Text::from("Auth, sessions, and persistence are the base layer for chat."))
                                .body(vec![
                                    CtaRow::builder()
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
                                        .render(),
                                    DemoResultPlaceholder::builder()
                                        .target_id(Text::from("auth-status-target"))
                                        .message(Text::from("Click “Check auth status” to load live session info."))
                                        .build()
                                        .render(),
                                    DemoResultPlaceholder::builder()
                                        .target_id(Text::from("session-status-target"))
                                        .message(Text::from("Click “Show session details” to load the session id and expiry."))
                                        .build()
                                        .render(),
                                    CtaRow::builder()
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
                                        .render(),
                                    DemoResultPlaceholder::builder()
                                        .target_id(Text::from("db-check-target"))
                                        .message(Text::from("Run a DB lookup to see the query and trace output."))
                                        .build()
                                        .render(),
                                ])
                                .build()
                                .render())
                            (SupportCard::builder()
                                .title(Text::from("Architecture boundaries + error strategy"))
                                .description(Text::from("Follow a single request through each boundary."))
                                .body(vec![
                                    FlowMap::builder()
                                        .steps(vec![
                                            Text::from("http::dto::Register"),
                                            Text::from("app::user::RegisterUser"),
                                            Text::from("domain::user::{Username, Email}"),
                                            Text::from("infra::repo::SqlxUserRepository"),
                                        ])
                                        .build()
                                        .render(),
                                    maud::html! {
                                        p class="muted" { "Domain types avoid serde/DB concerns; app orchestrates policy; infra owns SQL." }
                                    },
                                    CtaRow::builder()
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
                                        .render(),
                                    DemoResultPlaceholder::builder()
                                        .target_id(Text::from("boundary-target"))
                                        .message(Text::from("Run a validation check to see domain constraints in action."))
                                        .build()
                                        .render(),
                                ])
                                .build()
                                .render())
                            (SupportCard::builder()
                                .title(Text::from("Observability + realtime delivery"))
                                .description(Text::from("Every request is wrapped in structured spans and SSE fanout."))
                                .body(vec![
                                    maud::html! {
                                        button class="button secondary" data-on:click=(format!("@get('{}')", Route::PartialRequestMeta.as_str())) {
                                            "Fetch request metadata"
                                        }
                                    },
                                    DemoResultPlaceholder::builder()
                                        .target_id(Text::from("request-meta-target"))
                                        .message(Text::from("Click “Fetch request metadata” to load request ids and timing."))
                                        .build()
                                        .render(),
                                    maud::html! {
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
                                    },
                                ])
                                .build()
                                .render())
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
                        .target_id(Text::from("live-log-target"))
                        .message(Text::from("No events yet. Trigger a demo action to start streaming."))
                        .build()
                        .render())
                }

                section {
                    h2 { "Live network log (SSE)" }
                    p class="muted" {
                        "Server-side request timings emulate a network tab view."
                    }
                    (DemoResultPlaceholder::builder()
                        .target_id(Text::from("network-log-target"))
                        .message(Text::from("No requests yet. Trigger a demo action to populate this table."))
                        .build()
                        .render())
                }

                (DemoSection::builder()
                    .title(Text::from("Demo D: Live Chat System (Capstone)"))
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
                            .title(Text::from("Live chat room"))
                            .subtitle(Text::from("Sign in to send messages and see the chat room."))
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
