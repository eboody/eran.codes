use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Pill;

#[derive(Clone, Debug, Builder)]
pub struct EventRow {
    pub timestamp: Text,
    pub message: Text,
    #[builder(default)]
    pub pills: Vec<Pill>,
}

impl Render for EventRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li class="ui-log-entry" data-log-entry {
                span class="u-muted" data-log-timestamp { (&self.timestamp) }
                @for pill in &self.pills {
                    (pill)
                }
                span data-log-message { (&self.message) }
            }
        }
    }
}
