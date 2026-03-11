use maud::Render;

use crate::views::page::{Layout, NavMode};
use crate::views::partials::components::portfolio;

pub struct Home;

impl Render for Home {
    fn render(&self) -> maud::Markup {
        let content = portfolio::content::portfolio_home_content();

        let body = portfolio::Page {
            content: maud::html! {
                (portfolio::PortfolioHero { content: &content.hero })
                (portfolio::ProofStrip {
                    content: &content.proof_strip,
                })
                (portfolio::WorkSection {
                    content: &content.work_section,
                })
                (portfolio::CrateSection {
                    content: &content.crate_section,
                })
                (portfolio::ClosingSection {
                    title: &content.closing.title,
                    summary: &content.closing.summary,
                    actions: &content.closing.actions,
                })
            },
        }
        .render();

        Layout::builder()
            .title(&content.page_title.to_string())
            .content(body)
            .nav_mode(NavMode::Portfolio)
            .build()
            .render()
    }
}
