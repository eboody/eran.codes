use maud::Render;

use crate::types::Text;
use crate::views::partials::components::button;
use crate::views::partials::components::tab_set;

#[derive(Clone, Debug)]
pub(crate) struct Action {
    pub label: Text,
    pub href: Option<Text>,
}

impl From<&tab_set::content::Action> for Action {
    fn from(action: &tab_set::content::Action) -> Self {
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
                (button::Button::builder()
                    .label(self.label.clone())
                    .variant(button::Variant::Primary)
                    .role(button::Role::link(href.clone()))
                    .build())
            } @else {
                (button::Button::builder()
                    .label(self.label.clone())
                    .variant(button::Variant::Primary)
                    .build())
            }
        }
    }
}
