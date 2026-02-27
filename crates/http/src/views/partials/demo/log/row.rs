use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Pill;

#[derive(Clone, Debug, Builder)]
pub struct Row {
    pub timestamp: Text,
    pub message: Text,
    #[builder(default)]
    pub pills: Vec<Pill>,
}

impl Render for Row {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li data-log-entry {
                span data-muted data-log-timestamp { (&self.timestamp) }
                @for pill in &self.pills {
                    (pill.render())
                }
                span data-log-message { (&self.message) }
            }
        }
    }
}
