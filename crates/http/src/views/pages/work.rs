use maud::Render;

use crate::views::{page, partials};

pub struct Work;

impl Render for Work {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::work_index_content();

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                (partials::components::portfolio::WorkIndexSection { content })
                (partials::components::portfolio::SupportingTeaserSection {
                    content: &content.open_source_teaser,
                })
            },
        }
        .render();
        let page_content = page::Frame::builder().content(body).build().render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(page_content)
            .nav_mode(page::NavMode::Portfolio)
            .current_route(crate::paths::Route::Work)
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_case_studies_and_open_source_teaser() {
        let content = partials::components::portfolio::content::work_index_content();
        let markup = Work.render().into_string();
        let lead_title = content.title.to_string();
        let cases_title = content.cases_title.to_string();
        let teaser_title = content.open_source_teaser.title.to_string();

        assert!(markup.contains(lead_title.as_str()));
        assert!(markup.contains(cases_title.as_str()));
        assert!(markup.contains(teaser_title.as_str()));
        assert!(!markup.contains("data-code-block"));
        assert!(!markup.contains("data-portfolio-crate-switcher"));
        assert!(!markup.contains("images/work/chat-realtime"));
        assert!(!markup.contains("Automation at scale case preview"));
        assert!(
            markup.find(lead_title.as_str()).unwrap()
                < markup.find(cases_title.as_str()).unwrap()
        );
        assert!(
            markup.find(cases_title.as_str()).unwrap() < markup.find(teaser_title.as_str()).unwrap()
        );
    }
}
