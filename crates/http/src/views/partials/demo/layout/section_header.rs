use bon::Builder;
use maud::Render;
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
            header class="section-header" {
                div {
                    h2 { (self.title.to_string()) }
                    @if let Some(subtitle) = &self.subtitle {
                        p class="muted" { (subtitle.to_string()) }
                    }
                }
                @if let Some(action) = &self.action {
                    (action.clone())
                }
            }
            @if let Some(meta) = &self.meta {
                div class="section-meta" { (meta.clone()) }
            }
        }
    }
}
