use bon::Builder;
use maud::Render;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct ActionLink {
    pub label: Text,
    pub href: Text,
    #[builder(default)]
    pub secondary: bool,
    #[builder(default)]
    pub external: bool,
}

impl Render for ActionLink {
    fn render(&self) -> maud::Markup {
        let class_name = if self.secondary {
            "button secondary"
        } else {
            "button"
        };
        maud::html! {
            @if self.external {
                a class=(class_name) href=(&self.href) target="_blank" rel="noopener noreferrer" {
                    (&self.label)
                }
            } @else {
                a class=(class_name) href=(&self.href) {
                    (&self.label)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct MetaText {
    pub text: Text,
}

impl Render for MetaText {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p data-muted { (&self.text) }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct SectionHeader {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub action: Option<ActionLink>,
    pub meta: Option<MetaText>,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="ui-section-header" data-section-header {
                div class="ui-section-header-copy" {
                    h2 { (&self.title) }
                    @if let Some(subtitle) = &self.subtitle {
                        p data-muted { (subtitle) }
                    }
                }
                @if let Some(action) = &self.action {
                    (action)
                }
            }
            @if let Some(meta) = &self.meta {
                div class="ui-section-meta" data-section-meta { (meta) }
            }
        }
    }
}
