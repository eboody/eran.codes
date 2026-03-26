use maud::Render;

use crate::views::partials::components::portfolio::content::{
    ArchiveDetailsContent, ArchivedWorkCaseContent, WorkCaseContent, WorkCaseDetailLayout,
};

use super::{CardGrid, InsetCard, PortfolioHero, SectionActions, SectionCopy, Surface};

pub struct Work<'a> {
    pub content: &'a WorkCaseContent,
}

impl Render for Work<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section class="ui-portfolio-hero-flow ui-portfolio-work-case-flow" {
                (PortfolioHero {
                    content: &self.content.hero,
                    aside: None,
                })
                (case_sections(self.content))
            }
        }
    }
}

pub struct ArchiveCaseDetailsSection<'a> {
    pub intro: &'a ArchiveDetailsContent,
    pub cases: &'a [ArchivedWorkCaseContent],
}

impl Render for ArchiveCaseDetailsSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.intro.title,
                    subtitle: &self.intro.subtitle,
                })
            }).extra_class("ui-portfolio-case-archive-intro"))
            div class="ui-portfolio-showcase-stack" {
                @for case in self.cases {
                    (ArchivedWork {
                        slug: case.slug,
                        content: &case.content,
                        entry_label: &self.intro.entry_label,
                    })
                }
            }
        }
    }
}

struct ArchivedWork<'a> {
    slug: crate::views::partials::components::portfolio::content::WorkCaseSlug,
    content: &'a WorkCaseContent,
    entry_label: &'a crate::types::Text,
}

impl Render for ArchivedWork<'_> {
    fn render(&self) -> maud::Markup {
        let anchor_id = self
            .slug
            .archive_anchor_id()
            .expect("archived work case should expose an archive anchor id");

        maud::html! {
            div id=(anchor_id) class="ui-portfolio-showcase-stack ui-portfolio-archive-entry" {
                p class="ui-portfolio-eyebrow ui-portfolio-archive-note" {
                    (self.entry_label)
                }
                (Surface::section(maud::html! {
                    p class="ui-portfolio-eyebrow" { (&self.content.hero.eyebrow) }
                    h3 { (&self.content.hero.title) }
                    p class="ui-portfolio-summary" { (&self.content.hero.summary) }
                    @if !self.content.hero.actions.is_empty() {
                        (SectionActions {
                            actions: &self.content.hero.actions,
                        })
                    }
                }).extra_class("ui-portfolio-lead-surface ui-portfolio-lead-surface--compact"))
            }
        }
    }
}

fn case_sections(content: &WorkCaseContent) -> maud::Markup {
    match content.detail_layout {
        WorkCaseDetailLayout::CurrentProof => current_proof_sections(content),
        WorkCaseDetailLayout::ArchiveGrid => archive_grid_sections(content),
    }
}

fn archive_grid_sections(content: &WorkCaseContent) -> maud::Markup {
    let sections = [
        (&content.challenge.title, &content.challenge.items),
        (&content.implementation.title, &content.implementation.items),
        (&content.outcomes.title, &content.outcomes.items),
        (&content.stack.title, &content.stack.items),
    ];

    maud::html! {
        (CardGrid::new(maud::html! {
            @for (title, items) in sections {
                (InsetCard::new(maud::html! {
                    h2 { (title) }
                    ul class="ui-portfolio-list" {
                        @for item in items {
                            li { (item) }
                        }
                    }
                }).extra_class("ui-portfolio-case-section"))
            }
        }).extra_class("ui-portfolio-case-grid"))
    }
}

fn current_proof_sections(content: &WorkCaseContent) -> maud::Markup {
    maud::html! {
        section class="ui-portfolio-current-proof-detail" {
            div class="ui-portfolio-current-proof-main" {
                (InsetCard::new(maud::html! {
                    h2 { (&content.outcomes.title) }
                    ul class="ui-portfolio-list" {
                        @for item in &content.outcomes.items {
                            li { (item) }
                        }
                    }
                }).extra_class("ui-portfolio-case-section ui-portfolio-case-section--lead"))
                (InsetCard::new(maud::html! {
                    h2 { (&content.implementation.title) }
                    ul class="ui-portfolio-list" {
                        @for item in &content.implementation.items {
                            li { (item) }
                        }
                    }
                }).extra_class("ui-portfolio-case-section"))
            }
            aside class="ui-portfolio-current-proof-rail" {
                (InsetCard::new(maud::html! {
                    h2 { "Boundary and scope" }
                    ul class="ui-portfolio-list" {
                        @for item in &content.challenge.items {
                            li { (item) }
                        }
                    }
                }).extra_class("ui-portfolio-case-section ui-portfolio-current-proof-rail-card"))
                (InsetCard::new(maud::html! {
                    h2 { (&content.stack.title) }
                    ul class="ui-portfolio-badges ui-portfolio-current-proof-stack" {
                        @for item in &content.stack.items {
                            li { (item) }
                        }
                    }
                }).extra_class("ui-portfolio-case-section ui-portfolio-current-proof-rail-card"))
            }
        }
    }
}
