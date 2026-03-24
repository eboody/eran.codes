use bon::Builder;

use crate::views::{page, partials};

#[derive(Builder)]
pub struct Lab {
    pub user: Option<page::UserNav>,
    pub chat_demo: Option<partials::chat::DemoSection>,
    pub sse_tab_id: Option<crate::types::SseTabId>,
}

impl maud::Render for Lab {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            (partials::LabFlow::builder()
                .maybe_user(self.user.clone())
                .maybe_chat_demo(self.chat_demo.clone())
                .build())
        };
        let content_model = partials::components::portfolio::content::lab_page_content();
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title(&content_model.page_title.to_string())
            .content(content)
            .sse_mode(page::SseMode::Enabled)
            .maybe_with_user(self.user.clone())
            .maybe_sse_tab_id(self.sse_tab_id.clone())
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn uses_shared_page_stack_contract() {
        let markup = Lab::builder().build().render().into_string();

        assert!(markup.contains("class=\"u-page-stack u-page-stack--spacious\""));
        assert!(markup.contains("data-lab-page"));
    }
}
