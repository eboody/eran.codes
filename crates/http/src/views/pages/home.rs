use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{
    CapabilityShowcase, ChatDemoSection, DemoResultPlaceholder, HomeHero,
    ProfessionalismInPracticeTabs, SectionHeader,
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

                section class="proof-first" {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Live Proof, Not Slideware"))
                            .subtitle(
                                Text::from(
                                    "This site is both my portfolio and a working app platform. The sections below are wired to real runtime behavior, not static mockups.",
                                ),
                            )
                            .build()
                            .render()
                    })
                    div class="impact-metrics" {
                        article class="metric-card" {
                            p class="metric-value" { "4 Layers" }
                            p class="metric-label" { "domain -> app -> infra -> http" }
                        }
                        article class="metric-card" {
                            p class="metric-value" { "1 SSE Stream" }
                            p class="metric-label" { "Per visitor session, fanout across tabs" }
                        }
                        article class="metric-card" {
                            p class="metric-value" { "Typed Contracts" }
                            p class="metric-label" { "DTO -> command -> domain -> SQL row" }
                        }
                        article class="metric-card" {
                            p class="metric-value" { "Guardrails On" }
                            p class="metric-label" { "CI checks + centralized error mapping" }
                        }
                    }
                }

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

                section class="operations-surface" {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Operational View"))
                            .subtitle(
                                Text::from(
                                    "Run the chat demo and watch request, DB, and SSE behavior stream in real time.",
                                ),
                            )
                            .build()
                            .render()
                    })
                    div class="operations-grid" {
                        ({
                            DemoResultPlaceholder::builder()
                                .target_id(Text::from("live-log-target"))
                                .message(
                                    Text::from(
                                        "No backend events yet. Trigger a demo action to start streaming.",
                                    ),
                                )
                                .build()
                                .render()
                        })
                        ({
                            DemoResultPlaceholder::builder()
                                .target_id(Text::from("network-log-target"))
                                .message(
                                    Text::from(
                                        "No network events yet. Trigger a demo action to populate this table.",
                                    ),
                                )
                                .build()
                                .render()
                        })
                    }
                }

                section class="selected-work" {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Selected Work"))
                            .subtitle(
                                Text::from(
                                    "Three high-signal slices from this project, each tied to working routes and code.",
                                ),
                            )
                            .build()
                            .render()
                    })
                    div class="selected-work-grid" {
                        article class="selected-work-card" {
                            p class="selected-work-kicker" { "Capstone" }
                            h3 { "Live chat platform" }
                            p class="muted" {
                                "End-to-end message path with persistence, moderation, rate limiting, and SSE fanout."
                            }
                            ul {
                                li { "Routes: /demo/chat/messages, /chat/moderation" }
                                li { "Outcome: real-time multi-client updates from a persisted source of truth" }
                            }
                            a class="button secondary" href="#chat-demo" { "Open live demo" }
                        }
                        article class="selected-work-card" {
                            p class="selected-work-kicker" { "Security" }
                            h3 { "Auth + session durability" }
                            p class="muted" {
                                "axum-login and tower-sessions on top of Postgres-backed storage with encrypted cookies."
                            }
                            ul {
                                li { "Routes: /register, /login, /protected" }
                                li { "Outcome: stable identity context across request and SSE flows" }
                            }
                            a class="button secondary" href=(Route::Register) { "Walk auth flow" }
                        }
                        article class="selected-work-card" {
                            p class="selected-work-kicker" { "Observability" }
                            h3 { "Live + diagnostic traces" }
                            p class="muted" {
                                "Typed log targets/messages separate operational signal from deep diagnostics."
                            }
                            ul {
                                li { "Live panels: backend stream + network table + chat flow" }
                                li { "Outcome: failures are easier to localize without noisy dashboards" }
                            }
                            a class="button secondary" href="#portfolio-showcase" { "View system maps" }
                        }
                    }
                }

                (CapabilityShowcase::builder().build().render())

                (ProfessionalismInPracticeTabs::builder().build().render())
            }
        };

        crate::views::page::Layout::builder()
            .title("Home")
            .content(content)
            .enable_sse(true)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
