use bon::Builder;
use maud::Render;
use maud_extensions::inline_js;
use serde_json::json;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::{
    SectionHeader, SectionHeaderActionLink, SectionHeaderMetaText, chat,
};

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub room_id: Text,
    pub room_name: Text,
    pub messages: Vec<chat::Message>,
    #[builder(default)]
    pub interactivity: chat::Mode,
}

inline_js! {
    (() => {
      const root = document.getElementById("chat-demo");
      if (!root) return;

      const windows = root.querySelectorAll("[data-chat-window]");
      windows.forEach((windowElement) => {
        const list = windowElement.querySelector("[data-chat-messages]");
        if (!list) return;

        const scrollToLatest = () => {
          list.scrollTop = 0;
        };

        requestAnimationFrame(scrollToLatest);
        const observer = new MutationObserver(scrollToLatest);
        observer.observe(list, { childList: true });
      });
    })();
}

impl DemoSection {
    pub const ANCHOR_ID: &'static str = "chat-demo";
}

impl Render for DemoSection {
    fn render(&self) -> maud::Markup {
        let subtitle = match self.interactivity {
            chat::Mode::Interactive => {
                Text::from("Send messages as yourself or the demo user and watch SSE fanout.")
            }
            chat::Mode::DemoOnly => {
                Text::from("Try live posts as the demo user. Sign in to send as yourself.")
            }
        };
        maud::html! {
            section
                id=(Self::ANCHOR_ID)
                data-chat-surface
                data-signals=(json!({
                    "roomId": self.room_id.to_string(),
                    "body": "",
                    "botBody": "",
                    "sseConnected": false
                }).to_string()) {
                (SectionHeader::builder()
                    .title(Text::from("Live chat room"))
                    .subtitle(subtitle)
                    .action(match self.interactivity {
                        chat::Mode::Interactive => SectionHeaderActionLink::builder()
                            .label(Text::from("Moderation queue"))
                            .href(Text::from(Route::ChatModeration.as_str()))
                            .secondary(true)
                            .build(),
                        chat::Mode::DemoOnly => SectionHeaderActionLink::builder()
                            .label(Text::from("Sign in to interact"))
                            .href(Text::from(Route::Login.as_str()))
                            .secondary(true)
                            .build(),
                    })
                    .meta(SectionHeaderMetaText::builder()
                        .text(Text::from(format!("Room: {}", self.room_name)))
                        .build())
                    .build())
                (chat::Connection::builder()
                    .connected_signal(Text::from("$sseConnected"))
                    .build())
                div data-chat-columns {
                    (chat::Panel::builder()
                        .role(chat::Role::You)
                        .messages(self.messages.clone())
                        .interactivity(self.interactivity)
                        .build())
                    (chat::Panel::builder()
                        .role(chat::Role::Demo)
                        .messages(self.messages.clone())
                        .interactivity(self.interactivity)
                        .build())
                }
                (js())
                (chat::Styles.render())
            }
        }
    }
}
