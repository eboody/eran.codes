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
                    me {
                      --section-header-gap-row: var(--size-2);
                      --section-header-gap-column: var(--size-3);
                      --section-header-margin-bottom: var(--size-4);
                      --section-title-margin-bottom: var(--size-1);
                      --section-title-size: var(--font-size-fluid-2);
                      --section-title-line-height: var(--font-lineheight-1);
                      --section-subtitle-max-width: var(--size-content-3);
                      --section-meta-margin-top: calc(var(--size-2) * -1);
                      --section-meta-margin-bottom: var(--size-2);
                    }
                    me [data-section-header] {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      justify-content: space-between;
                      gap: var(--section-header-gap-row) var(--section-header-gap-column);
                      margin-bottom: var(--section-header-margin-bottom);
                    }
                    me [data-section-header] h2 {
                      margin-bottom: var(--section-title-margin-bottom);
                      font-size: var(--section-title-size);
                      line-height: var(--section-title-line-height);
                    }
                    me [data-section-header] [data-muted] {
                      margin-bottom: 0;
                      max-width: var(--section-subtitle-max-width);
                    }
                    me [data-section-meta] {
                      margin-top: var(--section-meta-margin-top);
                      margin-bottom: var(--section-meta-margin-bottom);
                    }
                }
            })
        }
    }
}
