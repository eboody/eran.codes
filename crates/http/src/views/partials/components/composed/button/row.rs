use bon::Builder;
use maud::Render;

use super::Button;

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
