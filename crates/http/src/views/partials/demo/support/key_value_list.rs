use bon::Builder;
use maud::Render;
use maud_extensions::css;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct KeyValueList {
    pub items: Vec<(Text, Text)>,
}

impl Render for KeyValueList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul data-key-value-list {
                @for (label, value) in &self.items {
                    li { (label) ": " (value) }
                }
            }
            ({
                css! {
                    me [data-key-value-list] {
                      margin: 0.5rem 0 0;
                      padding-left: 1rem;
                      font-size: 0.82rem;
                      color: var(--pico-muted-color);
                    }
                    me [data-key-value-list] li {
                      margin: 0.2rem 0;
                      word-break: break-word;
                    }
                }
            })
        }
    }
}
