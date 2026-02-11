use bon::Builder;
use maud::Render;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct KeyValueList {
    pub items: Vec<(Text, Text)>,
}

impl Render for KeyValueList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class="key-value-list" {
                @for (label, value) in &self.items {
                    li { (label) ": " (value) }
                }
            }
        }
    }
}
