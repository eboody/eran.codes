use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Tab;

#[derive(Clone, Debug)]
pub(crate) struct Set<'a> {
    pub aria_label: Text,
    pub tabs: List<'a>,
}

impl Render for Set<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            nav class="tab-set__tabs ui-tabs" role="tablist" aria-label=(&self.aria_label) {
                (self.tabs)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct List<'a> {
    pub children: &'a [Tab],
}

impl Render for List<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for tab in self.children {
                (tab)
            }
        }
    }
}
