use maud::Render;

use crate::types::Text;
use crate::views::partials::components::portfolio::content::CmsActionLink;

use super::{SectionActions, Surface};

pub struct ClosingSection<'a> {
    pub title: &'a Text,
    pub summary: &'a Text,
    pub actions: &'a [CmsActionLink],
}

impl Render for ClosingSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                h2 { (&self.title) }
                p class="ui-portfolio-summary" { (&self.summary) }
                (SectionActions {
                    actions: self.actions,
                })
            }).extra_class("ui-portfolio-closing"))
        }
    }
}
