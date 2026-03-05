use bon::Builder;
use maud::Render;
use maud_extensions::inline_css;
use maud_iconoir::regular;
use serde::Deserialize;

use crate::types::Text;
use crate::views::partials::components::{Tab, TabPanel};
use crate::views::proper_theme::THEME;

#[derive(Clone, Debug, Builder)]
pub struct TabsPanelShowcase {}

impl Render for TabsPanelShowcase {
    fn render(&self) -> maud::Markup {
        let content = load_content();
        let tabs = build_tabs(&content);

        maud::html! {
            section id="tabs-panel-showcase" class="tabs-panel-showcase" {
                (css())
                (TabPanel {
                    tabs: &tabs,
                    aria_label: Text::from("Solutions"),
                })
                @for (index, tab) in content.tabs.iter().enumerate() {
                    @let panel_id = panel_id(index);
                    section
                        class="tabs-panel-showcase__panel"
                        id=(panel_id)
                        role="tabpanel"
                        hidden[index != 0] {
                        div class="tabs-panel-showcase__preview" {
                            div class="tabs-panel-showcase__preview-frame" {
                                p class="tabs-panel-showcase__preview-label" { "Preview" }
                                @if let Some(preview) = &tab.preview {
                                    @if let Some(image) = &preview.image {
                                        p class="tabs-panel-showcase__preview-asset" {
                                            (&image.asset_ref)
                                        }
                                    }
                                    @if let Some(badge) = &preview.badge {
                                        p class="tabs-panel-showcase__badge" {
                                            (&badge.text)
                                        }
                                    }
                                }
                            }
                        }
                        div class="tabs-panel-showcase__copy" {
                            @if let Some(detail) = &tab.detail {
                                h2 { (&detail.title) }
                                @if let Some(subtitle) = &detail.subtitle {
                                    p class="tabs-panel-showcase__subtitle" { (subtitle) }
                                }
                                ul class="tabs-panel-showcase__features" {
                                    @for feature in &detail.features {
                                        li { (&feature.text) }
                                    }
                                }
                            }
                            @if let Some(cta) = &tab.cta {
                                @if let Some(href) = &cta.href {
                                    a class="button tabs-panel-showcase__cta" href=(href) {
                                        (&cta.label)
                                    }
                                } @else {
                                    button class="button tabs-panel-showcase__cta" type="button" {
                                        (&cta.label)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn build_tabs(content: &TabsPanelContent) -> Vec<Tab> {
    content
        .tabs
        .iter()
        .enumerate()
        .map(|(index, tab)| Tab {
            id: Text::from(tab_id(index)),
            controls: Text::from(panel_id(index)),
            palette: &THEME.yellow,
            is_selected: index == 0,
            icon_glyph: icon_for_index(index),
            text: Text::from(tab.label.full()),
        })
        .collect()
}

fn tab_id(index: usize) -> String {
    format!("tabs-panel-showcase-tab-{index}")
}

fn panel_id(index: usize) -> String {
    format!("tabs-panel-showcase-panel-{index}")
}

fn icon_for_index(index: usize) -> Option<maud::PreEscaped<&'static str>> {
    match index {
        0 => Some(regular::CHECK_CIRCLE),
        1 => Some(regular::SETTINGS),
        2 => Some(regular::KEY),
        3 => Some(regular::SHIELD_CHECK),
        4 => Some(regular::NETWORK_LEFT),
        5 => Some(regular::STATS_REPORT),
        _ => None,
    }
}

#[derive(Debug, Clone, Deserialize)]
struct TabsPanelContent {
    tabs: Vec<TabContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct TabContent {
    label: TabLabel,
    preview: Option<PreviewContent>,
    detail: Option<DetailContent>,
    cta: Option<CtaContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct TabLabel {
    line_1: String,
    line_2: Option<String>,
}

impl TabLabel {
    fn full(&self) -> String {
        match &self.line_2 {
            Some(line_2) => format!("{} {}", self.line_1, line_2),
            None => self.line_1.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PreviewContent {
    image: Option<AssetRef>,
    badge: Option<BadgeContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct AssetRef {
    asset_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BadgeContent {
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DetailContent {
    title: String,
    subtitle: Option<String>,
    features: Vec<FeatureContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct FeatureContent {
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CtaContent {
    label: String,
    href: Option<String>,
}

fn load_content() -> TabsPanelContent {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/cms/tabs_panel_showcase.json"
    ));
    serde_json::from_str(raw).unwrap_or_else(|_| TabsPanelContent { tabs: vec![] })
}

inline_css! {
    me {
      display: grid;
      gap: 1rem;
      margin-top: 1.5rem;
      padding: 1rem;
      border: 1px solid var(--portfolio-surface-border);
      border-radius: 18px;
      background: var(--portfolio-surface);
    }

    me > nav.showcase-tabs {
      border-bottom-color: var(--portfolio-surface-border);
      padding-bottom: 0.8rem;
    }

    me > .tabs-panel-showcase__panel {
      display: grid;
      gap: 1rem;
      align-items: start;
      grid-template-columns: 1.15fr 1fr;
      padding: 0.45rem 0.2rem 0.2rem;
    }

    me > .tabs-panel-showcase__panel[hidden] {
      display: none;
    }

    me > .tabs-panel-showcase__panel > .tabs-panel-showcase__preview {
      min-width: 0;
    }

    me > .tabs-panel-showcase__panel > .tabs-panel-showcase__preview > .tabs-panel-showcase__preview-frame {
      border: 1px solid var(--ui-border-soft);
      border-radius: 14px;
      min-height: 260px;
      background: color-mix(in srgb, var(--surface-shell) 90%, black 10%);
      padding: 1rem;
      display: grid;
      gap: 0.6rem;
      align-content: start;
    }

    me .tabs-panel-showcase__preview-label {
      margin: 0;
      font-size: 0.72rem;
      letter-spacing: 0.08rem;
      text-transform: uppercase;
      color: var(--ui-text-muted);
    }

    me .tabs-panel-showcase__preview-asset {
      margin: 0;
      font-size: 0.9rem;
      font-weight: 600;
    }

    me .tabs-panel-showcase__badge {
      margin: 0.5rem 0 0;
      width: fit-content;
      border-radius: 999px;
      padding: 0.35rem 0.65rem;
      border: 1px solid var(--ui-border-soft);
      font-size: 0.78rem;
      color: var(--ui-text-muted);
      background: var(--ui-surface-soft);
    }

    me .tabs-panel-showcase__copy h2 {
      margin: 0;
      font-size: 2rem;
      line-height: 1.1;
      letter-spacing: -0.02rem;
    }

    me .tabs-panel-showcase__subtitle {
      margin: 0.6rem 0 0;
      color: var(--ui-text-muted);
      max-width: 52ch;
    }

    me .tabs-panel-showcase__features {
      margin: 1rem 0 1.1rem;
      padding-left: 1.1rem;
      display: grid;
      gap: 0.45rem;
    }

    me .tabs-panel-showcase__cta {
      width: fit-content;
      min-width: 10rem;
      justify-content: center;
    }

    @media (max-width: 980px) {
      me > .tabs-panel-showcase__panel {
        grid-template-columns: 1fr;
      }

      me .tabs-panel-showcase__copy h2 {
        font-size: 1.65rem;
      }
    }
}
