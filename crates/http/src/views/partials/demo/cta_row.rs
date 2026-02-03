use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct CtaRow {
    pub items: Vec<maud::Markup>,
}

impl Render for CtaRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="cta-row" {
                @for item in &self.items {
                    (item.clone())
                }
            }
        }
    }
}
