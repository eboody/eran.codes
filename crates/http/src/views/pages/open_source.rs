use maud::Render;

use crate::views::{page, partials};

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
  max-inline-size: 82rem;
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
  max-width: 20ch;
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

me .ui-portfolio-crate-section--standalone [data-portfolio-crate-panel] {
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

  me .ui-portfolio-crate-section--standalone [data-portfolio-crate-panel] {
    min-block-size: auto;
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-3);
    max-inline-size: none;
    margin-top: var(--space-4);
  }

  me .ui-portfolio-hero {
    max-inline-size: none;
  }

  me .ui-portfolio-crate-section--standalone {
    gap: var(--space-2);
  }

  me .ui-portfolio-crate-section--standalone .ui-code-block {
    --_code-block-font-size: var(--text-size-meta-xs);
  }

}
"#
);

pub struct OpenSource;

impl Render for OpenSource {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::open_source_index_content();

        let body = partials::components::portfolio::Page {
            content: maud::html! {
                section class="ui-portfolio-open-source-flow" {
                    (css())
                    (partials::components::portfolio::PortfolioHero {
                        content: &content.hero,
                    })
                    (partials::components::portfolio::CrateSection {
                        content: &content.crate_section,
                        show_heading: false,
                    })
                }
            },
        }
        .render();
        let page_content = page::Frame::builder().content(body).build().render();

        page::Layout::builder()
            .title(&content.page_title.to_string())
            .content(page_content)
            .nav_mode(page::NavMode::Portfolio)
            .current_route(crate::paths::Route::OpenSource)
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_open_source_hero_and_crate_section() {
        let content = partials::components::portfolio::content::open_source_index_content();
        let markup = OpenSource.render().into_string();
        let hero_title = content.hero.title.to_string();

        assert!(markup.contains(hero_title.as_str()));
        assert!(markup.contains("data-portfolio-crate-switcher"));
        assert!(markup.contains("data-code-block"));
        assert!(!markup.contains("Open-source crate deep dives"));
        assert!(!markup.contains("Choose a crate"));
    }
}
