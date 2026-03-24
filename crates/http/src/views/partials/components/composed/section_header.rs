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

#[derive(Clone, Debug, Builder)]
pub struct SectionHeaderMetaText {
    pub text: Text,
}

impl Render for SectionHeaderMetaText {
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
    pub meta: Option<SectionHeaderMetaText>,
    #[builder(default)]
    pub level: SectionHeaderLevel,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="u-section-header" data-section-header {
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
                    (action)
                }
            }
            @if let Some(meta) = &self.meta {
                div class="u-section-meta" data-section-meta { (meta) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_h2_heading() {
        let markup = SectionHeader::builder()
            .title(Text::from("Live chat room"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<h2>Live chat room</h2>"));
    }

    #[test]
    fn can_render_h1_heading() {
        let markup = SectionHeader::builder()
            .title(Text::from("Chat room"))
            .level(SectionHeaderLevel::H1)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<h1>Chat room</h1>"));
    }
}
