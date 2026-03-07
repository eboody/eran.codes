use maud::Render;

use super::{button::CtaButton, link::CtaLink};

#[derive(Clone, Debug)]
pub enum CtaItem {
    Link(CtaLink),
    Button(CtaButton),
}

impl Render for CtaItem {
    fn render(&self) -> maud::Markup {
        match self {
            Self::Link(link) => link.render(),
            Self::Button(button) => button.render(),
        }
    }
}
