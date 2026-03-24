use bon::Builder;
use maud::Render;

use super::Button;

#[derive(Clone, Copy, Debug, Default)]
pub enum RowDensity {
    #[default]
    Default,
    Compact,
}

impl RowDensity {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum RowNarrowLayout {
    #[default]
    AutoGrid,
    Stack,
}

impl RowNarrowLayout {
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
    pub density: RowDensity,
    #[builder(default)]
    pub narrow_layout: RowNarrowLayout,
}

impl Render for Row {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div
                class="ui-button-row"
                data-button-row
                data-button-row-density=(self.density.as_attr())
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
            .density(RowDensity::Compact)
            .narrow_layout(RowNarrowLayout::Stack)
            .items(vec![Button::builder().label(Text::from("Inspect")).build()])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-button-row"));
        assert!(markup.contains("data-button-row-density=\"compact\""));
        assert!(markup.contains("data-button-row-narrow=\"stack\""));
    }
}
