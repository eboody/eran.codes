use maud::Render;

use crate::views::partials::components::portfolio::content::OpenSourceIndexContent;

crate::views::scoped::inline_css!(
    r#"
me {
  --open-source-inline-size: min(104rem, calc(100vw - (var(--shell-gutter) * 2)));

  display: grid;
  gap: clamp(1.35rem, 1.05rem + 0.9vw, 2.1rem);
  inline-size: var(--open-source-inline-size);
  max-inline-size: none;
  margin-top: var(--space-5);
  margin-inline: calc((100% - var(--open-source-inline-size)) / 2);
}

me > .ui-portfolio-surface {
  margin-top: 0;
}

me .ui-portfolio-hero {
  max-inline-size: 92rem;
  padding: 0;
  border: none;
  border-radius: 0;
  background: none;
  box-shadow: none;
}

me .ui-portfolio-lead-surface {
  gap: var(--space-3);
}

me .ui-portfolio-lead-surface h1 {
  max-width: 18ch;
}

me .ui-open-source-hero-aside {
  gap: var(--space-3);
}

me .ui-open-source-hero-intro {
  display: grid;
  gap: var(--space-1);
}

me .ui-open-source-hero-intro p {
  margin: 0;
}

me .ui-open-source-hero-list {
  margin: 0;
  padding: 0;
  list-style: none;
  display: grid;
  gap: var(--space-2);
}

me .ui-open-source-hero-item {
  display: grid;
  gap: 0.35rem;
  padding-top: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 78%, transparent);
}

me .ui-open-source-hero-item strong,
me .ui-open-source-hero-footnote strong {
  font-size: var(--text-size-label-sm);
  letter-spacing: var(--text-track-ui);
}

me .ui-open-source-hero-item p,
me .ui-open-source-hero-footnote {
  margin: 0;
  color: var(--ui-text-muted);
}

me .ui-open-source-hero-item-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
}

me .ui-open-source-hero-item-tag {
  padding: 0.18rem 0.48rem;
  border-radius: var(--radius-pill);
  border: 1px solid color-mix(in srgb, var(--ui-border-soft) 76%, transparent);
  background: color-mix(in srgb, var(--ui-surface-soft) 92%, transparent);
  font-size: var(--text-size-label-2xs);
  letter-spacing: var(--text-track-caps-md);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-open-source-hero-footnote {
  padding-top: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 78%, transparent);
}

me .ui-open-source-mobile-intro {
  display: none;
}

me .ui-portfolio-crate-section--standalone {
  inline-size: 100%;
  max-inline-size: none;
  gap: var(--space-4);
  margin-inline: 0;
  padding: 0;
  border: none;
  background: none;
  box-shadow: none;
}

me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-panel {
  min-block-size: 47rem;
}

me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-showcase {
  align-content: start;
  min-block-size: 100%;
  padding: clamp(1.45rem, 1.18rem + 0.95vw, 2.2rem);
  gap: clamp(1.25rem, 1.02rem + 0.8vw, 1.9rem);
}

me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-gallery {
  padding: 0;
  border: none;
  background: transparent;
  box-shadow: none;
}

me .ui-portfolio-crate-section--standalone .tab-set__preview-frame[data-preview-kind="code"] {
  min-height: 0;
}

me .ui-portfolio-crate-section--standalone .tab-set__preview-code-stack {
  gap: var(--space-4);
}

me .ui-portfolio-crate-section--standalone .ui-code-block {
  --_code-block-font-size: var(--text-size-body-md);
  --_code-block-padding: clamp(1rem, 0.9rem + 0.4vw, 1.35rem);
}

@media (max-width: 64rem) {
  me {
    inline-size: 100%;
    margin-inline: 0;
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-panel {
    min-block-size: auto;
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-4);
    max-inline-size: none;
    margin-top: var(--space-4);
  }

  me .ui-portfolio-hero {
    max-inline-size: none;
  }

  me .ui-open-source-hero-aside {
    display: none;
  }

  me .ui-open-source-mobile-intro {
    display: grid;
    gap: var(--space-1);
    margin-top: var(--space-1);
    padding-top: var(--space-2);
    border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 72%, transparent);
  }

  me .ui-open-source-mobile-intro-eyebrow {
    margin: 0;
    font-size: var(--text-size-meta-xs);
    letter-spacing: var(--text-track-caps-sm);
    text-transform: uppercase;
    color: var(--ui-text-muted);
  }

  me .ui-open-source-mobile-intro .ui-portfolio-summary {
    font-size: var(--text-size-body-md);
    line-height: 1.55;
  }

  me .ui-portfolio-crate-section--standalone {
    gap: var(--space-2);
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-gallery {
    margin-top: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 72%, transparent);
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-showcase-copy,
  me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-showcase-header {
    gap: var(--space-1);
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-inline-links {
    gap: calc(var(--space-1) * 0.75);
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-inline-link {
    font-size: var(--text-size-meta-xs);
  }

  me .ui-portfolio-crate-section--standalone .ui-code-block {
    --_code-block-font-size: var(--text-size-meta-xs);
  }
}

@media (max-width: 26rem) {
  me .ui-portfolio-lead-surface h1 {
    max-width: 13.2ch;
    font-size: clamp(2rem, 1.72rem + 0.95vw, 2.3rem);
  }

  me .ui-portfolio-summary {
    font-size: var(--text-size-body-md);
    line-height: 1.52;
  }

  me .ui-open-source-mobile-intro {
    gap: calc(var(--space-1) * 0.75);
    margin-top: calc(var(--space-1) * 0.75);
    padding-top: var(--space-1);
  }

  me .ui-portfolio-badges {
    gap: calc(var(--space-1) * 0.85);
  }

  me .ui-portfolio-badges li {
    padding: 0.28rem 0.55rem;
  }
}
"#
);

use super::{CrateSection, OpenSourceHeroAside, OpenSourceMobileIntro, PortfolioHero};

pub struct OpenSourceFlow<'a> {
    pub content: &'a OpenSourceIndexContent,
}

impl Render for OpenSourceFlow<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-open-source-flow" {
                (css())
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: Some(maud::html! {
                        (OpenSourceHeroAside {
                            crate_section: &self.content.crate_section,
                        })
                    }),
                })
                (OpenSourceMobileIntro {
                    crate_section: &self.content.crate_section,
                })
                (CrateSection {
                    content: &self.content.crate_section,
                    show_heading: false,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::open_source_index_content;

    #[test]
    fn renders_open_source_flow_from_shared_components() {
        let content = open_source_index_content();
        let markup = OpenSourceFlow { content }.render().into_string();

        assert!(markup.contains("ui-portfolio-open-source-flow"));
        assert!(markup.contains("ui-open-source-hero-aside"));
        assert!(markup.contains("ui-open-source-mobile-intro"));
        assert!(markup.contains("data-portfolio-crate-switcher"));
    }
}
