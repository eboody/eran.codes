use bon::Builder;
use maud::{Markup, Render};

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub title: String,
    pub content: Markup,
}

impl Render for DemoSection {
    fn render(&self) -> Markup {
        maud::html! {
            section {
                h2 { (&self.title) }
                article class="flow-card" {
                    (self.content.clone())
                }
            }
        }
    }
}
