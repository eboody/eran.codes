use crate::types::Text;
use bon::Builder;
use maud::Render;

crate::views::scoped::inline_css!(
    r#"
me {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.4rem;
  margin: 0 0 0.8rem;
}

me [data-chat-connection-state='connected'] {
  border-color: rgba(120, 210, 140, 0.72);
  color: rgba(150, 220, 160, 0.95);
}

me [data-chat-connection-state='disconnected'] {
  border-color: rgba(255, 196, 80, 0.68);
  color: rgba(255, 206, 108, 0.95);
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct Connection {
    pub connected_signal: Text,
}

impl Render for Connection {
    fn render(&self) -> maud::Markup {
        let connected = &self.connected_signal;
        maud::html! {
            div data-chat-connection-row data-pill-row {
                (css())
                span class="ui-pill" data-chat-connection-state="connected" data-show=(connected) style="display:none;" {
                    "SSE connected"
                }
                span class="ui-pill" data-chat-connection-state="disconnected" data-show=(format!("!{}", connected)) { "SSE disconnected" }
            }
        }
    }
}
