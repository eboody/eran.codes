use bon::Builder;
use maud::Render;

use crate::views::partials::KeyValueList;

#[derive(Clone, Debug, Builder)]
pub struct StatusCard {
    pub title: String,
    pub items: Vec<(String, String)>,
}

impl Render for StatusCard {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="demo-result status-card" {
                p { strong { (&self.title) } }
                (KeyValueList::builder().items(self.items.clone()).build().render())
            }
        }
    }
}
