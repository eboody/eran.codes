use bon::Builder;
use maud::Render;

use super::Button;

#[derive(Clone, Debug, Builder)]
pub struct Row {
    pub items: Vec<Button>,
}

impl Render for Row {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-button-row" data-button-row {
                @for item in &self.items {
                    (item)
                }
            }
        }
    }
}
