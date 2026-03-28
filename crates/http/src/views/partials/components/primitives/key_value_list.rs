use std::fmt::Write;

use bon::Builder;
use maud::{Escaper, Render};

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
  overflow-wrap: anywhere;
  word-break: break-word;
}

me dd {
  font-size: var(--text-size-meta-md);
  color: var(--ui-text-muted);
  overflow-wrap: anywhere;
}

me[data-key-value-layout='metrics-grid'] {
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 9.5rem), 1fr));
  gap: var(--space-3) var(--space-4);
}

me[data-key-value-layout='metrics-grid'] [data-key-value-item] {
  grid-template-columns: 1fr;
  gap: var(--space-1);
}

me[data-key-value-layout='metrics-grid'] dd {
  font-size: var(--text-size-body-sm);
  color: var(--text-strong);
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

  me[data-key-value-layout='metrics-grid'] {
    gap: var(--space-2);
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 26rem) {
  me [data-key-value-item] {
    grid-template-columns: 1fr;
    gap: calc(var(--space-1) * 0.5);
  }

  me[data-key-value-layout='metrics-grid'] {
    gap: calc(var(--space-1) * 1.5) var(--space-1);
  }
}

@media (max-width: 20rem) {
  me {
    gap: var(--space-1);
  }

  me dt {
    font-size: var(--text-size-label-2xs);
    letter-spacing: var(--text-track-caps-sm);
  }

  me dd {
    font-size: var(--text-size-label-sm);
  }

  me[data-key-value-layout='metrics-grid'] {
    grid-template-columns: 1fr;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct KeyValueList {
    pub items: Vec<KeyValueItem>,
    #[builder(default)]
    pub layout: Layout,
}

#[derive(Clone, Debug, Builder)]
pub struct KeyValueItem {
    pub label: Text,
    pub value: Text,
    #[builder(default)]
    pub value_attrs: Vec<KeyValueValueAttr>,
}

impl KeyValueItem {
    pub fn text(label: impl Into<Text>, value: impl Into<Text>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            value_attrs: Vec::new(),
        }
    }
}

impl From<(Text, Text)> for KeyValueItem {
    fn from((label, value): (Text, Text)) -> Self {
        Self::text(label, value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyValueValueAttr {
    Flag(Text),
}

impl KeyValueValueAttr {
    pub fn flag(name: impl Into<Text>) -> Self {
        Self::Flag(name.into())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Layout {
    #[default]
    Stacked,
    MetricsGrid,
}

impl Render for KeyValueList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            dl
                data-key-value-list
                data-key-value-layout=(match self.layout {
                    Layout::Stacked => "stacked",
                    Layout::MetricsGrid => "metrics-grid",
                })
            {
                (css())
                @for item in &self.items {
                    (item)
                }
            }
        }
    }
}

impl Render for KeyValueItem {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-key-value-item {
                dt { (&self.label) }
                (KeyValueValue {
                    value: &self.value,
                    attrs: &self.value_attrs,
                })
            }
        }
    }
}

struct KeyValueValue<'a> {
    value: &'a Text,
    attrs: &'a [KeyValueValueAttr],
}

impl Render for KeyValueValue<'_> {
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str("<dd");
        write_value_attrs(buffer, self.attrs);
        buffer.push('>');
        let _ = write!(Escaper::new(buffer), "{}", self.value);
        buffer.push_str("</dd>");
    }
}

fn write_value_attrs(buffer: &mut String, attrs: &[KeyValueValueAttr]) {
    for attr in attrs {
        match attr {
            KeyValueValueAttr::Flag(name) => {
                buffer.push(' ');
                let _ = write!(buffer, "{name}");
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
            .items(vec![KeyValueItem::text("endpoint", "/events")])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<dl"));
        assert!(markup.contains("data-key-value-item"));
        assert!(markup.contains("<dt>endpoint</dt>"));
        assert!(markup.contains("<dd>/events</dd>"));
        assert!(markup.contains("data-key-value-layout=\"stacked\""));
    }

    #[test]
    fn renders_value_attrs_on_definition_value() {
        let markup = KeyValueList::builder()
            .items(vec![KeyValueItem::builder()
                .label(Text::from("endpoint"))
                .value(Text::from("/events"))
                .value_attrs(vec![KeyValueValueAttr::flag("data-endpoint-value")])
                .build()])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<dd data-endpoint-value>/events</dd>"));
    }

    #[test]
    fn key_value_item_renders_directly() {
        let markup = KeyValueItem::text("workers", "24").render().into_string();

        assert!(markup.contains("data-key-value-item"));
        assert!(markup.contains("<dt>workers</dt>"));
        assert!(markup.contains("<dd>24</dd>"));
    }

    #[test]
    fn renders_metrics_grid_layout_flag() {
        let markup = KeyValueList::builder()
            .items(vec![KeyValueItem::text("workers", "24")])
            .layout(Layout::MetricsGrid)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-key-value-layout=\"metrics-grid\""));
    }
}
