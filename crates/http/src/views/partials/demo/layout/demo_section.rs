use bon::Builder;
use maud::{Markup, Render};
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct DemoSection {
    pub title: Text,
    pub content: Markup,
}

impl Render for DemoSection {
    fn render(&self) -> Markup {
        maud::html! {
            section {
                h2 { (self.title.to_string()) }
                article class="flow-card" {
                    (self.content.clone())
                }
            }
        }
    }
}
