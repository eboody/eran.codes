use maud::{Markup, Render};

use super::links::Crate as Links;
use crate::views::partials::components::portfolio::content::{
    CrateCardContent, CrateGalleryContent,
};
use crate::views::partials::components::tab_set;

pub(super) struct Crate<'a> {
    pub card: &'a CrateCardContent,
}

impl Render for Crate<'_> {
    fn render(&self) -> maud::Markup {
        let gallery = self.card.gallery.as_ref().map(render_gallery);

        maud::html! {
            article class="ui-portfolio-crate-showcase" {
                div class="ui-portfolio-crate-showcase-header" {
                    div class="ui-portfolio-crate-showcase-copy" {
                        h3 { (&self.card.name) }
                        p class="ui-portfolio-card-summary" { (&self.card.summary) }
                    }
                    (Links { card: self.card })
                }
                @if let Some(gallery) = gallery {
                    (gallery)
                }
            }
        }
    }
}

fn render_gallery(content: &CrateGalleryContent) -> Markup {
    let gallery_id = content.id.to_string();
    let gallery_content = content.tab_set_content();

    tab_set::Component::from_content(
        tab_set::ContentProps::builder()
            .id(gallery_id.as_str())
            .class("ui-portfolio-crate-gallery")
            .aria_label(content.aria_label.clone())
            .variant(tab_set::Variant::FlatGallery)
            .content(&gallery_content)
            .build(),
    )
    .render()
}
