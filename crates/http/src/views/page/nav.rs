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
    let (primary_links, meta_links) = match nav_mode {
        NavMode::Auth => auth_links(),
        NavMode::App | NavMode::Portfolio => portfolio_links(current_route),
    };

    partials::components::NavBar::builder()
        .brand(brand())
        .links(primary_links)
        .maybe_meta_links(meta_links)
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

fn portfolio_links(
    current_route: Option<Route>,
) -> (
    partials::components::NavLinkList,
    Option<partials::components::NavLinkList>,
) {
    let (primary_children, meta_children) =
        partials::components::portfolio::content::portfolio_nav_links()
            .iter()
            .fold((Vec::new(), Vec::new()), |mut grouped_links, link| {
                let href = link.href.to_string();
                let active =
                    !link.kind.is_external() && portfolio_link_is_active(current_route, &href);

                let nav_link = partials::components::NavLink::builder()
                    .label(link.label.clone())
                    .maybe_compact_label(compact_label_for_href(&href))
                    .href(link.href.clone())
                    .external(link.kind.is_external())
                    .active(active)
                    .build();

                if is_primary_portfolio_nav_href(&href) {
                    grouped_links.0.push(nav_link);
                } else {
                    grouped_links.1.push(nav_link);
                }

                grouped_links
            });

    let primary_links = partials::components::NavLinkList::builder()
        .role(partials::components::NavLinkListRole::Primary)
        .children(primary_children)
        .build();
    let meta_links = (!meta_children.is_empty()).then(|| {
        partials::components::NavLinkList::builder()
            .role(partials::components::NavLinkListRole::Meta)
            .children(meta_children)
            .build()
    });

    (primary_links, meta_links)
}

fn auth_links() -> (
    partials::components::NavLinkList,
    Option<partials::components::NavLinkList>,
) {
    (
        partials::components::NavLinkList::builder()
            .role(partials::components::NavLinkListRole::Primary)
            .children(vec![])
            .build(),
        None,
    )
}

fn is_primary_portfolio_nav_href(href: &str) -> bool {
    href == Route::Home.as_str()
        || href == Route::OpenSource.as_str()
        || href == Route::Lab.as_str()
}

fn auth(
    nav_mode: NavMode,
    current_route: Option<Route>,
    user: Option<&UserNav>,
) -> partials::components::NavAuth {
    match user {
        Some(user) => partials::components::NavAuth::SignedIn(signed_in(user)),
        None => match nav_mode {
            NavMode::Auth => partials::components::NavAuth::Switch(auth_switch(current_route)),
            NavMode::App | NavMode::Portfolio => {
                partials::components::NavAuth::Guest(guest_auth(current_route))
            }
        },
    }
}

fn guest_auth(current_route: Option<Route>) -> partials::components::NavGuestAuth {
    let sign_in_variant = if current_route == Some(Route::Login) {
        partials::button::Variant::Primary
    } else {
        partials::button::Variant::Secondary
    };
    let create_account_variant = if current_route == Some(Route::Login) {
        partials::button::Variant::Secondary
    } else {
        partials::button::Variant::Primary
    };

    partials::components::NavGuestAuth::builder()
        .sign_in_href(Text::from(Route::Login.as_str()))
        .create_account_href(Text::from(Route::Register.as_str()))
        .sign_in_variant(sign_in_variant)
        .create_account_variant(create_account_variant)
        .build()
}

fn auth_switch(current_route: Option<Route>) -> partials::components::NavGuestSwitch {
    let (label, href) = match current_route {
        Some(Route::Login) => ("Create account", Route::Register.as_str()),
        Some(Route::Register) => ("Sign in", Route::Login.as_str()),
        _ => ("Sign in", Route::Login.as_str()),
    };

    partials::components::NavGuestSwitch::builder()
        .label(Text::from(label))
        .href(Text::from(href))
        .build()
}

fn compact_label_for_href(href: &str) -> Option<Text> {
    match href {
        path if path == Route::Home.as_str() => Some(Text::from("Flagship")),
        path if path == Route::OpenSource.as_str() => Some(Text::from("Crates")),
        path if path == Route::Lab.as_str() => Some(Text::from("Lab")),
        path if path == Route::ResumeText.as_str() => Some(Text::from("Resume")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn portfolio_links_keep_primary_review_path_compact() {
        let (primary_links, meta_links) = portfolio_links(Some(Route::Home));
        let primary_labels: Vec<_> = primary_links
            .children
            .iter()
            .map(|link| link.label.to_string())
            .collect();
        let meta_labels: Vec<_> = meta_links
            .expect("meta links")
            .children
            .iter()
            .map(|link| link.label.to_string())
            .collect();

        assert_eq!(
            primary_labels,
            vec!["Flagship", "Crates", "Lab"]
        );
        assert_eq!(
            meta_labels,
            vec!["Resume"]
        );
    }

    #[test]
    fn login_route_promotes_sign_in_guest_action() {
        let markup = guest_auth(Some(Route::Login)).render().into_string();

        assert!(markup.contains(">Sign in<"));
        assert!(markup.contains("class=\"button\""));
        assert!(markup.contains("class=\"button secondary\""));
        assert!(markup.contains("href=\"/login\""));
        assert!(markup.contains("href=\"/register\""));
    }

    #[test]
    fn auth_mode_uses_single_contextual_switch_action() {
        let login_markup = auth(NavMode::Auth, Some(Route::Login), None)
            .render()
            .into_string();
        let register_markup = auth(NavMode::Auth, Some(Route::Register), None)
            .render()
            .into_string();

        assert!(login_markup.contains("data-nav-auth-switch"));
        assert!(login_markup.contains("href=\"/register\""));
        assert!(!login_markup.contains("data-nav-auth-action"));
        assert!(register_markup.contains("data-nav-auth-switch"));
        assert!(register_markup.contains("href=\"/login\""));
        assert!(!register_markup.contains("data-nav-auth-action"));
    }
}
