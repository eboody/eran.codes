use bon::Builder;
use maud::Render;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct EmptyState {
    pub message: Text,
}

impl Render for EmptyState {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="muted empty-state" { (&self.message) }
        }
    }
}
