use bon::Builder;
use maud::Render;

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
            div class="ui-demo-result ui-status-card" data-demo-result data-status-card {
                p { strong { (&self.title) } }
                (KeyValueList::builder().items(self.items.clone()).build())
            }
        }
    }
}
