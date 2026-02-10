use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct SupportCard {
    pub title: String,
    pub description: Option<String>,
    pub body: Vec<maud::Markup>,
}

impl Render for SupportCard {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article class="support-card" {
                h3 { (&self.title) }
                @if let Some(description) = &self.description {
                    p class="muted" { (description) }
                }
                @for block in &self.body {
                    (block.clone())
                }
            }
        }
    }
}
