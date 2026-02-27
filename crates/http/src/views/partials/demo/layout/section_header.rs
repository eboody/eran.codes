use bon::Builder;
use maud::Render;
use maud_extensions::css;
use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct SectionHeader {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub action: Option<maud::Markup>,
    pub meta: Option<maud::Markup>,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header data-section-header {
                div {
                    h2 { (&self.title) }
                    @if let Some(subtitle) = &self.subtitle {
                        p data-muted { (subtitle) }
                    }
                }
                @if let Some(action) = &self.action {
                    (action.clone())
                }
            }
            @if let Some(meta) = &self.meta {
                div data-section-meta { (meta.clone()) }
            }
            ({
                css! {
                    me [data-section-header] {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      justify-content: space-between;
                      gap: 0.9rem 1.2rem;
                      margin-bottom: 1.1rem;
                    }
                    me [data-section-header] h2 {
                      margin-bottom: 0.28rem;
                      font-size: clamp(1.5rem, 1.2rem + 1.1vw, 2rem);
                      line-height: 1.18;
                    }
                    me [data-section-header] [data-muted] {
                      margin-bottom: 0;
                      max-width: 70ch;
                    }
                    me [data-section-meta] {
                      margin-top: -0.5rem;
                      margin-bottom: 0.65rem;
                    }
                }
            })
        }
    }
}
