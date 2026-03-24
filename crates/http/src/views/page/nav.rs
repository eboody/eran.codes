use crate::paths::Route;
use crate::types::Text;
use crate::views::partials;

use super::{NavMode, UserNav};

pub(super) fn signed_in(user: &UserNav) -> partials::components::NavSignedIn {
    partials::components::NavSignedIn::builder()
        .username(user.username.clone())
        .account_href(Text::from(Route::Protected.as_str()))
        .logout_action(Text::from(Route::Logout.as_str()))
        .build()
}

pub(super) fn bar(
    nav_mode: NavMode,
    current_route: Option<Route>,
    user: Option<&UserNav>,
) -> partials::components::NavBar {
    partials::components::NavBar::builder()
        .brand(brand())
        .links(portfolio_links(current_route))
        .auth(auth(nav_mode, current_route, user))
        .build()
}

pub(super) fn portfolio_link_is_active(current_route: Option<Route>, href: &str) -> bool {
    let Some(current_route) = current_route else {
        return false;
    };

    match href {
        path if path == Route::Work.as_str() => matches!(
            current_route,
            Route::Work
                | Route::WorkChatRealtime
                | Route::WorkCommandSse
                | Route::WorkOperationalVisibility
        ),
        path if path == Route::WorkSensitiveSync.as_str() => {
            current_route == Route::WorkSensitiveSync
        }
        path => current_route.as_str() == path,
    }
}

fn brand() -> partials::components::NavBrand {
    partials::components::NavBrand::builder()
        .label(Text::from("eran.codes"))
        .href(Text::from(Route::Home.as_str()))
        .light_logo_src(Text::from("/static/eran.codes-light.svg"))
        .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
        .build()
}

fn portfolio_links(current_route: Option<Route>) -> partials::components::NavLinkList {
    partials::components::NavLinkList::builder()
        .role(partials::components::NavLinkListRole::Primary)
        .children(
            partials::components::portfolio::content::portfolio_nav_links()
                .iter()
                .map(|link| {
                    let href = link.href.to_string();
                    let active =
                        !link.kind.is_external() && portfolio_link_is_active(current_route, &href);

                    partials::components::NavLink::builder()
                        .label(link.label.clone())
                        .maybe_compact_label(compact_label_for_href(&href))
                        .href(link.href.clone())
                        .external(link.kind.is_external())
                        .active(active)
                        .build()
                })
                .collect(),
        )
        .build()
}

fn auth(
    _nav_mode: NavMode,
    current_route: Option<Route>,
    user: Option<&UserNav>,
) -> partials::components::NavAuth {
    match user {
        Some(user) => partials::components::NavAuth::SignedIn(signed_in(user)),
        None => partials::components::NavAuth::Guest(guest_links(current_route)),
    }
}

fn guest_links(current_route: Option<Route>) -> partials::components::NavLinkList {
    partials::components::NavLinkList::builder()
        .role(partials::components::NavLinkListRole::Auth)
        .children(vec![
            partials::components::NavLink::builder()
                .label(Text::from("Sign in"))
                .maybe_compact_label(Some(Text::from("Sign in")))
                .href(Text::from(Route::Login.as_str()))
                .active(current_route == Some(Route::Login))
                .build(),
            partials::components::NavLink::builder()
                .label(Text::from("Create account"))
                .maybe_compact_label(Some(Text::from("Register")))
                .href(Text::from(Route::Register.as_str()))
                .active(current_route == Some(Route::Register))
                .build(),
        ])
        .build()
}

fn compact_label_for_href(href: &str) -> Option<Text> {
    match href {
        path if path == Route::Lab.as_str() => Some(Text::from("Live")),
        path if path == Route::WorkSensitiveSync.as_str() => Some(Text::from("Current")),
        path if path == Route::Work.as_str() => Some(Text::from("Archive")),
        path if path == Route::OpenSource.as_str() => Some(Text::from("Code")),
        path if path == Route::ResumeText.as_str() => Some(Text::from("Resume")),
        "https://github.com/eboody/eran.codes" => Some(Text::from("GitHub")),
        "https://www.linkedin.com/search/results/all/?keywords=Eran%20Boodnero" => {
            Some(Text::from("LinkedIn"))
        }
        "mailto:eboodnero@gmail.com" => Some(Text::from("Contact")),
        _ => None,
    }
}
