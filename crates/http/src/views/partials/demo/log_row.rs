use bon::Builder;
use maud::Render;

use crate::views::partials::Pill;

#[derive(Clone, Debug, Builder)]
pub struct LogRow {
    pub timestamp: String,
    pub message: String,
    #[builder(default)]
    pub pills: Vec<Pill>,
}

impl Render for LogRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li class="log-entry" {
                span class="muted log-timestamp" { (&self.timestamp) }
                @for pill in &self.pills {
                    (pill.render())
                }
                span class="log-message" { (&self.message) }
            }
        }
    }
}
