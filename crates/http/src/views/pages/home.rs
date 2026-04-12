use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::portfolio_shell;

#[derive(Builder, Default)]
pub struct Home {
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for Home {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::portfolio_home_content();
        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::HomeFlow { content })
            },
        }
        .render();
        portfolio_shell::render(
            &content.page_title.to_string(),
            body,
            crate::paths::Route::Home,
            self.user.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_flagship_crate_before_applied_proof() {
        let content = partials::components::portfolio::content::portfolio_home_content();
        let markup = Home::default().render().into_string();
        let proof_title = content.current_proof_section.title.to_string();
        let flagship_name = "statum";

        assert!(
            markup.find(flagship_name).unwrap() < markup.find(proof_title.as_str()).unwrap()
        );
        assert!(markup.contains("Published Rust crates with one live application proof behind them."));
        assert!(markup.contains("Flagship crate"));
        assert!(markup.contains("Current applied proof"));
        assert!(markup.contains("href=\"/resume.txt\""));
        assert!(markup.contains("ui-portfolio-hero-aside"));
        assert!(markup.contains("ui-portfolio-crate-showcase"));
        assert!(!markup.contains("Most relevant experience"));
    }
}
