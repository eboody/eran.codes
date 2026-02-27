use crate::types::Text;
use bon::Builder;
use maud::Render;
use maud_extensions::css;

#[derive(Clone, Debug, Builder)]
pub struct Connection {
    pub connected_signal: Text,
}

impl Render for Connection {
    fn render(&self) -> maud::Markup {
        let connected = &self.connected_signal;
        maud::html! {
            div data-pill-row {
                span class="pill" data-chat-connection-state="connected" data-show=(connected) { "SSE connected" }
                span class="pill" data-chat-connection-state="disconnected" data-show=(format!("!{}", connected)) { "SSE disconnected" }
            }
            ({
                css! {
                    [data-pill-row] {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      gap: 0.4rem;
                      margin: 0 0 0.8rem;
                    }
                    [data-chat-connection-state="connected"] {
                      border-color: rgba(120, 210, 140, 0.72);
                      color: rgba(150, 220, 160, 0.95);
                    }
                    [data-chat-connection-state="disconnected"] {
                      border-color: rgba(255, 196, 80, 0.68);
                      color: rgba(255, 206, 108, 0.95);
                    }
                }
            })
        }
    }
}
