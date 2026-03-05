use crate::types::Text;
use bon::Builder;
use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Visible,
    Pending,
    Removed,
}

#[derive(Clone, Copy, Debug, strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug, Builder)]
pub struct Message {
    pub message_id: Text,
    pub author: Text,
    pub timestamp: Text,
    pub body: Text,
    pub status: Status,
}

impl Render for Message {
    fn render(&self) -> maud::Markup {
        let author = self.author.to_string();
        let status = self.status.as_ref();
        let avatar = author
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_else(|| "?".to_string());
        maud::html! {
            li id=(format!("chat-message-{}", self.message_id)) data-chat-message {
                span data-chat-avatar aria-hidden="true" { (avatar) }
                div data-chat-bubble {
                    div data-chat-meta {
                        strong { (author) }
                        span data-chat-timestamp { (&self.timestamp) }
                        span data-chat-status data-chat-status-kind=(status) {
                            (status)
                        }
                    }
                    p data-chat-body { (&self.body) }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Messages {
    pub messages: Vec<Message>,
    pub side: Side,
}

inline_css! {
    me {
      min-height: 0;
      display: flex;
    }
    me > [data-chat-messages] {
      list-style: none;
      margin: 0;
      padding: var(--chat-space-2);
      display: flex;
      flex-direction: column;
      gap: var(--chat-message-gap);
      overflow-y: auto;
      flex: 1;
      min-height: 0;
    }
    me > [data-chat-messages] > [data-chat-message] {
      display: flex;
      align-items: flex-end;
      gap: var(--chat-avatar-gap);
    }
    me > [data-chat-messages][data-chat-side="right"] > [data-chat-message] {
      justify-content: flex-end;
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-avatar] {
      width: var(--chat-avatar-size);
      height: var(--chat-avatar-size);
      border-radius: 999px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      font-size: var(--chat-font-avatar);
      font-weight: 700;
      border: 1px solid var(--chat-avatar-border);
      background: var(--chat-avatar-bg-left);
      color: var(--chat-avatar-fg-left);
    }
    me > [data-chat-messages][data-chat-side="right"] > [data-chat-message] > [data-chat-avatar] {
      order: 2;
      background: var(--chat-avatar-bg-right);
      border-color: var(--chat-avatar-border-right);
      color: var(--chat-avatar-fg-right);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] {
      border: 1px solid var(--chat-bubble-left-border);
      border-radius: var(--chat-bubble-left-radius);
      background: var(--chat-bubble-left-bg);
      color: var(--chat-bubble-left-fg);
      padding: var(--chat-space-1) var(--chat-space-2) var(--chat-space-2);
      min-width: 0;
      max-width: var(--chat-bubble-max);
    }
    me > [data-chat-messages][data-chat-side="right"] > [data-chat-message] > [data-chat-bubble] {
      order: 1;
      border-radius: var(--chat-bubble-right-radius);
      border-color: var(--chat-bubble-right-border);
      background: var(--chat-bubble-right-bg);
      color: var(--chat-bubble-right-fg);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--chat-meta-gap);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > strong {
      font-size: var(--chat-font-label);
      font-weight: 700;
      color: var(--chat-meta-fg-strong);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-timestamp] {
      font-size: var(--chat-font-micro);
      line-height: 1.1;
      color: var(--chat-meta-fg-muted);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status] {
      font-size: var(--chat-font-status);
      line-height: 1.1;
      padding: 0.1rem 0.32rem;
      border-radius: 999px;
      border: 1px solid color-mix(in srgb, var(--ui-text-muted) 26%, transparent);
      color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="visible"] {
      display: none;
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="pending"] {
      border-color: color-mix(in srgb, #f59e0b 38%, transparent);
      color: color-mix(in srgb, #f59e0b 74%, var(--ui-text) 26%);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-meta] > [data-chat-status-kind="removed"] {
      border-color: color-mix(in srgb, #ef4444 38%, transparent);
      color: color-mix(in srgb, #ef4444 74%, var(--ui-text) 26%);
    }
    me > [data-chat-messages] > [data-chat-message] > [data-chat-bubble] > [data-chat-body] {
      margin: var(--chat-message-body-top) 0 0;
      font-size: var(--chat-font-body);
      line-height: var(--chat-line-body);
      overflow-wrap: anywhere;
      color: var(--chat-body-fg);
    }
}

impl Render for Messages {
    fn render(&self) -> maud::Markup {
        let side = self.side.as_ref();
        maud::html! {
            div data-chat-feed {
                (css())
                ul data-chat-messages data-chat-side=(side) {
                    @for message in &self.messages {
                        (message.render())
                    }
                }
            }
        }
    }
}
