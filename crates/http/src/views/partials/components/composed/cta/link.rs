use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::tone::CtaTone;

#[derive(Clone, Debug, Builder)]
pub struct CtaLink {
    pub label: Text,
    pub href: Text,
    #[builder(default)]
    pub tone: CtaTone,
    #[builder(default)]
    pub external: bool,
}

impl Render for CtaLink {
    fn render(&self) -> maud::Markup {
        let class_name = self.tone.class_name();

        maud::html! {
            @if self.external {
                a class=(class_name) href=(&self.href) target="_blank" rel="noopener noreferrer" {
                    (&self.label)
                }
            } @else {
                a class=(class_name) href=(&self.href) {
                    (&self.label)
                }
            }
        }
    }
}
