use bon::Builder;
use maud::Render;
use maud_extensions::inline_css;

use crate::types::Text;
use crate::views::partials::chat;

#[derive(Clone, Debug, Builder)]
pub struct Window {
    pub title: Option<Text>,
    pub side: chat::message::Side,
    pub messages: Vec<chat::Message>,
}

inline_css! {
    me {
      border: 1px solid var(--chat-shell-border);
      border-radius: var(--chat-radius-window);
      background: var(--chat-shell-bg);
      box-shadow: var(--chat-shell-shadow);
      overflow: hidden;
      height: var(--chat-window-height);
      display: grid;
      grid-template-rows: auto minmax(0, 1fr);
    }
    me > header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--chat-space-1);
      margin: 0;
      padding: var(--chat-space-1) var(--chat-space-2);
      border-bottom: 1px solid var(--chat-shell-header-separator);
      background: var(--chat-shell-header-bg);
    }
    me > header > [data-chat-role] {
      font-size: var(--chat-font-label);
      font-weight: 700;
      letter-spacing: 0.04rem;
      text-transform: uppercase;
      color: var(--chat-shell-title);
    }
    me > header > [data-chat-room-state] {
      display: inline-flex;
      align-items: center;
      gap: var(--chat-space-dot-gap);
      font-size: var(--chat-font-micro);
      font-weight: 700;
      letter-spacing: 0.05rem;
      text-transform: uppercase;
      color: var(--chat-shell-live);
    }
    me > header > [data-chat-room-state]::before {
      content: "";
      width: var(--chat-live-dot-size);
      height: var(--chat-live-dot-size);
      border-radius: 999px;
      background: var(--chat-shell-live-dot);
      box-shadow: 0 0 0 var(--chat-live-dot-ring-size) var(--chat-shell-live-ring);
    }
}

impl Render for Window {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-window {
                (css())
                @if let Some(title) = &self.title {
                    header {
                        span data-chat-role { (title) }
                        span data-chat-room-state { "Live" }
                    }
                }
                (chat::message::Messages::builder()
                    .side(self.side)
                    .messages(self.messages.clone())
                    .build())
            }
        }
    }
}
