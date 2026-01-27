use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct ChatConnection {
    pub connected_signal: String,
}

impl Render for ChatConnection {
    fn render(&self) -> maud::Markup {
        let connected = self.connected_signal.as_str();
        maud::html! {
            div class="pill-row" {
                span class="pill secondary" data-show=(connected) { "SSE connected" }
                span class="pill muted" data-show=(format!("!{}", connected)) { "SSE disconnected" }
            }
        }
    }
}
