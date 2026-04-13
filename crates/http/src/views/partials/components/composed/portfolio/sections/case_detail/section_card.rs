use maud::Render;

use super::{CaseSectionRef, InsetCard};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum ListKind {
    #[default]
    List,
    Badges,
}

impl ListKind {
    fn class_name(self) -> &'static str {
        match self {
            Self::List => "ui-portfolio-list",
            Self::Badges => "ui-portfolio-badges ui-portfolio-current-proof-stack",
        }
    }
}

pub(super) struct CaseSectionCard<'a> {
    section: CaseSectionRef<'a>,
    extra_class: Option<&'static str>,
    title_override: Option<&'static str>,
    list_kind: ListKind,
}

impl<'a> CaseSectionCard<'a> {
    pub fn new(section: CaseSectionRef<'a>) -> Self {
        Self {
            section,
            extra_class: None,
            title_override: None,
            list_kind: ListKind::List,
        }
    }

    pub fn extra_class(mut self, class: &'static str) -> Self {
        self.extra_class = Some(class);
        self
    }

    pub fn title_override(mut self, title: &'static str) -> Self {
        self.title_override = Some(title);
        self
    }

    pub fn list_kind(mut self, list_kind: ListKind) -> Self {
        self.list_kind = list_kind;
        self
    }
}

impl Render for CaseSectionCard<'_> {
    fn render(&self) -> maud::Markup {
        let list_class = self.list_kind.class_name();

        let card = InsetCard::new(maud::html! {
            h2 {
                @if let Some(title) = self.title_override {
                    (title)
                } @else {
                    (self.section.title())
                }
            }
            ul class=(list_class) {
                @for item in self.section.items() {
                    li { (item) }
                }
            }
        });
        let card = match self.extra_class {
            Some(extra_class) => card.extra_class(extra_class),
            None => card,
        };

        maud::html! {
            (card)
        }
    }
}
