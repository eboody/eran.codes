use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::SectionHeader;

crate::views::scoped::inline_css!(
    r#"
me [data-info-grid] {
  display: grid;
  gap: var(--size-4);
}

me [data-info-card] {
  overflow: visible;
  border: 1px solid var(--border-default);
  border-radius: calc(var(--radius-card) - 2px);
  padding: var(--space-card);
  background: var(--surface-fill-field);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me [data-info-card] h3 {
  margin: 0 0 0.45rem;
}

me [data-info-card] p {
  margin: 0;
  font-size: 0.92rem;
  line-height: 1.48;
  color: var(--text-muted);
}

me [data-info-card] ul {
  margin: var(--size-3) 0 0;
  padding-left: var(--size-4);
  display: grid;
  gap: var(--size-2);
  font-size: 0.86rem;
  line-height: 1.45;
  color: var(--text-muted);
}

@media (min-width: 980px) {
  me [data-info-grid] {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (prefers-color-scheme: dark) {
  me [data-info-card] {
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct EngineeringQuality {}

impl Render for EngineeringQuality {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section id="engineering-quality" class="u-surface-card" {
                (css())
                (SectionHeader::builder()
                    .title(Text::from("Engineering Quality"))
                    .subtitle(Text::from(
                        "Quick signals a lead Rust engineer can validate in this running app and in the repo.",
                    ))
                    .build())
                div data-info-grid {
                    article data-info-card {
                        h3 { "Layer boundaries stay explicit" }
                        p {
                            "Business policy, transport, and storage code are separated so feature work does not leak implementation details across layers."
                        }
                        ul {
                            li { "App services consume traits, not SQL primitives." }
                            li { "Domain types carry invariants at compile time." }
                            li { "HTTP handlers stay focused on protocol concerns." }
                        }
                    }
                    article data-info-card {
                        h3 { "Runtime behavior is inspectable" }
                        p {
                            "The page exposes observable runtime surfaces that make backend behavior debuggable during normal interaction."
                        }
                        ul {
                            li { "Live logs stream backend events without manual refresh." }
                            li { "Network traces show request shape and timing in context." }
                            li { "Interactive demos can be correlated against transport output." }
                        }
                    }
                    article data-info-card {
                        h3 { "Quality guardrails are enforced" }
                        p {
                            "Architecture and component conventions are checked by scripts so regressions are caught before merge."
                        }
                        ul {
                            li { "Datastar command and SSE conventions are verified in CI." }
                            li { "Component composition and naming rules are enforced." }
                            li { "CMS-shaped fixtures are validated as part of component flow." }
                        }
                    }
                }
            }
        }
    }
}
