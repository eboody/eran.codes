use maud::Render;

use super::{CardFooter, CardGrid, InsetCard, SectionCopy, Surface, render_actions};
use crate::views::partials::components::portfolio::content::{
    ExperienceRoleContent, ExperienceSectionContent,
};

pub struct ExperienceSection<'a> {
    pub content: &'a ExperienceSectionContent,
}

impl Render for ExperienceSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                (CardGrid::new(maud::html! {
                    @for role in &self.content.roles {
                        (ExperienceRoleCard { role })
                    }
                }))
            }))
        }
    }
}

struct ExperienceRoleCard<'a> {
    role: &'a ExperienceRoleContent,
}

impl Render for ExperienceRoleCard<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (InsetCard::new(maud::html! {
                p class="ui-portfolio-card-kicker" {
                    (&self.role.company) " · " (&self.role.tenure)
                }
                h3 { (&self.role.title) }
                p class="ui-portfolio-card-summary" { (&self.role.summary) }
                ul class="ui-portfolio-list" {
                    @for item in &self.role.highlights {
                        li { (item) }
                    }
                }
                @if !self.role.actions.is_empty() {
                    (CardFooter::new(maud::html! {
                        (render_actions(&self.role.actions))
                    }))
                }
            }).extra_class("ui-portfolio-experience-card"))
        }
    }
}
