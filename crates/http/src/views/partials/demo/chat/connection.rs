use crate::types::Text;
use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct Connection {
    pub connected_signal: Text,
}

impl Render for Connection {
    fn render(&self) -> maud::Markup {
        let connected = &self.connected_signal;
        maud::html! {
            div class="ui-chat-connection-row" data-pill-row {
                span class="pill" data-chat-connection-state="connected" data-show=(connected) style="display:none;" {
                    "SSE connected"
                }
                span class="pill" data-chat-connection-state="disconnected" data-show=(format!("!{}", connected)) { "SSE disconnected" }
            }
        }
    }
}
