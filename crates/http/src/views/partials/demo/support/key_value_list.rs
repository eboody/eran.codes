use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct KeyValueList {
    pub items: Vec<(String, String)>,
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
