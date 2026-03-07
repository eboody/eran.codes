use maud::Render;

use crate::types::Text;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
pub(crate) struct Action {
    pub label: Text,
    pub href: Option<Text>,
}

impl Action {
    pub(crate) fn from_content(action: &tab_set::content::Action) -> Self {
        Self {
            label: action.label.clone(),
            href: action.href.clone(),
        }
    }
}

impl Render for Action {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @if let Some(href) = &self.href {
                a class="button tab-set__cta ui-cta" href=(href) {
                    (&self.label)
                }
            } @else {
                button class="button tab-set__cta ui-cta" type="button" {
                    (&self.label)
                }
            }
        }
    }
}
