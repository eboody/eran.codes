use bon::Builder;
use maud::Render;

use crate::views::partials::components::{CodeBlock, CodeLanguage, Tab};

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub(crate) struct Row {
    pub label: Text,
    pub value: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct Action {
    pub label: Text,
    pub href: Text,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct MockPanel {
    pub title: Text,
    pub subtitle: Text,
    pub rows: Vec<Row>,
}

#[derive(Clone, Debug, Builder)]
pub(crate) struct Panel {
    pub title: Text,
    pub subtitle: Text,
    pub bullets: Vec<Text>,
    pub mock_panel: Option<MockPanel>,
    pub chips_label: Text,
    pub chips: Vec<Text>,
    pub action: Option<Action>,
    pub code_path: Option<Text>,
    pub code: Option<Text>,
}

#[derive(Clone, Debug, Builder)]
pub(super) struct Component<'a> {
    pub tabs: &'a [Tab],
    pub panels: &'a [Panel],
}

impl Render for Component<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="showcase-panels" {
                @for (tab, panel) in self.tabs.iter().zip(self.panels.iter()) {
                    (render_panel(tab, panel))
                }
            }
        }
    }
}

fn render_panel(tab: &Tab, panel: &Panel) -> maud::Markup {
    let accent_style = format!("--showcase-tone-accent: {};", tab.palette.main.as_ref());
    let panel_class = if panel.mock_panel.is_none() {
        "showcase-panel showcase-panel--full"
    } else {
        "showcase-panel"
    };

    maud::html! {
        article
            class=(panel_class)
            role="tabpanel"
            id=(&tab.controls)
            aria-labelledby=(&tab.id)
            hidden[!tab.is_selected]
            tabindex=(if tab.is_selected { 0 } else { -1 })
            style=(accent_style)
        {
            @if let Some(mock_panel) = &panel.mock_panel {
                (render_mockup(mock_panel))
            }
            (render_copy(panel))
        }
    }
}

fn render_mockup(mock_panel: &MockPanel) -> maud::Markup {
    maud::html! {
        div class="showcase-mockup" {
            header {
                h3 { (&mock_panel.title) }
                p class="is-muted" { (&mock_panel.subtitle) }
            }
            dl class="showcase-rows" aria-label=(&mock_panel.title) {
                @for row in &mock_panel.rows {
                    div class="showcase-row" {
                        dt class="showcase-row-label" { (&row.label) }
                        dd class="showcase-row-value" { (&row.value) }
                    }
                }
            }
        }
    }
}

fn render_copy(panel: &Panel) -> maud::Markup {
    maud::html! {
        div class="showcase-copy" {
            div class="showcase-copy-content" {
                h3 { (&panel.title) }
                p class="is-muted" { (&panel.subtitle) }
                ul class="showcase-bullets" {
                    @for bullet in &panel.bullets {
                        li { (bullet) }
                    }
                }
                @if let Some(action) = &panel.action {
                    a class="button" href=(&action.href) { (&action.label) }
                }
                div class="showcase-integrations" {
                    p class="showcase-integrations-label" { (&panel.chips_label) }
                    ul class="showcase-chip-list" role="list" aria-label=(&panel.chips_label) {
                        @for chip in &panel.chips {
                            li class="showcase-chip" { (chip) }
                        }
                    }
                }
                @if let Some(path) = &panel.code_path {
                    p class="showcase-code-path" {
                        "Example: "
                        code { (path) }
                    }
                }
                @if let Some(code) = &panel.code {
                    (CodeBlock::builder()
                        .code(code.clone())
                        .language(CodeLanguage::Rust)
                        .build())
                }
            }
        }
    }
}
