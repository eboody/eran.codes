use bon::Builder;
use heroicons::{Icon, icon_name, icon_variant};
use maud::{PreEscaped, Render};

use crate::types::Text;
use crate::views::partials::components::{CodeBlock, CodeLanguage};

mod behavior;
mod styles;

use self::{behavior::Behavior, styles::Styles};

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
    fn cycle(index: usize) -> Self {
        match index % 6 {
            0 => Self::Indigo,
            1 => Self::Sky,
            2 => Self::Teal,
            3 => Self::Mint,
            4 => Self::Violet,
            _ => Self::Amber,
        }
    }

    fn as_attr(self) -> &'static str {
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
    fn as_attr(self) -> &'static str {
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
    fn render(self) -> maud::Markup {
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

impl Render for TabbedShowcase {
    fn render(&self) -> maud::Markup {
        if self.tabs.is_empty() {
            return maud::html! {};
        }

        let initial_tone = self
            .tabs
            .first()
            .and_then(|tab| tab.tone)
            .unwrap_or_else(|| TabbedShowcaseTone::cycle(0))
            .as_attr();

        maud::html! {
            section
                id=(&self.id)
                data-tabbed-showcase
                data-showcase-theme=(self.theme.as_attr())
            {
                div data-showcase-heading {
                    header data-showcase-title {
                        h2 data-showcase-title-text { (&self.title) }
                        p data-showcase-title-subtitle data-muted { (&self.subtitle) }
                    }
                }
                div data-showcase-root {
                    div data-showcase-shell {
                        (render_showcase_tabs(&self.id, &self.tabs, initial_tone))
                        (render_showcase_panels(&self.id, &self.tabs))
                    }
                }
                (Styles.render())
                (Behavior.render())
            }
        }
    }
}

fn render_showcase_tabs(
    showcase_id: &Text,
    tabs: &[TabbedShowcaseTab],
    initial_tone: &str,
) -> maud::Markup {
    maud::html! {
        nav
            data-showcase-tabs
            data-active-tone=(initial_tone)
            aria-label="Showcase tabs"
            role="tablist"
        {
            @for (index, tab) in tabs.iter().enumerate() {
                @let tab_id = format!("{}-tab-{}", showcase_id, index);
                @let panel_id = format!("{}-panel-{}", showcase_id, index);
                @let tone = tab
                    .tone
                    .unwrap_or_else(|| TabbedShowcaseTone::cycle(index));
                button
                    type="button"
                    data-tab-index=(index)
                    data-showcase-tone=(tone.as_attr())
                    role="tab"
                    id=(tab_id)
                    aria-controls=(panel_id)
                    aria-selected=(if index == 0 { "true" } else { "false" })
                    tabindex=(if index == 0 { "0" } else { "-1" })
                {
                    span data-showcase-tab-content {
                        @if let Some(icon) = tab.tab_icon {
                            span data-showcase-tab-icon aria-hidden="true" { (icon.render()) }
                        }
                        span data-showcase-tab-label { (&tab.tab_label) }
                    }
                }
            }
            span data-showcase-tab-indicator aria-hidden="true" {}
        }
    }
}

fn render_showcase_panels(showcase_id: &Text, tabs: &[TabbedShowcaseTab]) -> maud::Markup {
    maud::html! {
        div data-showcase-panels {
            @for (index, tab) in tabs.iter().enumerate() {
                (render_showcase_panel(showcase_id, tab, index))
            }
        }
    }
}

fn render_showcase_panel(
    showcase_id: &Text,
    tab: &TabbedShowcaseTab,
    index: usize,
) -> maud::Markup {
    let tab_id = format!("{}-tab-{}", showcase_id, index);
    let panel_id = format!("{}-panel-{}", showcase_id, index);
    let tone = tab
        .tone
        .unwrap_or_else(|| TabbedShowcaseTone::cycle(index));

    maud::html! {
        article
            data-showcase-panel
            data-showcase-tone=(tone.as_attr())
            data-panel-full[tab.mock_panel.is_none()]
            data-tab-index=(index)
            id=(panel_id)
            role="tabpanel"
            aria-labelledby=(tab_id)
            tabindex="0"
            hidden[index != 0]
        {
            @if let Some(mock_panel) = &tab.mock_panel {
                (render_showcase_mockup(mock_panel))
            }
            (render_showcase_copy(tab))
        }
    }
}

fn render_showcase_mockup(mock_panel: &TabbedShowcaseMockPanel) -> maud::Markup {
    maud::html! {
        div data-showcase-mockup {
            header {
                h3 { (&mock_panel.title) }
                p data-muted { (&mock_panel.subtitle) }
            }
            dl data-showcase-rows aria-label=(&mock_panel.title) {
                @for row in &mock_panel.rows {
                    div data-showcase-row {
                        dt data-showcase-row-label { (&row.label) }
                        dd data-showcase-row-value { (&row.value) }
                    }
                }
            }
        }
    }
}

fn render_showcase_copy(tab: &TabbedShowcaseTab) -> maud::Markup {
    maud::html! {
        div data-showcase-copy {
            div data-showcase-copy-content {
                h3 { (&tab.title) }
                p data-muted { (&tab.subtitle) }
                ul data-showcase-bullets {
                    @for bullet in &tab.bullets {
                        li { (bullet) }
                    }
                }
                @if let Some(action) = &tab.action {
                    a class="button" href=(&action.href) { (&action.label) }
                }
                div data-showcase-integrations {
                    p data-showcase-integrations-label { (&tab.chips_label) }
                    ul data-showcase-chip-list role="list" aria-label=(&tab.chips_label) {
                        @for chip in &tab.chips {
                            li data-showcase-chip { (chip) }
                        }
                    }
                }
                @if let Some(path) = &tab.code_path {
                    p data-code-path {
                        "Example: "
                        code { (path) }
                    }
                }
                @if let Some(code) = &tab.code {
                    (CodeBlock::builder()
                        .code(code.clone())
                        .language(CodeLanguage::Rust)
                        .build())
                }
            }
        }
    }
}

