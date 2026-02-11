use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{
    ChatDemoSection, DemoResultPlaceholder, DemoSection, DiagramPanel, DiagramRow,
    DiagramStatus, FeatureAccent, FeatureCard, FeatureGallery, HomeHero, SectionHeader,
};

#[derive(Builder)]
pub struct Home {
    pub user: Option<UserNav>,
    pub chat_demo: Option<ChatDemoSection>,
}

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                (HomeHero::builder().maybe_user(self.user.clone()).build().render())

                section class="portfolio-principles" {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("How I Think About Systems"))
                            .subtitle(
                                Text::from(
                                    "This portfolio emphasizes engineering judgment: correctness, clarity, and maintainability over surface area.",
                                ),
                            )
                            .build()
                            .render()
                    })
                    div class="principles-grid" {
                        article class="principle-card" {
                            h3 { "Correctness first" }
                            p class="muted" {
                                "Invariants live in domain types, policy lives in app services, and infra owns SQL + hashing."
                            }
                            ul {
                                li { "Typed boundaries and newtypes for critical data" }
                                li { "Explicit error mapping across layers" }
                            }
                        }
                        article class="principle-card" {
                            h3 { "Readable by design" }
                            p class="muted" {
                                "Builder pipelines and componentized Maud views keep call sites self-documenting."
                            }
                            ul {
                                li { "Bon builders with descriptive steps" }
                                li { "Reusable UI components (Render + enums)" }
                            }
                        }
                        article class="principle-card" {
                            h3 { "Maintainable over time" }
                            p class="muted" {
                                "Tracing, linting, and module READMEs turn architectural decisions into guardrails."
                            }
                            ul {
                                li { "Live + diagnostic trace logs" }
                                li { "CI checks for stringy logic + render coverage" }
                            }
                        }
                    }
                }

                section class="portfolio-audit" {
                    ({
                        SectionHeader::builder()
                            .title(
                                Text::from(
                                    "Architecture Audit (What This Site Demonstrates)",
                                ),
                            )
                            .subtitle(
                                Text::from(
                                    "A concise, end-to-end review of storage, representations, interactions, and idiomaticity across the workspace.",
                                ),
                            )
                            .build()
                            .render()
                    })
                    div class="audit-grid" {
                        article class="audit-card" {
                            h3 { "Storage layers" }
                            p class="muted" {
                                "Durable state in Postgres, transient state in SSE + in-memory logs, UI derived via Maud."
                            }
                            ul {
                                li {
                                    "DB: users, credentials, chat rooms/messages, sessions, audit"
                                }
                                li { "In-memory: per-request + per-session trace buffers" }
                                li { "UI: Maud components render typed values" }
                            }
                        }
                        article class="audit-card" {
                            h3 { "Representations in code" }
                            p class="muted" {
                                "Each layer owns its types to enforce invariants and boundaries."
                            }
                            ul {
                                li { "Domain: newtypes + invariants, no serde/HTTP/DB" }
                                li { "App: commands + traits for orchestration" }
                                li { "Infra: SQL rows + concrete hashing" }
                                li { "HTTP: DTOs + Renderable components" }
                            }
                        }
                        article class="audit-card" {
                            h3 { "Interactions" }
                            p class="muted" { "Request flow is explicit, traceable, and layered." }
                            ul {
                                li { "HTTP parses -> App orchestrates -> Infra persists" }
                                li { "SSE fanout for chat messages" }
                                li { "Live vs diagnostic trace logs" }
                            }
                        }
                        article class="audit-card" {
                            h3 { "Idiomaticity & guardrails" }
                            p class="muted" {
                                "The codebase encodes standards as lint, builders, and docs."
                            }
                            ul {
                                li { "Bon builders keep wiring readable" }
                                li { "Render components + enums (no stringly logic)" }
                                li { "CI checks: stringy logic, String fields, Render coverage" }
                                li { "Module READMEs for architecture mapping" }
                            }
                        }
                    }
                }

                ({
                    FeatureGallery::builder()
                        .title(
                            Text::from(
                                "Feature gallery: realtime delivery, grounded in systems",
                            ),
                        )
                        .subtitle(
                            Text::from(
                                "A quick visual scan of the core capabilities, followed by live diagrams that explain how requests and events flow through the stack.",
                            ),
                        )
                        .features(
                            vec![
                                FeatureCard::builder()
                                    .title(Text::from("Realtime chat fanout"))
                                    .description(
                                        Text::from(
                                            "SSE push with durable message storage and moderation gates.",
                                        ),
                                    )
                                    .bullets(
                                        vec![
                                            Text::from("Postgres-backed message history"),
                                            Text::from("Per-room moderation queues"),
                                            Text::from("Single SSE stream per session"),
                                        ],
                                    )
                                    .accent(FeatureAccent::Indigo)
                                    .badge(Text::from("Live"))
                                    .build(),
                                FeatureCard::builder()
                                    .title(Text::from("Session durability"))
                                    .description(
                                        Text::from(
                                            "Signed cookies + DB sessions keep identities consistent.",
                                        ),
                                    )
                                    .bullets(
                                        vec![
                                            Text::from("Encrypted session cookies"),
                                            Text::from("User id attached to tracing"),
                                            Text::from("Session store persistence"),
                                        ],
                                    )
                                    .accent(FeatureAccent::Emerald)
                                    .badge(Text::from("Auth"))
                                    .build(),
                                FeatureCard::builder()
                                    .title(Text::from("Boundary-safe flows"))
                                    .description(
                                        Text::from(
                                            "Typed DTOs, commands, and domain invariants protect policy.",
                                        ),
                                    )
                                    .bullets(
                                        vec![
                                            Text::from("Domain invariants via newtypes"),
                                            Text::from("App commands orchestrate policy"),
                                            Text::from("Infra owns SQL & hashing"),
                                        ],
                                    )
                                    .accent(FeatureAccent::Amber)
                                    .badge(Text::from("Architecture"))
                                    .build(),
                                FeatureCard::builder()
                                    .title(Text::from("Observability woven in"))
                                    .description(
                                        Text::from(
                                            "Spans, trace logs, and live network views in one place.",
                                        ),
                                    )
                                    .bullets(
                                        vec![
                                            Text::from("Structured request spans"),
                                            Text::from("SSE event visibility"),
                                            Text::from("Request grouping by trigger"),
                                        ],
                                    )
                                    .accent(FeatureAccent::Rose)
                                    .badge(Text::from("Tracing"))
                                    .build(),
                            ],
                        )
                        .diagrams(
                            vec![
                                DiagramPanel::builder()
                                    .title(Text::from("Realtime request flow"))
                                    .description(
                                        Text::from("What happens when a user sends a message."),
                                    )
                                    .rows(
                                        vec![
                                            DiagramRow::builder()
                                                .label(Text::from("POST /demo/chat/messages"))
                                                .value(Text::from("Accepted · 202"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("App: rate limit + moderation"))
                                                .value(Text::from("ok · queue=clean"))
                                                .status(DiagramStatus::Info)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("DB write"))
                                                .value(Text::from("chat_messages"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("Rate limit window"))
                                                .value(Text::from("near cap"))
                                                .status(DiagramStatus::Warning)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("SSE broadcast"))
                                                .value(Text::from("append .chat-messages"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                        ],
                                    )
                                    .build(),
                                DiagramPanel::builder()
                                    .title(Text::from("Identity + session durability"))
                                    .description(
                                        Text::from(
                                            "Signed cookies keep a single SSE stream per visitor.",
                                        ),
                                    )
                                    .rows(
                                        vec![
                                            DiagramRow::builder()
                                                .label(Text::from("session_id cookie"))
                                                .value(Text::from("signed · http-only"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("session store"))
                                                .value(Text::from("postgres"))
                                                .status(DiagramStatus::Info)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("sse stream"))
                                                .value(Text::from("one per visitor"))
                                                .status(DiagramStatus::Passive)
                                                .build(),
                                        ],
                                    )
                                    .build(),
                                DiagramPanel::builder()
                                    .title(Text::from("Boundary handoff"))
                                    .description(
                                        Text::from("Each layer owns its responsibilities."),
                                    )
                                    .rows(
                                        vec![
                                            DiagramRow::builder()
                                                .label(Text::from("HTTP DTO"))
                                                .value(Text::from("request parsing"))
                                                .status(DiagramStatus::Info)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("App command"))
                                                .value(Text::from("policy + orchestration"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("Domain types"))
                                                .value(Text::from("invariants"))
                                                .status(DiagramStatus::Active)
                                                .build(),
                                            DiagramRow::builder()
                                                .label(Text::from("Infra repo"))
                                                .value(Text::from("SQL + hashing"))
                                                .status(DiagramStatus::Passive)
                                                .build(),
                                        ],
                                    )
                                    .build(),
                            ],
                        )
                        .build()
                        .render()
                })

                section {
                    h2 { "Live backend log (SSE)" }
                    p class="muted" {
                        "Actions above stream real request, trace, and DB events into this log via SSE."
                    }
                    ({
                        DemoResultPlaceholder::builder()
                            .target_id(Text::from("live-log-target"))
                            .message(
                                Text::from(
                                    "No events yet. Trigger a demo action to start streaming.",
                                ),
                            )
                            .build()
                            .render()
                    })
                }

                section {
                    h2 { "Live network log (SSE)" }
                    p class="muted" { "Server-side request timings emulate a network tab view." }
                    ({
                        DemoResultPlaceholder::builder()
                            .target_id(Text::from("network-log-target"))
                            .message(
                                Text::from(
                                    "No requests yet. Trigger a demo action to populate this table.",
                                ),
                            )
                            .build()
                            .render()
                    })
                }

                ({
                    DemoSection::builder()
                        .title(Text::from("Demo D: Live Chat System (Capstone)"))
                        .content(
                            maud::html! {
                                p {
                                "Enterprise chat flow with persistence, moderation, and SSE fanout."
                                } ul { li {
                                "Messages are stored in Postgres and reloaded on entry." }
                                li {
                                "Rate limiting + moderation queue are enforced in the app layer."
                                } li { "SSE broadcasts updates to all connected visitors." }
                                }
                            },
                        )
                        .build()
                        .render()
                })

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    section id=(ChatDemoSection::ANCHOR_ID) class="chat-panel" {
                        ({
                            SectionHeader::builder()
                                .title(Text::from("Live chat room"))
                                .subtitle(
                                    Text::from(
                                        "Sign in to send messages and see the chat room.",
                                    ),
                                )
                                .action(
                                    maud::html! {
                                        a class = "button secondary" href = (Route::Login) {
                                        "Sign in" }
                                    },
                                )
                                .build()
                                .render()
                        })
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
