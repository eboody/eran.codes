use bon::Builder;
use maud::Render;
use maud_extensions::css;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::{SseMode, UserNav};
use crate::views::partials::{
    chat, CapabilityShowcase, DemoResultPlaceholder, HomeHero,
    ProfessionalismInPracticeTabs, RequestBurstDemo, SectionHeader,
};

#[derive(Builder)]
pub struct Home {
    pub user: Option<UserNav>,
    pub chat_demo: Option<chat::DemoSection>,
}

#[derive(Clone, Copy, Debug, Default)]
struct HomeStyles;

impl Render for HomeStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ({
                css! {
                    me [data-proof-first],
                    me #chat-demo,
                    me [data-operations-surface],
                    me [data-selected-work] {
                      margin-top: 2.8rem;
                      border: 1px solid var(--portfolio-surface-border);
                      border-radius: 18px;
                      padding: 1.35rem 1.35rem 1.45rem;
                      background: var(--portfolio-surface);
                      box-shadow: 0 6px 16px color-mix(in srgb, black 8%, transparent);
                    }
                    me [data-impact-metrics] {
                      display: grid;
                      gap: 0.85rem;
                      margin-top: 1.15rem;
                    }
                    @media (min-width: 900px) {
                      me [data-impact-metrics] {
                        grid-template-columns: repeat(4, minmax(0, 1fr));
                      }
                    }
                    me [data-impact-metrics] > article {
                      border: 1px solid var(--ui-border-soft);
                      border-radius: var(--ui-radius-md);
                      padding: 0.95rem 1rem;
                      background: var(--ui-surface-soft-alt);
                    }
                    me [data-impact-metrics] > article > p:first-child {
                      margin: 0;
                      font-size: 1.16rem;
                      font-weight: 700;
                      letter-spacing: -0.015rem;
                    }
                    me [data-impact-metrics] > article > p:last-child {
                      margin: 0.26rem 0 0;
                      font-size: 0.84rem;
                      line-height: 1.45;
                      color: var(--ui-text-muted);
                    }
                    me [data-operations-grid] {
                      display: grid;
                      gap: 1rem;
                    }
                    @media (min-width: 980px) {
                      me [data-operations-grid] {
                        grid-template-columns: 1fr 1fr;
                      }
                    }
                    me [data-selected-work-grid] {
                      display: grid;
                      gap: 1rem;
                    }
                    @media (min-width: 980px) {
                      me [data-selected-work-grid] {
                        grid-template-columns: repeat(3, minmax(0, 1fr));
                      }
                    }
                    me [data-selected-work-grid] > article {
                      border: 1px solid var(--ui-border-soft);
                      border-radius: var(--ui-radius-md);
                      padding: 1.05rem;
                      background: var(--ui-surface-soft);
                    }
                    me [data-selected-work-grid] > article h3 {
                      margin-bottom: 0.35rem;
                    }
                    me [data-selected-work-grid] > article > p:first-child {
                      margin: 0 0 0.25rem;
                      font-size: 0.72rem;
                      font-weight: 700;
                      letter-spacing: 0.07rem;
                      text-transform: uppercase;
                      color: var(--ui-text-muted);
                    }
                    me [data-selected-work-grid] > article ul {
                      margin: 0.72rem 0 0.9rem;
                      padding-left: 1rem;
                      font-size: 0.86rem;
                      line-height: 1.48;
                      color: var(--ui-text-muted);
                    }
                    me [data-selected-work-grid] > article [data-muted] {
                      color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
                    }
                    @media (hover: hover) {
                      me [data-selected-work-grid] > article:hover,
                      me [data-impact-metrics] > article:hover {
                        box-shadow: 0 9px 20px color-mix(in srgb, black 10%, transparent);
                      }
                    }
                    @media (prefers-reduced-motion: no-preference) {
                      me [data-proof-first],
                      me #request-burst-demo,
                      me #chat-demo,
                      me [data-operations-surface],
                      me [data-selected-work],
                      me #portfolio-showcase,
                      me #professionalism-practice,
                      me #live-log-target,
                      me #network-log-target {
                        animation: home-enter 480ms ease both;
                      }
                      me #portfolio-showcase {
                        animation-delay: 70ms;
                      }
                      me [data-proof-first] {
                        animation-delay: 90ms;
                      }
                      me #request-burst-demo {
                        animation-delay: 100ms;
                      }
                      me [data-selected-work] {
                        animation-delay: 120ms;
                      }
                      me [data-operations-surface] {
                        animation-delay: 160ms;
                      }
                    }
                    @keyframes home-enter {
                      from {
                        opacity: 0;
                        transform: translateY(8px);
                      }
                      to {
                        opacity: 1;
                        transform: translateY(0);
                      }
                    }
                    @media (max-width: 768px) {
                      me [data-proof-first],
                      me #request-burst-demo,
                      me #chat-demo,
                      me [data-operations-surface],
                      me [data-selected-work],
                      me #portfolio-showcase,
                      me #professionalism-practice {
                        margin-top: 1.8rem;
                        padding: 1rem 0.95rem 1.1rem;
                        border-radius: 16px;
                      }
                      me [data-impact-metrics] > article {
                        padding: 0.8rem 0.85rem;
                      }
                      me [data-impact-metrics] > article > p:first-child {
                        font-size: 1rem;
                      }
                    }
                }
            })
        }
    }
}

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                (HomeStyles.render())
                section
                    id="counter-demo"
                    data-signals="{count: 0, server_count: 0, server_connected: false}" {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Counter Test"))
                            .subtitle(
                                Text::from(
                                    "Quick Datastar command + SSE check. +/- updates local count and server_count updates arrive from SSE patches only.",
                                ),
                            )
                            .build()
                    })
                    div style="display:flex; gap:.75rem; align-items:center; flex-wrap:wrap;" {
                        button
                            class="button secondary"
                            type="button"
                            data-on:click="$count = $count - 1; @post('/api/counter/sync', {delta: -1})" { "-" }
                        button
                            class="button"
                            type="button"
                            data-on:click="$count = $count + 1; @post('/api/counter/sync', {delta: 1})" { "+" }
                        span { "local: " strong data-text="$count" { "0" } }
                        span { "server: " strong data-text="$server_count" { "0" } }
                        span data-text="$server_connected ? 'synced' : 'disconnected'" { "disconnected" }
                    }
                }
                (HomeHero::builder().maybe_user(self.user.clone()).build())

                section data-proof-first {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Live Proof, Not Slideware"))
                            .subtitle(
                                Text::from(
                                    "This site is both my portfolio and a working app platform. The sections below are wired to real runtime behavior, not static mockups.",
                                ),
                            )
                            .build()
                    })
                    div data-impact-metrics {
                        article {
                            p { "4 Layers" }
                            p { "domain -> app -> infra -> http" }
                        }
                        article {
                            p { "1 SSE Stream" }
                            p { "Per visitor session, fanout across tabs" }
                        }
                        article {
                            p { "Typed Contracts" }
                            p { "DTO -> command -> domain -> SQL row" }
                        }
                        article {
                            p { "Guardrails On" }
                            p { "CI checks + centralized error mapping" }
                        }
                    }
                }

                (RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    section id=(chat::DemoSection::ANCHOR_ID) {
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
                        })
                    }
                }

                section data-operations-surface {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Operational View"))
                            .subtitle(
                                Text::from(
                                    "Run the chat demo and watch request, DB, and SSE behavior stream in real time.",
                                ),
                            )
                            .build()
                    })
                    div data-operations-grid {
                        ({
                            DemoResultPlaceholder::builder()
                                .target_id(Text::from("live-log-target"))
                                .message(
                                    Text::from(
                                        "No backend events yet. Trigger a demo action to start streaming.",
                                    ),
                                )
                                .build()
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
                        })
                    }
                }

                section data-selected-work {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Selected Work"))
                            .subtitle(
                                Text::from(
                                    "Three high-signal slices from this project, each tied to working routes and code.",
                                ),
                            )
                            .build()
                    })
                    div data-selected-work-grid {
                        article {
                            p { "Capstone" }
                            h3 { "Live chat platform" }
                            p data-muted {
                                "End-to-end message path with persistence, moderation, rate limiting, and SSE fanout."
                            }
                            ul {
                                li { "Routes: /demo/chat/messages, /demo/chat/moderation" }
                                li { "Outcome: real-time multi-client updates from a persisted source of truth" }
                            }
                            a class="button secondary" href="#chat-demo" { "Open live demo" }
                        }
                        article {
                            p { "Security" }
                            h3 { "Auth + session durability" }
                            p data-muted {
                                "axum-login and tower-sessions on top of Postgres-backed storage with encrypted cookies."
                            }
                            ul {
                                li { "Routes: /register, /login, /protected" }
                                li { "Outcome: stable identity context across request and SSE flows" }
                            }
                            a class="button secondary" href=(Route::Register) { "Walk auth flow" }
                        }
                        article {
                            p { "Observability" }
                            h3 { "Live + diagnostic traces" }
                            p data-muted {
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

                (CapabilityShowcase::builder().build())

                (ProfessionalismInPracticeTabs::builder().build())
            }
        };

        crate::views::page::Layout::builder()
            .title("Home")
            .content(content)
            .sse_mode(SseMode::Enabled)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
