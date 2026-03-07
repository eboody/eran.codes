use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::{SseMode, UserNav};
use crate::views::partials::{
    DemoResultPlaceholder, EngineeringQuality, HomeHero, RequestBurstDemo, SectionHeader,
    SectionHeaderActionLink, TabSetShowcase, chat,
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
                                .action(SectionHeaderActionLink::builder()
                                    .label(Text::from("Sign in"))
                                    .href(Text::from(Route::Login.as_str()))
                                    .secondary(true)
                                    .build())
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
                                    "Run a demo interaction, then follow request, backend, and SSE behavior in one timeline.",
                                ),
                            )
                            .build()
                    })
                    ({
                        DemoResultPlaceholder::builder()
                            .target_id(Text::from("network-log-target"))
                            .message(
                                Text::from(
                                    "No timeline events yet. Trigger a demo action to populate this view.",
                                ),
                            )
                            .build()
                    })
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
