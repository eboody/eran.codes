use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::EmptyState;

crate::views::scoped::inline_css!(
    r#"
me {
  margin-top: 1rem;
  padding: 1rem 1.15rem;
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}
"#
);

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct DemoResultPlaceholder {
    pub target_id: Text,
    pub message: Text,
}

impl Render for DemoResultPlaceholder {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div id=(&self.target_id) class="u-muted u-inset-card" data-demo-result {
                (css())
                (EmptyState::builder()
                    .message(self.message.clone())
                    .build())
            }
        }
    }
}
