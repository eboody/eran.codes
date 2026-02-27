use bon::Builder;
use maud::Render;
use maud_extensions::css;

use crate::views::partials::KeyValueList;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct StatusCard {
    pub title: Text,
    pub items: Vec<(Text, Text)>,
}

impl Render for StatusCard {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-demo-result data-status-card {
                p { strong { (&self.title) } }
                (KeyValueList::builder().items(self.items.clone()).build())
            }
            ({
                css! {
                    me [data-demo-result] {
                      margin-top: 0.8rem;
                      padding: 0.8rem 1rem;
                      border-radius: var(--ui-radius-sm);
                      border: 1px solid var(--pico-muted-border-color);
                      background: var(--pico-card-background-color);
                    }
                    me [data-status-card] > p {
                      margin: 0;
                    }
                }
            })
        }
    }
}
