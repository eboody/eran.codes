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
pub enum SectionHeaderDensity {
    #[default]
    Standard,
    Compact,
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
    #[builder(default)]
    pub density: SectionHeaderDensity,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        let class_name = match self.density {
            SectionHeaderDensity::Standard => "u-section-header",
            SectionHeaderDensity::Compact => "u-section-header u-section-header--compact",
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
            .action(
                button::Button::builder()
                    .label(Text::from("Inspect"))
                    .role(button::Role::link("/lab"))
                    .build(),
            )
            .build()
            .render()
            .into_string();

        assert!(markup.contains("<h1>Chat room</h1>"));
        assert!(markup.contains("data-section-header-actions"));
    }

    #[test]
    fn can_render_compact_density() {
        let markup = SectionHeader::builder()
            .title(Text::from("Sensitive record proof"))
            .density(SectionHeaderDensity::Compact)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("class=\"u-section-header u-section-header--compact\""));
    }
}
