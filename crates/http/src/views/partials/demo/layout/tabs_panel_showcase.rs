use bon::Builder;
use maud::Render;
use serde::Deserialize;

// ci: style-system-component
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
                class="tabs-panel-showcase ui-surface-card"
                data-signals=(format!("{{active_tab_id: '{}'}}", initial_tab_id)) {
                nav class="tabs-panel-showcase__tabs ui-tabs" role="tablist" aria-label="Solutions" {
                    @for tab in &content.tabs {
                        @let selected_expr = format!("$active_tab_id == '{}'", tab.id);
                        button
                            class="tabs-panel-showcase__tab ui-tab"
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
                        class="tabs-panel-showcase__panel ui-panel"
                        role="tabpanel"
                        data-show=(show_expr) {
                        div class="tabs-panel-showcase__preview" {
                            div class="tabs-panel-showcase__preview-frame ui-preview-frame" {
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
                                ul class="tabs-panel-showcase__features ui-feature-list" {
                                    @for feature in &detail.features {
                                        li { (&feature.text) }
                                    }
                                }
                            }
                            @if let Some(cta) = &tab.cta {
                                @if let Some(href) = &cta.href {
                                    a class="button tabs-panel-showcase__cta ui-cta" href=(href) {
                                        (&cta.label)
                                    }
                                } @else {
                                    button class="button tabs-panel-showcase__cta ui-cta" type="button" {
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
