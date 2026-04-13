use maud::Render;

use crate::types::Text;

#[derive(Clone, Copy, Debug)]
pub(super) struct SectionCopy<'a> {
    pub title: &'a Text,
    pub subtitle: &'a Text,
}

impl Render for SectionCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="u-section-copy ui-portfolio-section-copy" {
                h2 { (&self.title) }
                p { (&self.subtitle) }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct LeadCopy<'a> {
    pub eyebrow: &'a Text,
    pub title: &'a Text,
    pub summary: &'a Text,
}

impl Render for LeadCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="ui-portfolio-eyebrow" { (&self.eyebrow) }
            h1 { (&self.title) }
            p class="ui-portfolio-summary" { (&self.summary) }
        }
    }
}
