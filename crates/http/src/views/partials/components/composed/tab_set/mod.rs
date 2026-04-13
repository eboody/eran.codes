mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{
    LocalTabRoot, LocalTabRootSurface, Tab, TabInteraction,
};
use crate::views::proper_theme::{Palette, THEME};

pub(crate) mod content;
pub(crate) mod pane;
pub(crate) mod tab;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Variant {
    #[default]
    Standard,
    FlatGallery,
}

impl Variant {
    fn class_name(self) -> Option<&'static str> {
        match self {
            Self::Standard => None,
            Self::FlatGallery => Some("tab-set-showcase--flat-gallery"),
        }
    }
}

// ci: descriptive-module-import crate::views::partials::components::tab_set
#[derive(Clone, Debug, Builder)]
pub(crate) struct Component<'a> {
    pub id: &'a str,
    pub class: &'a str,
    #[builder(default)]
    pub variant: Variant,
    pub active_tab_id: Text,
    pub tabs: tab::Set,
    pub panes: pane::List,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct ContentProps<'a> {
    pub id: &'a str,
    pub class: &'a str,
    pub aria_label: Text,
    pub content: &'a content::TabSet,
    #[builder(default)]
    pub variant: Variant,
    pub active_tab_id: Option<Text>,
    pub palette: Option<&'static Palette>,
}

impl<'a> Component<'a> {
    pub(crate) fn from_content(props: ContentProps<'a>) -> Self {
        let palette = props.palette.unwrap_or(&THEME.gray);
        let active_tab_id = props.active_tab_id.unwrap_or_else(|| {
            props
                .content
                .tabs
                .first()
                .map(|tab| tab.id.clone())
                .unwrap_or_else(|| Text::from("tab_0"))
        });
        let tabs = tabs_from_content(
            props.id,
            palette,
            &active_tab_id,
            props.content,
        );
        let panes = panes_from_content(&active_tab_id, props.content, &tabs);

        Self {
            id: props.id,
            class: props.class,
            variant: props.variant,
            active_tab_id,
            tabs: tab::Set {
                aria_label: props.aria_label,
                style: tab::Style::Standard,
                tabs: tab::List { children: tabs },
            },
            panes: pane::List { children: panes },
        }
    }

    fn root_class_names(&self) -> String {
        let mut classes = vec!["tab-set-showcase"];
        if !self.class.is_empty() {
            classes.push(self.class);
        }
        if let Some(class_name) = self.variant.class_name() {
            classes.push(class_name);
        }
        classes.join(" ")
    }
}

impl Render for Component<'_> {
    fn render(&self) -> maud::Markup {
        let class_names = self.root_class_names();

        LocalTabRoot {
            surface: LocalTabRootSurface::standard(self.id, class_names.as_str()),
            active_tab_id: self.active_tab_id.clone(),
            content: maud::html! {
                (styles::render())
                (self.tabs)
                (self.panes)
            },
        }
        .render()
    }
}

fn tabs_from_content(
    root_id: &str,
    palette: &'static Palette,
    active_tab_id: &Text,
    content: &content::TabSet,
) -> Vec<Tab> {
    content
        .tabs
        .iter()
        .enumerate()
        .map(|(index, tab)| Tab {
            id: Text::from(format!("{root_id}-tab-{index}")),
            controls: Text::from(format!("{root_id}-pane-{index}")),
            palette,
            is_selected: tab.id == *active_tab_id,
            icon: tab.icon.clone(),
            primary_text: tab.label.primary.clone(),
            secondary_text: tab.label.secondary.clone(),
            interaction: TabInteraction::LocalTabs {
                value: tab.id.clone(),
            },
        })
        .collect()
}

fn panes_from_content(
    active_tab_id: &Text,
    content: &content::TabSet,
    tabs: &[Tab],
) -> Vec<pane::Item> {
    content
        .tabs
        .iter()
        .zip(tabs.iter())
        .map(|(tab_content, tab)| {
            pane::Item::from_content(tab, tab_content, tab_content.id == *active_tab_id)
        })
        .collect()
}
