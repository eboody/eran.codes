use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::{
    HomeHero, SensitiveProofPanel, SupportingProofTabs,
};

#[derive(Builder)]
pub struct LabFlow {
    pub user: Option<page::UserNav>,
    pub chat_demo: Option<partials::chat::DemoSection>,
}

impl Render for LabFlow {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="u-page-stack u-page-stack--spacious" data-lab-page data-page-section {
                (HomeHero::builder().maybe_user(self.user.clone()).build())
                (SensitiveProofPanel::builder().build())
                (SupportingProofTabs::builder()
                    .maybe_chat_demo(self.chat_demo.clone())
                    .build())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_shared_lab_flow_stack() {
        let markup = LabFlow::builder().build().render().into_string();

        assert!(markup.contains("class=\"u-page-stack u-page-stack--spacious\""));
        assert!(markup.contains("data-lab-page"));
        assert!(markup.contains("Sensitive record proof"));
        assert!(markup.contains("Validate the main proof from other angles"));
        assert!(!markup.contains("Engineering Quality"));
    }
}
