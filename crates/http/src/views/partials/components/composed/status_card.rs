#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{KeyValueItem, KeyValueList};

use super::ResultCard;

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct StatusCard {
    pub title: Text,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
// ci: partials-render-exempt
pub struct Item {
    pub label: Text,
    pub value: Text,
}

impl Item {
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
            .map(|item| KeyValueItem::text(item.label.clone(), item.value.clone()))
            .collect();

        ResultCard::builder()
            .extra_class("u-status-card")
            .status_card(true)
            .content(maud::html! {
                p class="u-status-card-title" data-status-card-title { strong { (&self.title) } }
                (KeyValueList::builder().items(items).build())
            })
            .build()
            .render()
    }
}
