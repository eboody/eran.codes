use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Tab;

#[derive(Clone, Debug)]
pub(crate) struct Set {
    pub aria_label: Text,
    pub tabs: List,
}

impl Render for Set {
    fn render(&self) -> maud::Markup {
        maud::html! {
            nav class="tab-set__tabs" role="tablist" aria-label=(&self.aria_label) {
                (self.tabs)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct List {
    pub children: Vec<Tab>,
}

impl Render for List {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for tab in &self.children {
                (tab)
            }
        }
    }
}
