use maud::{Markup, Render};

use crate::types::Text;
use crate::views::partials::button;

use super::content::{CmsActionLink, CtaKind, ProofKind};

mod case_detail;
mod closing;
mod crate_section;
mod hero;
mod proof_strip;
mod work;

pub use case_detail::WorkCaseDetail;
pub use closing::ClosingSection;
pub use crate_section::CrateSection;
pub use hero::PortfolioHero;
pub use proof_strip::ProofStrip;
pub use work::{WorkIndexSection, WorkSection};

#[derive(Clone, Copy, Debug)]
pub(super) struct SectionCopy<'a> {
    pub title: &'a Text,
    pub subtitle: &'a Text,
}

impl Render for SectionCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-portfolio-section-copy" {
                h2 { (&self.title) }
                p { (&self.subtitle) }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct LeadCopy<'a> {
    pub eyebrow: &'a Text,
    pub title: &'a Text,
    pub summary: &'a Text,
}

impl Render for LeadCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            p class="ui-portfolio-eyebrow" { (&self.eyebrow) }
            h1 { (&self.title) }
            p class="ui-portfolio-summary" { (&self.summary) }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SurfaceTag {
    Header,
    Section,
    Article,
}

pub(super) struct Surface {
    tag: SurfaceTag,
    extra_class: Option<&'static str>,
    content: Markup,
}

impl Surface {
    const BASE_CLASS: &str = "u-surface-card ui-portfolio-surface";

    pub fn header(content: Markup) -> Self {
        Self {
            tag: SurfaceTag::Header,
            extra_class: None,
            content,
        }
    }

    pub fn section(content: Markup) -> Self {
        Self {
            tag: SurfaceTag::Section,
            extra_class: None,
            content,
        }
    }

    pub fn article(content: Markup) -> Self {
        Self {
            tag: SurfaceTag::Article,
            extra_class: None,
            content,
        }
    }

    pub fn extra_class(mut self, class: &'static str) -> Self {
        self.extra_class = Some(class);
        self
    }

    fn class_attr(&self) -> String {
        match self.extra_class {
            Some(extra_class) => format!("{} {extra_class}", Self::BASE_CLASS),
            None => String::from(Self::BASE_CLASS),
        }
    }
}

impl Render for Surface {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        match self.tag {
            SurfaceTag::Header => maud::html! {
                header class=(class_attr) {
                    (&self.content)
                }
            },
            SurfaceTag::Section => maud::html! {
                section class=(class_attr) {
                    (&self.content)
                }
            },
            SurfaceTag::Article => maud::html! {
                article class=(class_attr) {
                    (&self.content)
                }
            },
        }
    }
}

pub(super) fn render_actions(actions: &[CmsActionLink]) -> button::Row {
    button::Row::builder()
        .items(
            actions
                .iter()
                .map(|action| {
                    button::Button::builder()
                        .label(action.label.clone())
                        .variant(match action.tone {
                            CtaKind::Primary => button::Variant::Primary,
                            CtaKind::Secondary => button::Variant::Secondary,
                        })
                        .role(if action.kind.is_external() {
                            button::Role::external_link(action.href.clone())
                        } else {
                            button::Role::link(action.href.clone())
                        })
                        .build()
                })
                .collect(),
        )
        .build()
}

pub(super) fn proof_kind_attr(kind: ProofKind) -> &'static str {
    match kind {
        ProofKind::Outcome => "outcome",
        ProofKind::Architecture => "architecture",
        ProofKind::Reliability => "reliability",
    }
}
