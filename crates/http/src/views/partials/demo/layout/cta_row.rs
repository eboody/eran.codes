use bon::Builder;
use maud::Render;
use maud_extensions::css;

#[derive(Clone, Debug, Builder)]
pub struct CtaRow {
    pub items: Vec<maud::Markup>,
}

impl Render for CtaRow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-cta-row {
                @for item in &self.items {
                    (item.clone())
                }
            }
            ({
                css! {
                    me [data-cta-row] {
                      display: flex;
                      flex-wrap: wrap;
                      gap: 0.65rem;
                      margin-top: 1rem;
                    }
                }
            })
        }
    }
}
