use maud::Markup;

use crate::paths::Route;
use crate::views::partials;

pub(super) fn hero_actions(
    actions: &[partials::components::portfolio::content::CmsActionLink],
) -> Markup {
    maud::html! {
        div data-home-hero-primary-actions {
            (partials::button::Row::builder()
                .density(partials::button::RowDensity::Compact)
                .narrow_layout(partials::button::RowNarrowLayout::Stack)
                .items(actions.iter().map(button_from_cta).collect())
                .build())
        }
    }
}

pub(super) fn signed_in_session_action(label: &crate::types::Text) -> Markup {
    maud::html! {
        div data-home-hero-card-actions {
            (partials::button::Row::builder()
                .density(partials::button::RowDensity::Compact)
                .narrow_layout(partials::button::RowNarrowLayout::Stack)
                .items(vec![internal_primary_button(label.clone(), Route::Protected.as_str())])
                .build())
        }
    }
}

pub(super) fn guest_session_actions(
    actions: &[partials::components::portfolio::content::CmsActionLink],
) -> Markup {
    maud::html! {
        div data-home-hero-card-actions {
            @if let Some(primary_action) = actions.first() {
                div data-home-hero-card-primary-action {
                    (internal_primary_button(
                        primary_action.label.clone(),
                        primary_action.href.clone(),
                    ))
                }
            }
            @if let Some(secondary_action) = actions.get(1) {
                p data-home-hero-card-secondary-copy {
                    "Already have an account? "
                    a
                        href=(secondary_action.href.clone())
                        data-home-hero-card-secondary-link
                    {
                        (&secondary_action.label)
                    }
                }
            }
        }
    }
}

fn button_from_cta(
    action: &partials::components::portfolio::content::CmsActionLink,
) -> partials::button::Button {
    partials::button::Button::builder()
        .label(action.label.clone())
        .variant(match action.tone {
            partials::components::portfolio::content::CtaKind::Primary => {
                partials::button::Variant::Primary
            }
            partials::components::portfolio::content::CtaKind::Secondary => {
                partials::button::Variant::Secondary
            }
        })
        .role(if action.kind.is_external() {
            partials::button::Role::external_link(action.href.clone())
        } else {
            partials::button::Role::link(action.href.clone())
        })
        .build()
}

fn internal_primary_button(
    label: crate::types::Text,
    href: impl Into<crate::types::Text>,
) -> partials::button::Button {
    partials::button::Button::builder()
        .label(label)
        .variant(partials::button::Variant::Primary)
        .role(partials::button::Role::link(href.into()))
        .build()
}
