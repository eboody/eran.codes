use bon::Builder;
use maud::Render;

use crate::views::partials::EmptyState;

#[derive(Clone, Debug, Builder)]
pub struct DemoResultPlaceholder {
    pub target_id: String,
    pub message: String,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id=(self.target_id.as_str()) class="demo-result muted" {
                (EmptyState::builder()
                    .message(self.message.clone())
                    .build()
                    .render())
            }
        }
    }
}
