use maud::Render;

mod action;
mod body;
mod item;
mod preview;

pub(crate) use action::Action;
pub(crate) use body::Body;
pub(crate) use item::Item;
pub(crate) use preview::Preview;

// ci: render-composition-component
// ci: bon-builder-exempt
#[derive(Clone, Debug)]
pub(crate) struct List {
    pub children: Vec<Item>,
}

impl Render for List {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for pane in &self.children {
                (pane)
            }
        }
    }
}
