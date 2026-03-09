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
pub(crate) struct List<'a> {
    pub children: &'a [Item],
}

impl Render for List<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for pane in self.children {
                (pane)
            }
        }
    }
}

fn json_literal(value: &Text) -> String {
    serde_json::to_string(&value.to_string()).unwrap_or_else(|_| "\"\"".to_string())
}

pub(super) fn show_expr(value: &Text) -> String {
    format!("$active_tab_id == {}", json_literal(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_show_expression_uses_json_literal() {
        assert_eq!(
            show_expr(&Text::from("quote'\"id")),
            "$active_tab_id == \"quote'\\\"id\""
        );
    }
}
