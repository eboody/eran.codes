use bon::Builder;
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct SectionHeader {
    pub title: String,
    pub subtitle: Option<String>,
    pub action: Option<maud::Markup>,
    pub meta: Option<maud::Markup>,
}

impl Render for SectionHeader {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="section-header" {
                div {
                    h2 { (&self.title) }
                    @if let Some(subtitle) = &self.subtitle {
                        p class="muted" { (subtitle) }
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
