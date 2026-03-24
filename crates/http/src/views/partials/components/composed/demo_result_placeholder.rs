use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::EmptyState;

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct DemoResultPlaceholder {
    pub target_id: Text,
    pub message: Text,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id=(&self.target_id) class="u-demo-result-card u-muted u-inset-card" data-demo-result {
                (EmptyState::builder()
                    .message(self.message.clone())
                    .build())
            }
        }
    }
}
