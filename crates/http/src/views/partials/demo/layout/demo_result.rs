use bon::Builder;
use maud::Render;

use crate::views::partials::components::EmptyState;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct DemoResultPlaceholder {
    pub target_id: Text,
    pub message: Text,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id=(&self.target_id) class="demo-result muted" {
                (EmptyState::builder()
                    .message(self.message.clone())
                    .build()
                    .render())
            }
        }
    }
}
