use maud::{Markup, Render};

use crate::types::Text;
use crate::views::partials;

use super::content::{CmsActionLink, CtaKind};

mod case_detail;
mod flagship_crate_aside;
mod crate_section;
mod hero;
mod home_flow;
mod open_source_flow;
mod open_source_intro;
mod work;
mod work_flow;

pub use case_detail::Work as WorkCaseDetail;
pub use crate_section::CrateSection;
pub use flagship_crate_aside::FlagshipCrateHeroAside;
pub use hero::Portfolio as PortfolioHero;
pub use home_flow::HomeFlow;
pub use open_source_flow::OpenSourceFlow;
pub use open_source_intro::{OpenSourceHeroAside, OpenSourceMobileIntro};
pub use work::{Section as WorkSection, SectionVariant as WorkSectionVariant};
pub use work_flow::WorkFlow;

#[derive(Clone, Copy, Debug)]
pub(super) struct SectionCopy<'a> {
    pub title: &'a Text,
    pub subtitle: &'a Text,
}

impl Render for SectionCopy<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="u-section-copy ui-portfolio-section-copy" {
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

pub(super) struct CardGrid {
    extra_class: Option<&'static str>,
    content: Markup,
}

impl CardGrid {
    const BASE_CLASS: &str = "ui-portfolio-card-grid";

    pub fn new(content: Markup) -> Self {
        Self {
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
            None => Self::BASE_CLASS.to_string(),
        }
    }
}

impl Render for CardGrid {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        maud::html! {
            div class=(class_attr) {
                (&self.content)
            }
        }
    }
}

pub(super) struct InsetCard {
    extra_class: Option<&'static str>,
    content: Markup,
}

impl InsetCard {
    const BASE_CLASS: &str = "ui-portfolio-card u-inset-card";

    pub fn new(content: Markup) -> Self {
        Self {
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
            None => Self::BASE_CLASS.to_string(),
        }
    }
}

impl Render for InsetCard {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        maud::html! {
            article class=(class_attr) {
                (&self.content)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SurfaceTag {
    Header,
    Section,
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
        }
    }
}

pub(super) struct SectionActions<'a> {
    pub actions: &'a [CmsActionLink],
}

impl Render for SectionActions<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="ui-portfolio-section-actions" {
                (render_actions(self.actions))
            }
        }
    }
}

pub(super) struct CardFooter {
    content: Markup,
}

impl CardFooter {
    const BASE_CLASS: &str = "ui-portfolio-card-footer";

    pub fn new(content: Markup) -> Self {
        Self { content }
    }
}

impl Render for CardFooter {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class=(Self::BASE_CLASS) {
                (&self.content)
            }
        }
    }
}

pub(super) fn render_actions(actions: &[CmsActionLink]) -> partials::button::Row {
    partials::button::Row::builder()
        .density(partials::button::RowDensity::Compact)
        .frame(partials::button::RowFrame::Contained)
        .narrow_layout(partials::button::RowNarrowLayout::Stack)
        .items(
            actions
                .iter()
                .map(|action| {
                    partials::button::Button::builder()
                        .label(action.label.clone())
                        .variant(match action.tone {
                            CtaKind::Primary => partials::button::Variant::Primary,
                            CtaKind::Secondary => partials::button::Variant::Secondary,
                        })
                        .role(if action.kind.is_external() {
                            partials::button::Role::external_link(action.href.clone())
                        } else {
                            partials::button::Role::link(action.href.clone())
                        })
                        .build()
                })
                .collect(),
        )
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_actions_uses_contained_button_row_frame() {
        let markup = render_actions(&[CmsActionLink {
            label: Text::from("Inspect"),
            href: Text::from("/work"),
            kind: Default::default(),
            tone: CtaKind::Primary,
        }])
        .render()
        .into_string();

        assert!(markup.contains("data-button-row-frame=\"contained\""));
        assert!(markup.contains("data-button-row-narrow=\"stack\""));
    }
}
