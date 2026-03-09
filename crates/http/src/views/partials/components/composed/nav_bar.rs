use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct NavLink {
    pub label: Text,
    pub href: Text,
    #[builder(default)]
    pub external: bool,
}

impl Render for NavLink {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li {
                @if self.external {
                    a class="ui-nav-link" href=(&self.href) target="_blank" rel="noopener noreferrer" {
                        (&self.label)
                    }
                } @else {
                    a class="ui-nav-link" href=(&self.href) {
                        (&self.label)
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavLinkList {
    pub class_name: Text,
    pub children: Vec<NavLink>,
}

impl Render for NavLinkList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class=(&self.class_name) {
                @for item in &self.children {
                    (item)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavSignedIn {
    pub username: Text,
    pub account_href: Text,
    pub logout_action: Text,
}

impl Render for NavSignedIn {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class="ui-nav-list ui-nav-auth" {
                li {
                    span class="ui-nav-auth-text" { "Signed in as " (&self.username) }
                }
                li {
                    a class="ui-nav-link" href=(&self.account_href) { "Account" }
                }
                li {
                    form method="post" action=(&self.logout_action) {
                        button type="submit" class="secondary ui-nav-auth-action" {
                            "Sign out"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum NavAuth {
    Hidden,
    Guest(NavLinkList),
    SignedIn(NavSignedIn),
}

impl Render for NavAuth {
    fn render(&self) -> maud::Markup {
        match self {
            Self::Hidden => maud::html! {},
            Self::Guest(links) => links.render(),
            Self::SignedIn(signed_in) => signed_in.render(),
        }
    }
}

// ci: style-system-component
// ci: render-composition-component
#[derive(Clone, Debug, Builder)]
pub struct NavBar {
    pub brand: NavLinkList,
    pub links: NavLinkList,
    pub auth: NavAuth,
}

impl Render for NavBar {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="container ui-nav-shell" {
                nav class="ui-nav" {
                    (&self.brand)
                    (&self.links)
                    (&self.auth)
                }
            }
        }
    }
}
