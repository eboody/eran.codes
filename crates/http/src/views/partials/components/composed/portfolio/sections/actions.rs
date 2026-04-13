use maud::Render;

use crate::views::partials;

use super::super::content::{CmsActionLink, CtaKind};

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
