use maud::Render;

mod base;
mod panels;
mod responsive;

use self::{base::BaseStyles, panels::PanelStyles, responsive::ResponsiveStyles};

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Styles;

impl Render for Styles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (BaseStyles.render())
            (PanelStyles.render())
            (ResponsiveStyles.render())
        }
    }
}
