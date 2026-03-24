use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::EmptyState;

use super::ResultCard;

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct DemoResultPlaceholder {
    pub target_id: Text,
    pub message: Text,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        ResultCard::builder()
            .target_id(self.target_id.clone())
            .muted(true)
            .content(
                EmptyState::builder()
                    .message(self.message.clone())
                    .build()
                    .render(),
            )
            .build()
            .render()
    }
}
