mod styles;
#[cfg(test)]
mod tests;

use bon::Builder;
use maud::{Markup, Render};

use super::button;
use crate::types::Text;


#[derive(Clone, Debug, Builder)]
pub struct NavLink {
    pub label: Text,
    pub compact_label: Option<Text>,
    pub href: Text,
    #[builder(default)]
    pub external: bool,
    #[builder(default)]
    pub cta: bool,
    #[builder(default)]
    pub active: bool,
}

impl Render for NavLink {
    fn render(&self) -> maud::Markup {
        let kind = self.item_kind();

        maud::html! {
            li data-nav-link-item-kind=(kind) {
                @if self.external {
                    a
                        data-nav-link
                        data-nav-link-cta=(self.cta)
                        href=(&self.href)
                        target="_blank"
                        rel="noopener noreferrer"
                    {
                        (self.render_labels())
                    }
                } @else {
                    @if self.active {
                        a
                            data-nav-link
                            data-nav-link-cta=(self.cta)
                            href=(&self.href)
                            aria-current="page"
                        {
                            (self.render_labels())
                        }
                    } @else {
                        a data-nav-link data-nav-link-cta=(self.cta) href=(&self.href) {
                            (self.render_labels())
                        }
                    }
                }
            }
        }
    }
}

impl NavLink {
    fn item_kind(&self) -> &'static str {
        if self.external { "external" } else { "internal" }
    }

    fn render_labels(&self) -> Markup {
        maud::html! {
            span data-nav-link-label="full" { (&self.label) }
            @if let Some(compact_label) = &self.compact_label {
                span data-nav-link-label="compact" { (compact_label) }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, strum_macros::AsRefStr)]
pub enum NavLinkListRole {
    #[default]
    #[strum(serialize = "primary")]
    Primary,
    #[strum(serialize = "meta")]
    Meta,
}

#[derive(Clone, Debug, Builder)]
pub struct NavLinkList {
    #[builder(default)]
    pub role: NavLinkListRole,
    pub children: Vec<NavLink>,
}

impl NavLinkList {
    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl Render for NavLinkList {
    fn render(&self) -> maud::Markup {
        if self.is_empty() {
            return maud::html! {};
        }

        maud::html! {
            ul data-nav-list=(self.role.as_ref()) {
                @for item in &self.children {
                    (item)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavBrand {
    pub label: Text,
    pub href: Text,
    pub light_logo_src: Text,
    pub dark_logo_src: Text,
}

impl Render for NavBrand {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-nav-brand {
                a data-nav-brand-link href=(&self.href) {
                    span data-nav-brand-mark-wrap {
                        picture data-nav-brand-picture {
                            source
                                media="(prefers-color-scheme: dark)"
                                srcset=(&self.dark_logo_src);
                            img
                                data-nav-brand-mark
                                src=(&self.light_logo_src)
                                width="40"
                                height="40"
                                alt=""
                                aria-hidden="true";
                        }
                    }
                    span data-nav-brand-text { (&self.label) }
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
        let auth_label = format!("Signed in as {}", self.username);

        maud::html! {
            ul data-nav-list="auth" {
                li {
                    span data-nav-auth-text aria-label=(auth_label) {
                        span data-nav-auth-prefix { "Signed in as" }
                        span data-nav-auth-name title=(&self.username) { (&self.username) }
                    }
                }
                li data-nav-account-item {
                    a data-nav-link data-nav-account-link href=(&self.account_href) { "Account" }
                }
                li {
                    form method="post" action=(&self.logout_action) {
                        (button::Button::builder()
                            .label(Text::from("Sign out"))
                            .variant(button::Variant::Secondary)
                            .role(button::Role::submit())
                            .data_attrs(vec![button::DataAttr::flag("data-nav-auth-action")])
                            .build())
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavGuestAuth {
    pub sign_in_href: Text,
    pub create_account_href: Text,
    #[builder(default = button::Variant::Secondary)]
    pub sign_in_variant: button::Variant,
    #[builder(default)]
    pub create_account_variant: button::Variant,
}

impl Render for NavGuestAuth {
    fn render(&self) -> maud::Markup {
        let actions = button::Row::builder()
            .density(button::RowDensity::Compact)
            .frame(button::RowFrame::Contained)
            .items(vec![
                button::Button::builder()
                    .label(Text::from("Sign in"))
                    .variant(self.sign_in_variant.clone())
                    .role(button::Role::link(self.sign_in_href.clone()))
                    .data_attrs(vec![
                        button::DataAttr::flag("data-nav-auth-action"),
                        button::DataAttr::flag("data-nav-sign-in-action"),
                    ])
                    .build(),
                button::Button::builder()
                    .label(Text::from("Create account"))
                    .variant(self.create_account_variant.clone())
                    .role(button::Role::link(self.create_account_href.clone()))
                    .data_attrs(vec![
                        button::DataAttr::flag("data-nav-auth-action"),
                        button::DataAttr::flag("data-nav-create-account-action"),
                    ])
                    .build(),
            ])
            .build();

        maud::html! {
            div data-nav-guest-auth {
                (actions)
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavGuestSwitch {
    pub label: Text,
    pub href: Text,
}

impl Render for NavGuestSwitch {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div data-nav-guest-auth data-nav-guest-auth-variant="switch" {
                a data-nav-link data-nav-auth-switch href=(&self.href) { (&self.label) }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum NavAuth {
    Guest(NavGuestAuth),
    Switch(NavGuestSwitch),
    SignedIn(NavSignedIn),
}

impl Render for NavAuth {
    fn render(&self) -> maud::Markup {
        match self {
            Self::Guest(links) => links.render(),
            Self::Switch(link) => link.render(),
            Self::SignedIn(signed_in) => signed_in.render(),
        }
    }
}

// ci: style-system-component
// ci: render-composition-component
#[derive(Clone, Debug, Builder)]
pub struct NavBar {
    pub brand: NavBrand,
    pub links: NavLinkList,
    pub meta_links: Option<NavLinkList>,
    pub auth: NavAuth,
}

impl Render for NavBar {
    fn render(&self) -> maud::Markup {
        let nav_layout = if self.links.is_empty() && self.meta_links.is_none() {
            "split"
        } else {
            "default"
        };

        maud::html! {
            header class="u-container" data-nav-shell {
                (styles::render())
                nav data-nav data-nav-layout=(nav_layout) {
                    (&self.brand)
                    (&self.links)
                    div data-nav-trailing {
                        @if let Some(meta_links) = &self.meta_links {
                            (meta_links)
                        }
                        (&self.auth)
                    }
                }
            }
        }
    }
}
