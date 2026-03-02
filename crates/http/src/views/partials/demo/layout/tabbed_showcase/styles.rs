use maud::Render;

mod base;
mod panels;
mod responsive;
mod tabs;

use self::{
    base::BaseStyles, panels::PanelStyles, responsive::ResponsiveStyles, tabs::TabStyles,
};

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Styles;

impl Render for Styles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (BaseStyles.render())
            (TabStyles.render())
            (PanelStyles.render())
            (ResponsiveStyles.render())
        }
    }
}
