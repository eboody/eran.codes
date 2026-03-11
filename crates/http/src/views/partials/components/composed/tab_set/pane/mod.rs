use maud::Render;

use crate::types::Text;

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

fn json_literal(value: &Text) -> String {
    serde_json::to_string(&value.to_string()).unwrap_or_else(|_| "\"\"".to_string())
}

pub(super) fn show_expr(signal_name: &Text, value: &Text) -> String {
    format!("${} == {}", signal_name, json_literal(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_show_expression_uses_signal_name_and_json_literal() {
        assert_eq!(
            show_expr(&Text::from("selected_tab"), &Text::from("quote'\"id")),
            "$selected_tab == \"quote'\\\"id\""
        );
    }
}
