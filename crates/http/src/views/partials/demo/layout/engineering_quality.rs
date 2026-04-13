mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::Render;

use super::SurfaceSection;

#[derive(Clone, Debug, Builder)]
pub struct EngineeringQuality {}

impl Render for EngineeringQuality {
    fn render(&self) -> maud::Markup {
        let content = crate::views::partials::components::portfolio::content::lab_page_content();

        SurfaceSection::builder()
            .id(crate::types::Text::from("engineering-quality"))
            .title(content.engineering_quality.title.clone())
            .subtitle(content.engineering_quality.subtitle.clone())
            .content(maud::html! {
                (styles::render())
                div data-engineering-quality-grid {
                    @for card in &content.engineering_quality.cards {
                        article class="u-inset-card" data-engineering-quality-card {
                            h3 data-engineering-quality-card-title { (&card.title) }
                            p data-engineering-quality-card-summary { (&card.summary) }
                            ul data-engineering-quality-card-points {
                                @for point in &card.points {
                                    li { (point) }
                                }
                            }
                        }
                    }
                }
            })
            .build()
            .render()
    }
}
