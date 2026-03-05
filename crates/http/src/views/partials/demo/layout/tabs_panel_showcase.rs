use bon::Builder;
use maud::Render;
use maud_extensions::inline_css;
use serde::Deserialize;

#[derive(Clone, Debug, Builder)]
pub struct TabsPanelShowcase {}

impl Render for TabsPanelShowcase {
    fn render(&self) -> maud::Markup {
        let content = load_content();
        let initial_tab_id = content
            .tabs
            .first()
            .map(|tab| tab.id.as_str())
            .unwrap_or("tab_0");

        maud::html! {
            section
                id="tabs-panel-showcase"
                class="tabs-panel-showcase"
                data-signals=(format!("{{active_tab_id: '{}'}}", initial_tab_id)) {
                (css())
                nav class="tabs-panel-showcase__tabs" role="tablist" aria-label="Solutions" {
                    @for tab in &content.tabs {
                        @let selected_expr = format!("$active_tab_id == '{}'", tab.id);
                        button
                            class="tabs-panel-showcase__tab"
                            type="button"
                            role="tab"
                            data-tab-id=(&tab.id)
                            data-class:is-selected=(selected_expr)
                            data-on:click=(format!("$active_tab_id = '{}'", tab.id)) {
                            span class="tabs-panel-showcase__tab-line" { (&tab.label.line_1) }
                            @if let Some(line_2) = &tab.label.line_2 {
                                span class="tabs-panel-showcase__tab-line" { (line_2) }
                            }
                        }
                    }
                }

                @for tab in &content.tabs {
                    @let show_expr = format!("$active_tab_id == '{}'", tab.id);
                    section
                        class="tabs-panel-showcase__panel"
                        role="tabpanel"
                        data-show=(show_expr) {
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

#[derive(Debug, Clone, Deserialize)]
struct TabsPanelContent {
    tabs: Vec<TabContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct TabContent {
    id: String,
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

    me > .tabs-panel-showcase__tabs {
      display: flex;
      gap: var(--size-2);
      overflow-x: auto;
      padding-bottom: 0.8rem;
      border-bottom: var(--border-size-1) solid var(--portfolio-surface-border);
    }

    me .tabs-panel-showcase__tab {
      display: inline-flex;
      align-items: center;
      gap: 0.35rem;
      padding: 0.55rem 0.85rem;
      border-radius: 999px;
      border: 1px solid var(--ui-border-soft);
      background: transparent;
      color: var(--ui-text-muted);
      cursor: pointer;
      font-size: 0.82rem;
      font-weight: 600;
      white-space: nowrap;
    }

    me .tabs-panel-showcase__tab.is-selected {
      color: var(--ui-text);
      border-color: color-mix(in srgb, var(--ui-text) 30%, transparent);
      background: color-mix(in srgb, var(--ui-surface-soft) 84%, transparent);
    }

    me .tabs-panel-showcase__tab-line + .tabs-panel-showcase__tab-line {
      margin-left: 0.2rem;
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
