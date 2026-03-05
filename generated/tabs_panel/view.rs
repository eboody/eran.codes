use maud::{html, Markup};

use crate::generated::tabs_panel::state::{TabContent, TabsPanelContent};

// BEGIN MDS GENERATED:component
pub fn tabs_panel_view(content: &TabsPanelContent) -> Markup {
    let initial_tab = content
        .tabs
        .first()
        .map(|tab| tab.id.as_str())
        .unwrap_or("tab_0");

    html! {
        section class="tabs-panel"
            data-signals={ format!("{{active_tab_id: '{}', server_connected: false}}", initial_tab) } {
            nav class="tabs-panel__tabs" role="tablist" aria-label="tabs" {
                @for tab in &content.tabs {
                    (render_tab_item(tab))
                }
            }
            article class="tabs-panel__content" {
                @for tab in &content.tabs {
                    @let visible_expr = format!("$active_tab_id == '{}'", tab.id);
                    section class="tabs-panel__panel" data-show=(visible_expr) {
                        @if let Some(detail) = &tab.detail {
                            h2 class="tabs-panel__title" { (&detail.title) }
                            @if let Some(subtitle) = &detail.subtitle {
                                p class="tabs-panel__subtitle" { (subtitle) }
                            }
                            ul class="tabs-panel__features" {
                                @for feature in &detail.features {
                                    li class="tabs-panel__feature" { (&feature.text) }
                                }
                            }
                        }
                        @if let Some(cta) = &tab.cta {
                            @if let Some(href) = &cta.href {
                                a class="tabs-panel__cta" href=(href) { (&cta.label) }
                            } @else {
                                button class="tabs-panel__cta" type="button" { (&cta.label) }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_tab_item(tab: &TabContent) -> Markup {
    let is_selected = format!("$active_tab_id == '{}'", tab.id);
    html! {
        button
            class="tabs-panel__tab"
            type="button"
            role="tab"
            data-tab-id=(&tab.id)
            data-class:is-selected=(is_selected)
            data-on:click="@dispatch('tab_select', {tab_id: $el.dataset.tabId}); $active_tab_id = $el.dataset.tabId" {
            span class="tabs-panel__tab-label-line" { (&tab.label.line_1) }
            @if let Some(line_2) = &tab.label.line_2 {
                span class="tabs-panel__tab-label-line" { (line_2) }
            }
        }
    }
}
// END MDS GENERATED:component
