use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct Connection {
    pub connected_signal: Text,
}

impl Render for Connection {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-chat-connection-row data-pill-row {
                span
                    class="ui-pill"
                    data-chat-connection-state="connected"
                    data-show=(&self.connected_signal)
                    style="display:none;"
                {
                    "SSE connected"
                }
                span
                    class="ui-pill"
                    data-chat-connection-state="disconnected"
                    data-show=(format!("!{}", self.connected_signal))
                {
                    "SSE disconnected"
                }
            }
        }
    }
}
