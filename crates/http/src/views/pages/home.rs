use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::{SseMode, UserNav};
use crate::views::partials::{
    chat, DemoResultPlaceholder, EngineeringQuality, HomeHero, RequestBurstDemo,
    SectionHeader, TabSetShowcase,
};

#[derive(Builder)]
pub struct Home {
    pub user: Option<UserNav>,
    pub chat_demo: Option<chat::DemoSection>,
}

impl maud::Render for Home {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                (HomeHero::builder().maybe_user(self.user.clone()).build())

                (TabSetShowcase::builder().build())

                (RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    section id=(chat::DemoSection::ANCHOR_ID) class="ui-surface-card" {
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

                section id="operations-surface" class="ui-surface-card" data-operations-surface {
                    ({
                        SectionHeader::builder()
                            .title(Text::from("Operational View"))
                            .subtitle(
                                Text::from(
                                    "Run a demo interaction, then inspect request, DB, and SSE behavior in real time.",
                                ),
                            )
                            .build()
                    })
                    div class="ui-grid-two-column" {
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

                (EngineeringQuality::builder().build())
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
