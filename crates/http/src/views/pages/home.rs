use maud::Render;

use crate::views::{page, partials};

pub struct Home;

impl Render for Home {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::portfolio_home_content();

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::PortfolioHero { content: &content.hero })
                (partials::components::portfolio::ProofStrip {
                    content: &content.proof_strip,
                })
                (partials::components::portfolio::CrateSection {
                    content: &content.crate_section,
                    show_heading: true,
                })
                (partials::components::portfolio::WorkSection {
                    content: &content.work_section,
                })
                (partials::components::portfolio::ClosingSection {
                    title: &content.closing.title,
                    summary: &content.closing.summary,
                    actions: &content.closing.actions,
                })
            },
        }
        .render();
        let page_content = page::Frame::builder().content(body).build().render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(page_content)
            .nav_mode(page::NavMode::Portfolio)
            .current_route(crate::paths::Route::Home)
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_crate_section_before_work_section() {
        let content = partials::components::portfolio::content::portfolio_home_content();
        let markup = Home.render().into_string();
        let crate_title = content.crate_section.title.to_string();
        let work_title = content.work_section.title.to_string();

        assert!(
            markup.find(crate_title.as_str()).unwrap()
                < markup.find(work_title.as_str()).unwrap()
        );
    }
}
