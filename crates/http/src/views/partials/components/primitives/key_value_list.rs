use bon::Builder;
use maud::Render;

use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  margin: var(--space-2) 0 0;
  padding-left: var(--space-4);
  font-size: var(--text-size-meta-md);
  color: var(--ui-text-muted);
}

me li {
  margin: var(--space-1) 0;
  word-break: break-word;
}

@media (max-width: 48rem) {
  me {
    margin-top: var(--space-1);
    padding-left: var(--space-3);
    font-size: var(--text-size-meta-sm);
  }

  me li {
    margin: calc(var(--space-1) * 0.75) 0;
  }
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
