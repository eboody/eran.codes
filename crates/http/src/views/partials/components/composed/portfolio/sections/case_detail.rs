use maud::Render;

use crate::views::partials::components::portfolio::content::{
    ArchiveDetailsContent, ArchivedWorkCaseContent, WorkCaseContent,
};

use super::{CardGrid, InsetCard, LeadCopy, SectionActions, SectionCopy, Surface};

pub struct Work<'a> {
    pub content: &'a WorkCaseContent,
}

impl Render for Work<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (LeadCopy {
                    eyebrow: &self.content.eyebrow,
                    title: &self.content.title,
                    summary: &self.content.summary,
                })
                @if !self.content.actions.is_empty() {
                    (SectionActions {
                        actions: &self.content.actions,
                    })
                }
            }).extra_class("ui-portfolio-lead-surface ui-portfolio-lead-surface--compact"))

            (case_sections(self.content, ArchiveHeading::Page))
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
                    p class="ui-portfolio-eyebrow" { (&self.content.eyebrow) }
                    h3 { (&self.content.title) }
                    p class="ui-portfolio-summary" { (&self.content.summary) }
                    @if !self.content.actions.is_empty() {
                        (SectionActions {
                            actions: &self.content.actions,
                        })
                    }
                }).extra_class("ui-portfolio-lead-surface"))
                (case_sections(self.content, ArchiveHeading::Archive))
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ArchiveHeading {
    Page,
    Archive,
}

fn case_sections(content: &WorkCaseContent, heading: ArchiveHeading) -> maud::Markup {
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
                    @match heading {
                        ArchiveHeading::Page => {
                            h2 { (title) }
                        }
                        ArchiveHeading::Archive => {
                            h4 { (title) }
                        }
                    }
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
