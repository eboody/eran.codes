use maud::Render;

use super::{CardGrid, InsetCard, SectionCopy, Surface};
use crate::views::partials::components::portfolio::content::{
    SkillGroupContent, SkillSectionContent,
};

pub struct Section<'a> {
    pub content: &'a SkillSectionContent,
}

impl Render for Section<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (SectionCopy {
                    title: &self.content.title,
                    subtitle: &self.content.subtitle,
                })
                (CardGrid::new(maud::html! {
                    @for group in &self.content.groups {
                        (SkillGroupCard { group })
                    }
                }))
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
            (InsetCard::new(maud::html! {
                h3 { (&self.group.title) }
                ul class="ui-portfolio-list" {
                    @for item in &self.group.items {
                        li { (item) }
                    }
                }
            }).extra_class("ui-portfolio-skill-card"))
        }
    }
}
