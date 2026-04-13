use bon::Builder;
use maud::Render;

use super::Button;

const STYLES: &str = r#"
.ui-button-row {
  --button-row-gap: var(--space-3);
  --button-row-padding: var(--interactive-bleed);
  --button-row-margin-inline: calc(var(--button-row-padding) * -1);
  --button-row-margin-block-start: calc(1.1rem - var(--button-row-padding));
  --button-row-margin-block-end: calc(var(--button-row-padding) * -1);
  --button-row-item-min-inline-size: 10rem;
  --button-row-grid-template:
    repeat(auto-fit, minmax(min(100%, var(--button-row-item-min-inline-size)), 1fr));

  display: flex;
  flex-wrap: wrap;
  gap: var(--button-row-gap);
  min-width: 0;
  position: relative;
  isolation: isolate;
  padding: var(--button-row-padding);
  margin-inline: var(--button-row-margin-inline);
  margin-block: var(--button-row-margin-block-start) var(--button-row-margin-block-end);
  overflow: visible;
}

.ui-button-row > :where(button, a)[data-button] {
  min-inline-size: var(--button-row-item-min-inline-size);
}

.ui-button-row[data-button-row-density='compact'] {
  --button-row-gap: var(--space-2);
  --button-row-item-min-inline-size: 9rem;
}

.ui-button-row[data-button-row-frame='contained'] {
  --button-row-padding: 0;
  --button-row-margin-inline: 0;
  --button-row-margin-block-start: 0;
  --button-row-margin-block-end: 0;
}

@media (max-width: 48rem) {
  .ui-button-row {
    --button-row-padding: 0;
    --button-row-margin-inline: 0;
    --button-row-margin-block-start: var(--space-2);
    --button-row-margin-block-end: 0;
    display: grid;
    grid-template-columns: var(--button-row-grid-template);
    align-items: stretch;
  }

  .ui-button-row[data-button-row-narrow='stack'] {
    grid-template-columns: 1fr;
  }

  .ui-button-row > :where(button, a)[data-button] {
    min-inline-size: 0;
    width: 100%;
  }
}

@media (max-width: 26rem) {
  .ui-button-row {
    --button-row-gap: var(--space-2);
    --button-row-item-min-inline-size: min(100%, 8.5rem);
  }

  .ui-button-row[data-button-row-density='compact'] {
    --button-row-item-min-inline-size: min(100%, 7.5rem);
  }
}

@media (max-width: 20rem) {
  .ui-button-row[data-button-row-narrow='stack'] > :where(button, a)[data-button] {
    white-space: normal;
    text-wrap: balance;
  }
}
"#;

pub(super) fn head_styles() -> maud::Markup {
    crate::views::scoped::style(STYLES)
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Density {
    #[default]
    Default,
    Compact,
}

impl Density {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Frame {
    #[default]
    Bleed,
    Contained,
}

impl Frame {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::Bleed => "bleed",
            Self::Contained => "contained",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum NarrowLayout {
    #[default]
    AutoGrid,
    Stack,
}

impl NarrowLayout {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::AutoGrid => "auto-grid",
            Self::Stack => "stack",
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct Row {
    pub items: Vec<Button>,
    #[builder(default)]
    pub density: Density,
    #[builder(default)]
    pub frame: Frame,
    #[builder(default)]
    pub narrow_layout: NarrowLayout,
}

impl Render for Row {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div
                class="ui-button-row"
                data-button-row
                data-button-row-density=(self.density.as_attr())
                data-button-row-frame=(self.frame.as_attr())
                data-button-row-narrow=(self.narrow_layout.as_attr())
            {
                @for item in &self.items {
                    (item)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Text;

    use super::*;

    #[test]
    fn renders_density_and_narrow_layout_contract() {
        let markup = Row::builder()
            .density(Density::Compact)
            .frame(Frame::Contained)
            .narrow_layout(NarrowLayout::Stack)
            .items(vec![Button::builder().label(Text::from("Inspect")).build()])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-button-row"));
        assert!(markup.contains("data-button-row-density=\"compact\""));
        assert!(markup.contains("data-button-row-frame=\"contained\""));
        assert!(markup.contains("data-button-row-narrow=\"stack\""));
    }
}
