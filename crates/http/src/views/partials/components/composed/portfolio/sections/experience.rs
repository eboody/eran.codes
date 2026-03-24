use maud::Render;

use super::{SectionCopy, Surface, render_actions};
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
                div class="ui-portfolio-card-grid" {
                    @for role in &self.content.roles {
                        (ExperienceRoleCard { role })
                    }
                }
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
            article class="ui-portfolio-card ui-portfolio-experience-card u-inset-card" {
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
                    div class="ui-portfolio-work-card-footer" {
                        (render_actions(&self.role.actions))
                    }
                }
            }
        }
    }
}
