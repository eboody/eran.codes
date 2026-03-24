use maud::Render;

use super::{SectionCopy, Surface};
use crate::views::partials::components::portfolio::content::{
    SkillGroupContent, SkillSectionContent,
};

pub struct SkillGroupsSection<'a> {
    pub content: &'a SkillSectionContent,
}

impl Render for SkillGroupsSection<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                div class="ui-portfolio-card-grid" {
                    @for group in &self.content.groups {
                        (SkillGroupCard { group })
                    }
                }
            }))
        }
    }
}

struct SkillGroupCard<'a> {
    group: &'a SkillGroupContent,
}

impl Render for SkillGroupCard<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            article class="ui-portfolio-card ui-portfolio-skill-card u-inset-card" {
                h3 { (&self.group.title) }
                ul class="ui-portfolio-list" {
                    @for item in &self.group.items {
                        li { (item) }
                    }
                }
            }
        }
    }
}
