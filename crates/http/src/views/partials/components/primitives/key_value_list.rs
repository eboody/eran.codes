use bon::Builder;
use maud::Render;

use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  margin: 0.5rem 0 0;
  padding-left: 1rem;
  font-size: 0.82rem;
  color: var(--ui-text-muted);
}

me li {
  margin: 0.2rem 0;
  word-break: break-word;
}
"#
);

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct KeyValueList {
    pub items: Vec<(Text, Text)>,
}

impl Render for KeyValueList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul data-key-value-list {
                (css())
                @for (label, value) in &self.items {
                    li { (label) ": " (value) }
                }
            }
        }
    }
}
