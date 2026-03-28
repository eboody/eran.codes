use maud::Render;

use crate::types::Text;
use crate::views::partials::components::Tab;

#[derive(Clone, Debug)]
pub(crate) struct Set {
    pub aria_label: Text,
    pub style: Style,
    pub tabs: List,
}

impl Render for Set {
    fn render(&self) -> maud::Markup {
        maud::html! {
            nav
                class="tab-set__tabs"
                role="tablist"
                aria-label=(&self.aria_label)
                data-tab-set-tabs-style=(self.style.as_str()) {
                (self.tabs)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Style {
    #[default]
    Standard,
    PillCluster,
}

impl Style {
    fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::PillCluster => "pill-cluster",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct List {
    pub children: Vec<Tab>,
}

impl Render for List {
    fn render(&self) -> maud::Markup {
        maud::html! {
            @for tab in &self.children {
                (tab)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_set_emits_public_tabs_style_contract() {
        let markup = Set {
            aria_label: Text::from("Example tabs"),
            style: Style::PillCluster,
            tabs: List { children: vec![] },
        }
        .render()
        .into_string();

        assert!(markup.contains("data-tab-set-tabs-style=\"pill-cluster\""));
    }
}
