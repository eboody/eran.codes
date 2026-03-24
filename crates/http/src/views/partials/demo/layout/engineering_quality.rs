use bon::Builder;
use maud::Render;

use super::SurfaceSection;

crate::views::scoped::inline_css!(
    r#"
me [data-info-grid] {
  display: grid;
  gap: var(--space-4);
}

me [data-info-card] {
  --inset-card-padding: var(--space-card);

  display: grid;
  gap: var(--space-2);
  overflow: visible;
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me [data-info-card] h3 {
  margin: 0 0 var(--space-2);
}

me [data-info-card] p {
  margin: 0;
  font-size: var(--text-size-body-md);
  line-height: var(--text-line-body);
  color: var(--text-muted);
}

me [data-info-card] ul {
  margin: var(--space-3) 0 0;
  padding-left: var(--space-4);
  display: grid;
  gap: var(--space-2);
  font-size: var(--text-size-body-xs);
  line-height: var(--text-line-body);
  color: var(--text-muted);
}

@media (min-width: 980px) {
  me [data-info-grid] {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (hover: hover) {
  me [data-info-card]:hover {
    transform: var(--motion-lift-subtle);
    border-color: color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
    box-shadow: var(--shadow-panel-hover);
  }
}

@media (prefers-reduced-motion: reduce) {
  me [data-info-card] {
    transition: none;
  }
}
"#
);

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
                (css())
                div data-info-grid {
                    @for card in &content.engineering_quality.cards {
                        article class="u-inset-card" data-info-card {
                            h3 { (&card.title) }
                            p { (&card.summary) }
                            ul {
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
