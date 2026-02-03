use bon::Builder;
use maud::{PreEscaped, Render};

use crate::paths::Route;
use crate::views::partials::{ChatConnection, ChatPanel, ChatPanelRole};

#[derive(Clone, Debug, Builder)]
pub struct ChatDemoSection {
    pub room_id: String,
    pub room_name: String,
    pub messages: Vec<crate::views::partials::ChatMessage>,
}

impl ChatDemoSection {
    pub const ANCHOR_ID: &'static str = "chat-demo";
}

impl Render for ChatDemoSection {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section id=(Self::ANCHOR_ID)
                class="chat-panel"
                data-signals=(format!(
                    "{{roomId: '{}', body: '', botBody: '', sseConnected: false}}",
                    self.room_id
                )) {
                header class="section-header" {
                    div {
                        h2 { "Live chat room" }
                        p class="muted" {
                            "Send messages as yourself or the demo user and watch SSE fanout."
                        }
                        p class="muted" { "Room: " (&self.room_name) }
                    }
                    a class="button secondary" href=(Route::ChatModeration.as_str()) { "Moderation queue" }
                }
                (ChatConnection::builder()
                    .connected_signal("$sseConnected".to_string())
                    .build()
                    .render())
                div class="chat-columns" {
                    (ChatPanel::builder()
                        .role(ChatPanelRole::You)
                        .messages(self.messages.clone())
                        .build()
                        .render())
                    (ChatPanel::builder()
                        .role(ChatPanelRole::Demo)
                        .messages(self.messages.clone())
                        .build()
                        .render())
                }
                script {
                    (PreEscaped(r#"
(() => {
  const root = document.getElementById('chat-demo');
  if (!root) return;
  const windows = root.querySelectorAll('.chat-window');
  windows.forEach((win) => {
    const list = win.querySelector('.chat-messages');
    if (!list) return;
    const scroll = () => { list.scrollTop = list.scrollHeight; };
    requestAnimationFrame(scroll);
    const obs = new MutationObserver(scroll);
    obs.observe(list, { childList: true });
  });
})();
                    "#))
                }
            }
        }
    }
}
