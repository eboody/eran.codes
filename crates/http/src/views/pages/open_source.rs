use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::portfolio_shell;

#[derive(Builder, Default)]
pub struct OpenSource {
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for OpenSource {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::open_source_index_content();
        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::OpenSourceFlow { content })
            },
        }
        .render();
        portfolio_shell::render_with_frame_width(
            &content.page_title.to_string(),
            body,
            crate::paths::Route::OpenSource,
            self.user.clone(),
            page::FrameWidth::Wide,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_open_source_hero_and_crate_section() {
        let content = partials::components::portfolio::content::open_source_index_content();
        let markup = OpenSource::default().render().into_string();
        let hero_title = content.hero.title.to_string();

        assert!(markup.contains(hero_title.as_str()));
        assert!(markup.contains("Library proof"));
        assert!(markup.contains("Three crates. One invariants-first through-line."));
        assert!(markup.contains("What to inspect"));
        assert!(markup.contains("ui-open-source-hero-item-tag"));
        assert!(markup.contains("data-portfolio-crate-switcher"));
        assert!(markup.contains("data-code-block"));
        assert!(markup.contains("ui-portfolio-hero-aside"));
        assert!(markup.contains("u-container--wide"));
        assert!(!markup.contains("Open-source crate deep dives"));
    }
}
