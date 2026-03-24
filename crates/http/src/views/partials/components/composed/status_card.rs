use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::KeyValueList;

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct StatusCard {
    pub title: Text,
    pub items: Vec<StatusCardItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
// ci: partials-render-exempt
pub struct StatusCardItem {
    pub label: Text,
    pub value: Text,
}

impl StatusCardItem {
    pub fn text(label: impl Into<Text>, value: impl Into<Text>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }

    pub fn optional(label: impl Into<Text>, value: Option<Text>) -> Self {
        Self {
            label: label.into(),
            value: value.unwrap_or_else(|| Text::from("none")),
        }
    }
}

impl Render for StatusCard {
    fn render(&self) -> maud::Markup {
        let items = self
            .items
            .iter()
            .map(|item| (item.label.clone(), item.value.clone()))
            .collect();

        maud::html! {
            div class="u-demo-result-card u-inset-card" data-demo-result data-status-card {
                p { strong { (&self.title) } }
                (KeyValueList::builder().items(items).build())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_item_defaults_to_none() {
        assert_eq!(
            StatusCardItem::optional("user_id", None),
            StatusCardItem {
                label: Text::from("user_id"),
                value: Text::from("none"),
            }
        );
    }
}
