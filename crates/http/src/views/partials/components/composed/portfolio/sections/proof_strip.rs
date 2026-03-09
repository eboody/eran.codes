use maud::Render;

use super::{proof_kind_attr, SectionCopy};
use crate::views::partials::components::portfolio::content::ProofStripContent;

pub struct ProofStrip<'a> {
    pub content: &'a ProofStripContent,
}

impl Render for ProofStrip<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-surface-card" {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                div class="ui-portfolio-proof-strip" {
                    @for item in &self.content.items {
                        article class="ui-portfolio-proof-item" data-proof-kind=(proof_kind_attr(item.kind)) {
                            h3 { (&item.title) }
                            p { (&item.text) }
                        }
                    }
                }
            }
        }
    }
}
