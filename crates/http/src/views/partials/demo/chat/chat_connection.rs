use bon::Builder;
use maud::Render;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct ChatConnection {
    pub connected_signal: Text,
}

impl Render for ChatConnection {
    fn render(&self) -> maud::Markup {
        let connected = self.connected_signal.to_string();
        maud::html! {
            div class="pill-row" {
                span class="pill secondary" data-show=(connected) { "SSE connected" }
                span class="pill muted" data-show=(format!("!{}", connected)) { "SSE disconnected" }
            }
        }
    }
}
