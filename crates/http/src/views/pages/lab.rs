use bon::Builder;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.2rem, 0.9rem + 1.2vw, 2rem);
  padding-bottom: calc(var(--space-section) + var(--space-7));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
}
"#
);

#[derive(Builder)]
pub struct Lab {
    pub user: Option<page::UserNav>,
    pub chat_demo: Option<partials::chat::DemoSection>,
    pub sse_tab_id: Option<crate::types::SseTabId>,
}

impl maud::Render for Lab {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            div data-lab-page data-page-section {
                (css())
                (partials::HomeHero::builder().maybe_user(self.user.clone()).build())

                (partials::TabSetShowcase::builder().build())

                (partials::RequestBurstDemo::builder()
                    .endpoint(Text::from(Route::PartialRequestBurstProbe.as_str()))
                    .build())

                (partials::SensitiveProofPanel::builder().build())

                @if let Some(chat_demo) = &self.chat_demo { (chat_demo.render()) } @else {
                    (partials::GuestChatFallback::builder().build())
                    }

                (partials::OperationsSurface::builder().build())

                (partials::EngineeringQuality::builder().build())
            }
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
