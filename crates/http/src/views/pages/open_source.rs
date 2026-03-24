use bon::Builder;
use maud::Render;

use crate::views::{page, partials};

use super::portfolio_shell;

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

me [data-open-source-hero-aside] {
  gap: var(--space-3);
}

me [data-open-source-hero-intro] {
  display: grid;
  gap: var(--space-1);
}

me [data-open-source-hero-intro] p {
  margin: 0;
}

me [data-open-source-hero-list] {
  margin: 0;
  padding: 0;
  list-style: none;
  display: grid;
  gap: var(--space-2);
}

me [data-open-source-hero-item] {
  display: grid;
  gap: 0.35rem;
  padding-top: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 78%, transparent);
}

me [data-open-source-hero-item] strong,
me [data-open-source-hero-footnote] strong {
  font-size: var(--text-size-label-sm);
  letter-spacing: var(--text-track-ui);
}

me [data-open-source-hero-item] p,
me [data-open-source-hero-footnote] {
  margin: 0;
  color: var(--ui-text-muted);
}

me [data-open-source-hero-item-tags] {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
}

me [data-open-source-hero-item-tag] {
  padding: 0.18rem 0.48rem;
  border-radius: var(--radius-pill);
  border: 1px solid color-mix(in srgb, var(--ui-border-soft) 76%, transparent);
  background: color-mix(in srgb, var(--ui-surface-soft) 92%, transparent);
  font-size: var(--text-size-label-2xs);
  letter-spacing: var(--text-track-caps-md);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me [data-open-source-hero-footnote] {
  padding-top: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 78%, transparent);
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

  me [data-open-source-hero-aside] {
    display: none;
  }

  me .ui-portfolio-crate-section--standalone {
    gap: var(--space-2);
  }

  me .ui-portfolio-crate-section--standalone .ui-portfolio-crate-gallery {
    margin-top: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 72%, transparent);
  }

  me .ui-portfolio-crate-section--standalone .ui-code-block {
    --_code-block-font-size: var(--text-size-meta-xs);
  }

}
"#
);

#[derive(Builder, Default)]
pub struct OpenSource {
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for OpenSource {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::open_source_index_content();
        let hero_aside = maud::html! {
            div data-open-source-hero-aside {
                div data-open-source-hero-intro {
                    p class="ui-portfolio-hero-aside-kicker" { "Library proof" }
                    h2 { "Three crates. One invariants-first through-line." }
                    p class="ui-portfolio-card-summary" {
                        "Typestate, nested enum modeling, and namespace discipline packaged as reusable Rust APIs."
                    }
                }
                ul data-open-source-hero-list {
                    @for card in content.crate_section.cards.iter().take(3) {
                        li data-open-source-hero-item {
                            strong { (&card.name) }
                            div data-open-source-hero-item-tags {
                                @for tag in card.tags.iter().take(2) {
                                    span data-open-source-hero-item-tag { (tag) }
                                }
                            }
                        }
                    }
                }
                p data-open-source-hero-footnote {
                    strong { "What to inspect" }
                    " Read the API, then check the code and docs against the same published surface."
                }
            }
        };

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                section class="ui-portfolio-open-source-flow" {
                    (css())
                    (partials::components::portfolio::PortfolioHero {
                        content: &content.hero,
                        aside: Some(hero_aside),
                    })
                    (partials::components::portfolio::CrateSection {
                        content: &content.crate_section,
                        show_heading: false,
                    })
                }
            },
        }
        .render();
        portfolio_shell::render(
            &content.page_title.to_string(),
            body,
            crate::paths::Route::OpenSource,
            self.user.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_open_source_hero_and_crate_section() {
        let content = partials::components::portfolio::content::open_source_index_content();
        let markup = OpenSource::default().render().into_string();
        let hero_title = content.hero.title.to_string();

        assert!(markup.contains(hero_title.as_str()));
        assert!(markup.contains("Library proof"));
        assert!(markup.contains("Three crates. One invariants-first through-line."));
        assert!(markup.contains("What to inspect"));
        assert!(markup.contains("data-open-source-hero-item-tag"));
        assert!(markup.contains("data-portfolio-crate-switcher"));
        assert!(markup.contains("data-code-block"));
        assert!(markup.contains("ui-portfolio-hero-aside"));
        assert!(!markup.contains("Open-source crate deep dives"));
        assert!(!markup.contains("Choose a crate"));
    }
}
