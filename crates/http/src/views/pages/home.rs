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
                (partials::components::portfolio::WorkSection {
                    content: &content.work_section,
                })
                (partials::components::portfolio::CrateSection {
                    content: &content.crate_section,
                })
                (partials::components::portfolio::ClosingSection {
                    title: &content.closing.title,
                    summary: &content.closing.summary,
                    actions: &content.closing.actions,
                })
            },
        }
        .render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(body)
            .nav_mode(page::NavMode::Portfolio)
            .build()
            .render()
    }
}
