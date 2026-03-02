use bon::Builder;
use heroicons::{Icon, icon_name, icon_variant};
use maud::PreEscaped;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseRow {
    pub label: Text,
    pub value: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseAction {
    pub label: Text,
    pub href: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseMockPanel {
    pub title: Text,
    pub subtitle: Text,
    pub rows: Vec<TabbedShowcaseRow>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TabbedShowcaseTone {
    Indigo,
    Sky,
    Teal,
    Mint,
    Violet,
    Amber,
}

impl TabbedShowcaseTone {
    pub(super) fn cycle(index: usize) -> Self {
        match index % 6 {
            0 => Self::Indigo,
            1 => Self::Sky,
            2 => Self::Teal,
            3 => Self::Mint,
            4 => Self::Violet,
            _ => Self::Amber,
        }
    }

    pub(super) fn as_attr(self) -> &'static str {
        match self {
            Self::Indigo => "indigo",
            Self::Sky => "sky",
            Self::Teal => "teal",
            Self::Mint => "mint",
            Self::Violet => "violet",
            Self::Amber => "amber",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum TabbedShowcaseTheme {
    #[default]
    Netbird,
    NetbirdDetail,
}

impl TabbedShowcaseTheme {
    pub(super) fn as_attr(self) -> &'static str {
        match self {
            Self::Netbird => "netbird",
            Self::NetbirdDetail => "netbird-detail",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TabbedShowcaseIcon {
    ShieldCheck,
    ArrowsRightLeft,
    Signal,
    ChatBubbleLeftRight,
}

impl TabbedShowcaseIcon {
    pub(super) fn render(self) -> maud::Markup {
        let svg = match self {
            Self::ShieldCheck => Icon {
                name: icon_name::ShieldCheck,
                variant: icon_variant::Outline,
                id: "",
                class: "",
            }
            .to_string(),
            Self::ArrowsRightLeft => Icon {
                name: icon_name::ArrowsRightLeft,
                variant: icon_variant::Outline,
                id: "",
                class: "",
            }
            .to_string(),
            Self::Signal => Icon {
                name: icon_name::Signal,
                variant: icon_variant::Outline,
                id: "",
                class: "",
            }
            .to_string(),
            Self::ChatBubbleLeftRight => Icon {
                name: icon_name::ChatBubbleLeftRight,
                variant: icon_variant::Outline,
                id: "",
                class: "",
            }
            .to_string(),
        };

        maud::html! { (PreEscaped(svg)) }
    }
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct TabbedShowcaseTab {
    pub tone: Option<TabbedShowcaseTone>,
    pub tab_icon: Option<TabbedShowcaseIcon>,
    pub tab_label: Text,
    pub title: Text,
    pub subtitle: Text,
    pub bullets: Vec<Text>,
    pub mock_panel: Option<TabbedShowcaseMockPanel>,
    pub chips_label: Text,
    pub chips: Vec<Text>,
    pub action: Option<TabbedShowcaseAction>,
    pub code_path: Option<Text>,
    pub code: Option<Text>,
}

#[derive(Clone, Debug, Builder)]
pub struct TabbedShowcase {
    pub id: Text,
    pub title: Text,
    pub subtitle: Text,
    pub tabs: Vec<TabbedShowcaseTab>,
    #[builder(default)]
    pub theme: TabbedShowcaseTheme,
}
