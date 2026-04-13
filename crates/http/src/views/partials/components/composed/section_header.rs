#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::button;

#[derive(Clone, Copy, Debug, Default)]
pub enum SectionHeaderLevel {
    H1,
    #[default]
    H2,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Density {
    #[default]
    Standard,
    Compact,
}

#[derive(Clone, Debug, Builder)]
pub struct MetaText {
    pub text: Text,
}

impl Render for MetaText {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="u-muted" { (&self.text) }
        }
    }
}

#[derive(Clone, Debug, Builder)]
// ci: style-system-component
pub struct SectionHeader {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub action: Option<button::Button>,
    pub meta: Option<MetaText>,
    #[builder(default)]
    pub level: SectionHeaderLevel,
    #[builder(default)]
    pub density: Density,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        let class_name = match self.density {
            Density::Standard => "u-section-header",
            Density::Compact => "u-section-header u-section-header--compact",
        };

        maud::html! {
            header class=(class_name) data-section-header {
                div class="u-section-copy u-section-header-copy" data-section-header-copy {
                    @match self.level {
                        SectionHeaderLevel::H1 => h1 { (&self.title) },
                        SectionHeaderLevel::H2 => h2 { (&self.title) },
                    }
                    @if let Some(subtitle) = &self.subtitle {
                        p class="u-muted" { (subtitle) }
                    }
                }
                @if let Some(action) = &self.action {
                    div class="u-section-header-actions" data-section-header-actions {
                        (action)
                    }
                }
            }
            @if let Some(meta) = &self.meta {
                div class="u-section-meta" data-section-meta { (meta) }
            }
        }
    }
}
