use maud::Render;

use super::{CardFooter, CardGrid, InsetCard, SectionActions, SectionCopy, Surface};
use crate::types::Text;
use crate::views::partials;
use crate::views::partials::components::portfolio::content::{
    WorkCardContent, WorkSectionContent,
};

#[derive(Clone, Copy, Debug)]
enum ActionsPlacement {
    AfterCards,
    WithCopy,
}

#[derive(Clone, Copy, Debug)]
struct SectionLayout {
    root_class: &'static str,
    copy_wrapper_class: Option<&'static str>,
    actions_placement: ActionsPlacement,
}

const STANDARD_LAYOUT: SectionLayout = SectionLayout {
    root_class: "ui-portfolio-work-section",
    copy_wrapper_class: None,
    actions_placement: ActionsPlacement::AfterCards,
};
const CURRENT_PROOF_LAYOUT: SectionLayout = SectionLayout {
    root_class: "ui-portfolio-work-section ui-portfolio-work-section--current-proof",
    copy_wrapper_class: Some("ui-portfolio-work-section-rail"),
    actions_placement: ActionsPlacement::WithCopy,
};

struct SectionActionsMarkup<'a> {
    actions: &'a [crate::views::partials::components::portfolio::content::CmsActionLink],
}

pub struct Section<'a> {
    content: &'a WorkSectionContent,
    layout: SectionLayout,
}

impl<'a> Section<'a> {
    pub fn standard(content: &'a WorkSectionContent) -> Self {
        Self {
            content,
            layout: STANDARD_LAYOUT,
        }
    }

    pub fn current_proof(content: &'a WorkSectionContent) -> Self {
        Self {
            content,
            layout: CURRENT_PROOF_LAYOUT,
        }
    }

    fn render_copy(&self) -> maud::Markup {
        maud::html! {
            (SectionCopy {
                title: &self.content.title,
                subtitle: &self.content.subtitle,
            })
        }
    }

    fn render_actions(&self) -> Option<SectionActionsMarkup<'_>> {
        (!self.content.actions.is_empty()).then_some(SectionActionsMarkup {
            actions: &self.content.actions,
        })
    }

    fn render_copy_block(&self) -> maud::Markup {
        let copy = self.render_copy();
        let actions = self.render_actions();

        let content = maud::html! {
            (copy)
            @if matches!(self.layout.actions_placement, ActionsPlacement::WithCopy) {
                @if let Some(actions) = actions {
                    (actions)
                }
            }
        };

        match self.layout.copy_wrapper_class {
            Some(class) => maud::html! { div class=(class) { (content) } },
            None => content,
        }
    }

    fn render_trailing_actions(&self) -> Option<SectionActionsMarkup<'_>> {
        matches!(self.layout.actions_placement, ActionsPlacement::AfterCards)
            .then(|| self.render_actions())
            .flatten()
    }
}

impl Render for Section<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (Surface::section(maud::html! {
                (self.render_copy_block())
                (Cards { cards: &self.content.cards })
                @if let Some(actions) = self.render_trailing_actions() {
                    (actions)
                }
            }).extra_class(self.layout.root_class))
        }
    }
}

impl Render for SectionActionsMarkup<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (SectionActions {
                actions: self.actions,
            })
        }
    }
}

struct Cards<'a> {
    cards: &'a [WorkCardContent],
}

impl Render for Cards<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (CardGrid::new(maud::html! {
                @for card in self.cards {
                    (Card { content: card })
                }
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::partials::components::portfolio::content::{
        portfolio_home_content, work_index_content,
    };

    #[test]
    fn standard_section_keeps_actions_after_card_grid() {
        let content = &work_index_content().supporting_cases_section;
        let markup = Section::standard(content).render().into_string();
        let card_title = content.cards[0].title.to_string();

        assert!(markup.contains("ui-portfolio-work-section"));
        assert!(!markup.contains("ui-portfolio-work-section-rail"));
        assert!(content.actions.is_empty());
        assert!(markup.contains(card_title.as_str()));
        assert!(!markup.contains("ui-button-row"));
    }

    #[test]
    fn current_proof_section_keeps_actions_with_copy_rail() {
        let content = &portfolio_home_content().current_proof_section;
        let markup = Section::current_proof(content).render().into_string();
        let card_title = content.cards[0].title.to_string();
        let action_label = content.actions[0].label.to_string();

        assert!(markup.contains("ui-portfolio-work-section--current-proof"));
        assert!(markup.contains("ui-portfolio-work-section-rail"));
        assert!(markup.find(action_label.as_str()).unwrap() < markup.find(card_title.as_str()).unwrap());
    }
}

struct Card<'a> {
    content: &'a WorkCardContent,
}

impl Render for Card<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (InsetCard::new(maud::html! {
                p class="ui-portfolio-card-kicker" { (&self.content.category) }
                h3 { (&self.content.title) }
                @if let Some(outcome) = &self.content.outcome {
                    p class="ui-portfolio-card-outcome" {
                        span class="ui-portfolio-card-outcome-label" { "Outcome" }
                        span class="ui-portfolio-card-outcome-text" { (outcome) }
                    }
                }
                p class="ui-portfolio-card-summary" { (&self.content.summary) }
                (CardFooter::new(maud::html! {
                    (partials::button::Button::builder()
                        .label(self.content.cta_label.clone())
                        .variant(partials::button::Variant::Secondary)
                        .role(partials::button::Role::link(Text::from(self.content.slug.public_href())))
                        .build())
                }))
            }).extra_class("ui-portfolio-work-card"))
        }
    }
}
