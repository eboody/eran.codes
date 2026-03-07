use bon::Builder;
use maud::Render;

use super::item::CtaItem;

#[derive(Clone, Debug, Builder)]
pub struct CtaRow {
    pub items: Vec<CtaItem>,
}

impl Render for CtaRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-cta-row" data-cta-row {
                @for item in &self.items {
                    (item)
                }
            }
        }
    }
}
