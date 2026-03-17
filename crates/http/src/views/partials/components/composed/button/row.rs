use bon::Builder;
use maud::Render;

use super::Button;

crate::views::scoped::inline_css!(
    r#"
me {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
  position: relative;
  isolation: isolate;
  padding: var(--interactive-bleed);
  margin-inline: calc(var(--interactive-bleed) * -1);
  margin-block: calc(1.1rem - var(--interactive-bleed)) calc(var(--interactive-bleed) * -1);
  overflow: visible;
}

me > :where(button, a)[data-button] {
  min-inline-size: 10rem;
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct Row {
    pub items: Vec<Button>,
}

impl Render for Row {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-button-row" data-button-row {
                (css())
                @for item in &self.items {
                    (item)
                }
            }
        }
    }
}
