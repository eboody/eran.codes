use bon::Builder;
use maud::Render;

use crate::views::partials::components::Pill;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct LogRow {
    pub timestamp: Text,
    pub message: Text,
    #[builder(default)]
    pub pills: Vec<Pill>,
}

impl Render for LogRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li class="log-entry" {
                span class="muted log-timestamp" { (self.timestamp.to_string()) }
                @for pill in &self.pills {
                    (pill.render())
                }
                span class="log-message" { (self.message.to_string()) }
            }
        }
    }
}
