use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::SectionHeader;

#[derive(Clone, Debug, Builder)]
pub struct EngineeringQuality {}

impl Render for EngineeringQuality {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section id="engineering-quality" class="ui-surface-card" {
                (SectionHeader::builder()
                    .title(Text::from("Engineering Quality"))
                    .subtitle(Text::from(
                        "Quick signals a lead Rust engineer can validate in this running app and in the repo.",
                    ))
                    .build())
                div class="ui-info-grid" {
                    article class="ui-info-card" {
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
                    article class="ui-info-card" {
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
                    article class="ui-info-card" {
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
