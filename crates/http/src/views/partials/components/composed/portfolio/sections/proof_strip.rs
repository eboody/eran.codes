use maud::Render;

use super::{SectionCopy, Surface, proof_kind_attr};
use crate::views::partials::components::portfolio::content::ProofStripContent;

pub struct ProofStrip<'a> {
    pub content: &'a ProofStripContent,
}

impl Render for ProofStrip<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                div class="ui-portfolio-proof-strip" {
                    @for item in &self.content.items {
                        article class="ui-portfolio-proof-item u-inset-card" data-proof-kind=(proof_kind_attr(item.kind)) {
                            h3 { (&item.title) }
                            p { (&item.text) }
                        }
                    }
                }
            }))
        }
    }
}
