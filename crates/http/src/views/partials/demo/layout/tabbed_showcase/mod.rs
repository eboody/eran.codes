// ci: descriptive-module-import crate::views::partials::demo::layout::tabbed_showcase
mod panels;
mod render;
mod showcase;
mod styles;

use panels::Component as PanelsComponent;
use showcase::Component;
use styles::Styles;

pub(crate) use crate::views::theme::Theme;
pub(crate) use panels::{Action, MockPanel, Panel, Row};
pub(crate) use showcase::ComponentBuilder;

pub(crate) fn builder() -> ComponentBuilder {
    showcase::Component::builder()
}
