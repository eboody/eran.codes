use bon::Builder;
use heroicons::{Icon, icon_name, icon_variant};
use maud::{PreEscaped, Render};
use maud_extensions::{inline_css, inline_js};

use crate::types::Text;
use crate::views::partials::components::{CodeBlock, CodeLanguage};

use super::SectionHeader;

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
}

impl TabbedShowcaseTheme {
    fn as_attr(self) -> &'static str {
        match self {
            Self::Netbird => "netbird",
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
                    (SectionHeader::builder()
                        .title(self.title.clone())
                        .subtitle(self.subtitle.clone())
                        .build())
                }
                div data-showcase-root {
                    div data-showcase-shell {
                        (render_showcase_tabs(&self.id, &self.tabs, initial_tone))
                        (render_showcase_panels(&self.id, &self.tabs))
                    }
                }
                (css())
                (js())
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
            ul data-showcase-rows {
                @for row in &mock_panel.rows {
                    li {
                        span data-showcase-row-label { (&row.label) }
                        span data-showcase-row-value { (&row.value) }
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


inline_css! {
    me {
      --showcase-space-1: var(--size-2);
      --showcase-space-2: var(--size-3);
      --showcase-space-3: var(--size-4);
      --showcase-space-4: var(--size-5);
      --showcase-space-5: var(--size-6);
      --showcase-space-6: var(--size-7);
      --showcase-radius-shell: var(--radius-5);
      --showcase-radius-surface: var(--radius-4);
      --showcase-border-size: var(--border-size-1);
      --showcase-border-size-strong: var(--border-size-2);
      --showcase-section-margin-top: var(--size-7);
      --showcase-heading-gap: var(--size-4);
      --showcase-tab-gap: var(--size-4);
      --showcase-tab-list-padding-bottom: var(--size-2);
      --showcase-tab-font-size: var(--font-size-0);
      --showcase-tab-font-weight: var(--font-weight-6);
      --showcase-tab-letter-spacing: var(--font-letterspacing-2);
      --showcase-tab-padding-y: var(--size-2);
      --showcase-tab-padding-x: var(--size-1);
      --showcase-tab-hover-offset: calc(var(--border-size-1) * -1);
      --showcase-focus-outline-size: var(--border-size-2);
      --showcase-focus-outline-offset: 0;
      --showcase-focus-border-color: color-mix(
        in srgb,
        var(--tone-accent) 82%,
        white 18%
      );
      --showcase-focus-background: color-mix(
        in srgb,
        var(--tone-tab-soft) 82%,
        transparent 18%
      );
      --showcase-focus-inset-ring: inset 0 0 0 var(--showcase-focus-outline-size)
        var(--tone-accent);
      --showcase-shell-blur: var(--size-px-2);
      --showcase-tab-transition-duration: 240ms;
      --showcase-tab-indicator-transition-duration: 320ms;
      --showcase-tab-icon-size: var(--size-4);
      --showcase-tab-icon-min-width: calc(
        var(--showcase-tab-icon-size) + var(--size-1)
      );
      --showcase-shell-bg: var(--gray-12);
      --showcase-shell-bg-alt: color-mix(
        in srgb,
        var(--gray-12) 82%,
        var(--blue-12) 18%
      );
      --showcase-shell-border: color-mix(
        in srgb,
        var(--gray-7) 62%,
        transparent 38%
      );
      --showcase-shell-highlight-a: color-mix(
        in srgb,
        var(--orange-5) 16%,
        transparent 84%
      );
      --showcase-shell-highlight-b: color-mix(
        in srgb,
        var(--blue-5) 14%,
        transparent 86%
      );
      --showcase-shell-shadow: var(--shadow-6);
      --showcase-panel-bg: color-mix(
        in srgb,
        var(--gray-12) 92%,
        var(--gray-11) 8%
      );
      --showcase-panel-border: color-mix(
        in srgb,
        var(--gray-8) 56%,
        transparent 44%
      );
      --showcase-row-bg: color-mix(in srgb, var(--gray-12) 86%, var(--gray-10) 14%);
      --showcase-row-border: color-mix(in srgb, var(--gray-8) 54%, transparent 46%);
      --showcase-tab-bg: transparent;
      --showcase-tab-text: var(--gray-5);
      --showcase-tab-text-active: var(--gray-0);
      --showcase-tab-active-bg: color-mix(
        in srgb,
        var(--tone-tab-soft, hsl(220 14% 18% / 0.82)) 86%,
        transparent 14%
      );
      --showcase-tab-active-border: color-mix(
        in srgb,
        var(--tone-accent, hsl(220 60% 58%)) 48%,
        transparent 52%
      );
      --showcase-tab-active-shadow: var(--shadow-2);
      --showcase-tab-border: transparent;
      --showcase-tab-divider: color-mix(
        in srgb,
        var(--gray-8) 72%,
        transparent 28%
      );
      --showcase-copy-text-default: var(--gray-0);
      --showcase-copy-muted-default: color-mix(
        in srgb,
        var(--gray-4) 92%,
        transparent 8%
      );
      --showcase-copy-title-size: var(--font-size-3);
      --showcase-copy-line-height: var(--font-lineheight-3);
      --showcase-copy-padding: var(--size-4);
      --showcase-copy-max-width: var(--size-content-4);
      --showcase-copy-inner-gap: var(--size-3);
      --showcase-copy-bullets-padding-left: var(--size-5);
      --showcase-copy-bullets-gap: var(--size-2);
      --showcase-integrations-gap: var(--size-1);
      --showcase-chip-padding-block: var(--size-1);
      --showcase-chip-padding-inline: var(--size-2);
      --showcase-chip-font-size: var(--font-size-00);
      --showcase-row-gap: var(--size-2);
      --showcase-row-padding-block: var(--size-2);
      --showcase-row-padding-inline: var(--size-3);
      --showcase-row-label-size: var(--font-size-00);
      --showcase-row-value-size: var(--font-size-0);
      --showcase-mockup-title-margin-bottom: var(--size-1);
      --showcase-indicator-height: var(--border-size-2);
      --showcase-indicator-width: 0;
      --showcase-indicator-x: 0;
      --showcase-mobile-margin-top: var(--size-5);
      --showcase-mobile-shell-padding: var(--size-3);
      --showcase-mobile-tab-font-size: var(--font-size-00);
      --showcase-row-value-font-weight: var(--font-weight-6);
      --showcase-chip-font-weight: var(--font-weight-6);
      --showcase-button-border-color: color-mix(
        in srgb,
        var(--gray-0) 36%,
        transparent 64%
      );
      --showcase-selection-text: var(--gray-0);
      --showcase-button-shadow: var(--shadow-3);
      margin-top: var(--showcase-section-margin-top);
    }
    me[data-showcase-theme="netbird"] {
      border: var(--showcase-border-size) solid var(--showcase-shell-border);
      border-radius: var(--showcase-radius-shell);
      padding: var(--showcase-space-4);
      background:
        radial-gradient(
          circle at 0% 0%,
          var(--showcase-shell-highlight-a),
          transparent 48%
        ),
        radial-gradient(
          circle at 100% 0%,
          var(--showcase-shell-highlight-b),
          transparent 55%
        ),
        linear-gradient(
          180deg,
          var(--showcase-shell-bg),
          var(--showcase-shell-bg-alt) 74%
        );
      box-shadow: var(--showcase-shell-shadow);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone] {
      --tone-base: var(--indigo-5);
      --tone-base-strong: var(--indigo-6);
      --tone-base-end: var(--indigo-8);
      --tone-accent: var(--indigo-5);
      --tone-tab-soft: hsl(229 69% 25% / 0.28);
      --tone-surface-start: hsl(229 40% 14%);
      --tone-surface-end: hsl(229 48% 10%);
      --tone-border: hsl(229 58% 32% / 0.56);
      --tone-copy-text: hsl(0 0% 100%);
      --tone-copy-muted: hsl(214 18% 76% / 0.88);
      --tone-chip-bg: hsl(220 22% 14% / 0.92);
      --tone-chip-border: hsl(220 10% 32% / 0.74);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="indigo"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="indigo"] {
      --tone-base: var(--indigo-5);
      --tone-base-strong: var(--indigo-6);
      --tone-base-end: var(--indigo-8);
      --tone-accent: var(--indigo-5);
      --tone-tab-soft: hsl(229 69% 25% / 0.28);
      --tone-surface-start: hsl(229 40% 14%);
      --tone-surface-end: hsl(229 48% 10%);
      --tone-border: hsl(229 58% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="sky"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="sky"] {
      --tone-base: var(--blue-5);
      --tone-base-strong: var(--blue-6);
      --tone-base-end: var(--blue-8);
      --tone-accent: var(--blue-5);
      --tone-tab-soft: hsl(205 75% 24% / 0.28);
      --tone-surface-start: hsl(208 46% 14%);
      --tone-surface-end: hsl(212 52% 10%);
      --tone-border: hsl(208 64% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="teal"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="teal"] {
      --tone-base: var(--teal-5);
      --tone-base-strong: var(--teal-6);
      --tone-base-end: var(--cyan-8);
      --tone-accent: var(--teal-5);
      --tone-tab-soft: hsl(182 58% 24% / 0.28);
      --tone-surface-start: hsl(186 44% 13%);
      --tone-surface-end: hsl(190 52% 9%);
      --tone-border: hsl(184 55% 31% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="mint"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="mint"] {
      --tone-base: var(--green-5);
      --tone-base-strong: var(--green-6);
      --tone-base-end: var(--lime-8);
      --tone-accent: var(--green-5);
      --tone-tab-soft: hsl(152 46% 23% / 0.28);
      --tone-surface-start: hsl(154 40% 13%);
      --tone-surface-end: hsl(158 48% 9%);
      --tone-border: hsl(153 48% 30% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="violet"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="violet"] {
      --tone-base: var(--violet-5);
      --tone-base-strong: var(--violet-6);
      --tone-base-end: var(--purple-8);
      --tone-accent: var(--violet-5);
      --tone-tab-soft: hsl(274 53% 24% / 0.28);
      --tone-surface-start: hsl(275 42% 14%);
      --tone-surface-end: hsl(278 50% 10%);
      --tone-border: hsl(274 50% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="amber"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="amber"] {
      --tone-base: hsl(25 91% 58%);
      --tone-base-strong: hsl(25 91% 58%);
      --tone-base-end: hsl(14 89% 57%);
      --tone-accent: hsl(25 91% 58%);
      --tone-tab-soft: hsl(25 91% 58% / 0.24);
      --tone-surface-start: hsl(22 56% 14%);
      --tone-surface-end: hsl(18 66% 10%);
      --tone-border: hsl(24 72% 35% / 0.6);
    }
    me > [data-showcase-heading] {
      margin-bottom: var(--showcase-heading-gap);
    }
    me > [data-showcase-root] > [data-showcase-shell] {
      border: var(--showcase-border-size) solid var(--showcase-shell-border);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-space-2);
      background: var(--showcase-shell-bg);
      backdrop-filter: blur(var(--showcase-shell-blur));
    }
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs] {
      position: relative;
      display: flex;
      align-items: stretch;
      gap: var(--showcase-tab-gap);
      overflow-x: auto;
      padding-bottom: calc(
        var(--showcase-tab-list-padding-bottom) + var(--showcase-indicator-height)
      );
      border-bottom: var(--showcase-border-size) solid var(--showcase-tab-divider);
      scrollbar-width: thin;
      --showcase-active-tone: hsl(25 91% 58%);
      --showcase-active-tone-shadow: 0 2px 8px hsl(25 91% 58% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"] {
      cursor: pointer;
      border-radius: var(--radius-2);
      border: var(--showcase-border-size) solid var(--showcase-tab-border);
      background: var(--showcase-tab-bg);
      color: var(--showcase-tab-text);
      padding: var(--showcase-tab-padding-y) var(--showcase-tab-padding-x);
      min-width: max-content;
      font-size: var(--showcase-tab-font-size);
      font-weight: var(--showcase-tab-font-weight);
      letter-spacing: var(--showcase-tab-letter-spacing);
      display: inline-flex;
      align-items: center;
      gap: var(--showcase-space-2);
      transition:
        border-color var(--showcase-tab-transition-duration) var(--ease-3),
        background-color var(--showcase-tab-transition-duration) var(--ease-3),
        color var(--showcase-tab-transition-duration) var(--ease-3),
        transform var(--showcase-tab-transition-duration) var(--ease-3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:hover {
      border-color: transparent;
      background: var(--tone-tab-soft);
      color: var(--showcase-tab-text-active);
      transform: translateY(var(--showcase-tab-hover-offset));
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"][aria-selected="true"] {
      border-color: var(--showcase-tab-active-border);
      background: var(--showcase-tab-active-bg);
      color: var(--showcase-tab-text-active);
      box-shadow: var(--showcase-tab-active-shadow);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:focus-visible {
      outline: none;
      outline-offset: var(--showcase-focus-outline-offset);
      border-color: var(--showcase-focus-border-color);
      background: var(--showcase-focus-background);
      box-shadow: var(--showcase-focus-inset-ring);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:focus:not(:focus-visible) {
      outline: none;
      box-shadow: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content] {
      display: inline-flex;
      align-items: center;
      gap: var(--showcase-space-2);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-label] {
      white-space: nowrap;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-icon] {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-width: var(--showcase-tab-icon-min-width);
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-icon]
      > svg {
      width: var(--showcase-tab-icon-size);
      height: var(--showcase-tab-icon-size);
      display: block;
      stroke: currentColor;
      fill: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tab-indicator] {
      position: absolute;
      left: 0;
      bottom: 0;
      width: var(--showcase-indicator-width);
      height: var(--showcase-indicator-height);
      transform: translateX(var(--showcase-indicator-x));
      border-radius: var(--radius-round);
      background: var(--showcase-active-tone);
      box-shadow: var(--showcase-active-tone-shadow);
      transition:
        transform var(--showcase-tab-indicator-transition-duration)
          var(--ease-spring-2),
        width var(--showcase-tab-indicator-transition-duration) var(--ease-3),
        background-color var(--showcase-tab-transition-duration) var(--ease-out-3),
        box-shadow var(--showcase-tab-transition-duration) var(--ease-out-3);
      pointer-events: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="indigo"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--indigo-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(229 68% 58% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="sky"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--blue-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(206 80% 58% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="teal"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--teal-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(183 61% 52% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="mint"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--green-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(151 48% 48% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="violet"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--violet-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(274 64% 61% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="amber"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--orange-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(25 91% 58% / 0.3);
    }
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-panels] {
      margin-top: var(--showcase-space-3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel] {
      display: grid;
      gap: var(--showcase-space-3);
      min-width: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel][hidden] {
      display: none;
    }
    @media (min-width: 64rem) {
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-panels]
        > [data-showcase-panel] {
        grid-template-columns: 1.1fr 0.9fr;
        align-items: stretch;
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-panels]
        > [data-showcase-panel][data-panel-full] {
        grid-template-columns: 1fr;
      }
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > * {
      min-width: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup] {
      border: var(--showcase-border-size) solid var(--showcase-panel-border);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-space-3);
      background: var(--showcase-panel-bg);
      box-shadow: var(--shadow-1);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > header
      > h3 {
      margin-bottom: var(--showcase-mockup-title-margin-bottom);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows] {
      list-style: none;
      margin: var(--showcase-space-4) 0 0;
      padding: 0;
      display: grid;
      gap: var(--showcase-row-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > li {
      border: var(--showcase-border-size) solid var(--showcase-row-border);
      border-radius: var(--radius-2);
      background: var(--showcase-row-bg);
      padding: var(--showcase-row-padding-block) var(--showcase-row-padding-inline);
      display: flex;
      justify-content: space-between;
      gap: var(--showcase-space-3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > li
      > [data-showcase-row-label] {
      color: var(--showcase-copy-muted-default);
      font-size: var(--showcase-row-label-size);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > li
      > [data-showcase-row-value] {
      font-size: var(--showcase-row-value-size);
      font-weight: var(--showcase-row-value-font-weight);
      text-align: right;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy] {
      border: var(--showcase-border-size) solid var(--tone-border);
      border-top: var(--showcase-border-size-strong) solid var(--tone-accent);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-copy-padding);
      background: linear-gradient(
        180deg,
        var(--tone-surface-start),
        var(--tone-surface-end)
      );
      color: var(--tone-copy-text, var(--showcase-copy-text-default));
      box-shadow: var(--shadow-2);
      overflow: hidden;
      display: flex;
      justify-content: flex-start;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content] {
      width: 100%;
      max-width: var(--showcase-copy-max-width);
      display: grid;
      gap: var(--showcase-copy-inner-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > h3 {
      color: inherit;
      margin: 0;
      font-size: var(--showcase-copy-title-size);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-muted] {
      margin: 0;
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
      line-height: var(--showcase-copy-line-height);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets] {
      margin: 0;
      padding-left: var(--showcase-copy-bullets-padding-left);
      display: grid;
      gap: var(--showcase-copy-bullets-gap);
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets]
      > li {
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets]
      > li::marker {
      color: var(--tone-accent);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > .button {
      margin: 0;
      border-radius: var(--radius-round);
      background: var(--tone-accent);
      color: var(--tone-copy-text, var(--showcase-copy-text-default));
      border: var(--showcase-border-size) solid var(--showcase-button-border-color);
      box-shadow: var(--showcase-button-shadow);
      justify-self: start;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations] {
      margin: 0;
      display: grid;
      gap: var(--showcase-space-1);
      font-size: var(--showcase-chip-font-size);
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-integrations-label] {
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-chip-list] {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      gap: var(--showcase-integrations-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-chip-list]
      > [data-showcase-chip] {
      border: var(--showcase-border-size) solid var(--tone-chip-border);
      border-radius: var(--radius-round);
      padding: var(--showcase-chip-padding-block)
        var(--showcase-chip-padding-inline);
      font-weight: var(--showcase-chip-font-weight);
      background: var(--tone-chip-bg, hsl(0 0% 100% / 0.14));
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-code-path] {
      margin: 0;
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
      font-size: var(--font-size-0);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-code-path]
      > code {
      word-break: break-all;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]::selection {
      background: var(--tone-accent);
      color: var(--showcase-selection-text);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]::-moz-selection {
      background: var(--tone-accent);
      color: var(--showcase-selection-text);
    }
    @media (max-width: 48rem) {
      me[data-showcase-theme="netbird"] {
        margin-top: var(--showcase-mobile-margin-top);
        padding: var(--showcase-space-3);
      }
      me > [data-showcase-root] > [data-showcase-shell] {
        padding: var(--showcase-mobile-shell-padding);
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > button[role="tab"] {
        min-width: max-content;
        font-size: var(--showcase-mobile-tab-font-size);
      }
    }
    @media (prefers-reduced-motion: reduce) {
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > button[role="tab"] {
        transition: none;
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > [data-showcase-tab-indicator] {
        transition: none;
      }
    }
}

// Tab selection here is local presentation state; Surreal wires DOM behavior directly,
// and Datastar signals are intentionally not used because no backend/shared state is involved.
inline_js! {
    (() => {
      const root = me();
      if (!root) return;

      if (root.attribute("data-showcase-bound") === "true") return;
      root.attribute("data-showcase-bound", "true");

      const tabList = me("[data-showcase-tabs][role=\"tablist\"]", root, false);
      const indicator = me("[data-showcase-tab-indicator]", root, false);
      const tabs = any("[role=\"tab\"]", root, false);
      const panels = any("[data-showcase-panel][role=\"tabpanel\"]", root, false);
      if (!tabList || !tabs.length || !panels.length) return;

      const prefersReducedMotion =
        typeof window.matchMedia === "function" &&
        window.matchMedia("(prefers-reduced-motion: reduce)").matches;

      const lastIndex = tabs.length - 1;
      let activeIndex = tabs.findIndex(
        (tab) => tab.attribute("aria-selected") === "true",
      );
      if (activeIndex < 0) activeIndex = 0;

      const moveIndicator = () => {
        if (!indicator) return;

        const activeTab = tabs[activeIndex];
        if (!activeTab) return;

        const listRect = tabList.getBoundingClientRect();
        const tabRect = activeTab.getBoundingClientRect();
        const offset = tabRect.left - listRect.left + tabList.scrollLeft;

        indicator.styles({
          transform: "translateX(" + Math.max(0, offset) + "px)",
          width: String(tabRect.width) + "px",
        });

        const tone = activeTab.attribute("data-showcase-tone");
        if (tone) tabList.attribute("data-active-tone", tone);
      };

      const syncActiveTabVisibility = (fromInteraction) => {
        if (tabList.scrollWidth <= tabList.clientWidth) return;
        const activeTab = tabs[activeIndex];
        if (!activeTab) return;
        const behavior =
          fromInteraction && !prefersReducedMotion ? "smooth" : "auto";
        activeTab.scrollIntoView({ inline: "center", block: "nearest", behavior });
      };

      const activate = (nextIndex, focusTab, fromInteraction) => {
        activeIndex = nextIndex;

        tabs.forEach((tab, index) => {
          const isActive = index === nextIndex;
          tab.attribute({
            "aria-selected": isActive ? "true" : "false",
            tabindex: isActive ? "0" : "-1",
          });
          if (focusTab && isActive) tab.focus();
        });

        panels.forEach((panel, index) => {
          const isActive = index === nextIndex;
          panel.hidden = !isActive;
          panel.attribute("tabindex", isActive ? "0" : "-1");
        });

        syncActiveTabVisibility(fromInteraction);
        moveIndicator();
      };

      tabs.forEach((tab, index) => {
        tab.on("click", () => activate(index, false, true));
        tab.on("keydown", (event) => {
          let next = null;
          if (event.key === "ArrowRight" || event.key === "ArrowDown") {
            next = index === lastIndex ? 0 : index + 1;
          } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
            next = index === 0 ? lastIndex : index - 1;
          } else if (event.key === "Home") {
            next = 0;
          } else if (event.key === "End") {
            next = lastIndex;
          }

          if (next !== null) {
            event.preventDefault();
            activate(next, true, true);
          }
        });
      });

      tabList.on("scroll", moveIndicator);
      window.addEventListener("resize", moveIndicator, { passive: true });

      if (document.fonts && document.fonts.ready) {
        document.fonts.ready
          .then(() => requestAnimationFrame(moveIndicator))
          .catch(() => {});
      }

      activate(activeIndex, false, false);
    })();
}
