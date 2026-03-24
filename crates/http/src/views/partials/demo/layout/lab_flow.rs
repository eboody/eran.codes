use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

use super::{
    EngineeringQuality, GuestChatFallback, HomeHero, OperationsSurface, RequestBurstDemo,
    SensitiveProofPanel, TabSetShowcase,
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
                (TabSetShowcase::builder().build())
                (RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())
                (SensitiveProofPanel::builder().build())
                @if let Some(chat_demo) = &self.chat_demo {
                    (chat_demo.render())
                } @else {
                    (GuestChatFallback::builder().build())
                }
                (OperationsSurface::builder().build())
                (EngineeringQuality::builder().build())
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
        assert!(markup.contains("High-Volume Request Burst"));
        assert!(markup.contains("Sensitive record proof"));
    }
}
