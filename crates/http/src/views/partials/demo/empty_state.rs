use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct EmptyState {
    pub message: String,
}

impl Render for EmptyState {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="muted empty-state" { (&self.message) }
        }
    }
}
