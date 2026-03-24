use bon::Builder;
use maud::Render;

use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  margin: var(--space-2) 0 0;
  display: grid;
  gap: var(--space-2);
}

me [data-key-value-item] {
  display: grid;
  grid-template-columns: minmax(0, max-content) minmax(0, 1fr);
  align-items: start;
  gap: calc(var(--space-1) * 0.5) var(--space-3);
}

me dt,
me dd {
  margin: 0;
  min-width: 0;
}

me dt {
  font-size: var(--text-size-label-xs);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me dd {
  font-size: var(--text-size-meta-md);
  color: var(--ui-text-muted);
  overflow-wrap: anywhere;
}

@media (max-width: 48rem) {
  me {
    margin-top: var(--space-1);
    gap: calc(var(--space-1) * 1.5);
  }

  me [data-key-value-item] {
    gap: calc(var(--space-1) * 0.5) var(--space-2);
  }

  me dd {
    font-size: var(--text-size-meta-sm);
  }
}

@media (max-width: 26rem) {
  me [data-key-value-item] {
    grid-template-columns: 1fr;
    gap: calc(var(--space-1) * 0.5);
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
            dl data-key-value-list {
                (css())
                @for (label, value) in &self.items {
                    div data-key-value-item {
                        dt { (label) }
                        dd { (value) }
                    }
                }
            } 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_semantic_key_value_markup() {
        let markup = KeyValueList::builder()
            .items(vec![(Text::from("endpoint"), Text::from("/events"))])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<dl"));
        assert!(markup.contains("data-key-value-item"));
        assert!(markup.contains("<dt>endpoint</dt>"));
        assert!(markup.contains("<dd>/events</dd>"));
    }
}
