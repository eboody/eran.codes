use bon::Builder;
use maud::Render;

use super::button;
use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  --_nav-shell-padding-block: 0.8rem;
  --_nav-shell-padding-inline: 1rem;
  --_nav-shell-radius: var(--ui-radius-md);
  --_nav-shell-radius-compact: calc(var(--ui-radius-md) - 2px);
  --_nav-link-padding-block: calc(var(--control-padding-block) - 0.25rem);
  --_nav-link-padding-inline: calc(var(--control-padding-inline) - 0.3rem);
  --_nav-link-radius: calc(var(--control-radius) - 2px);
  --_nav-link-font-size: 0.84rem;
  --_nav-action-padding-block: var(--control-padding-block-compact);
  --_nav-action-padding-inline: var(--control-padding-inline-compact);
  --_nav-action-font-size: var(--control-font-size-compact);
  --_nav-brand-gap: var(--control-gap);
  --_nav-brand-focus-radius: calc(var(--control-radius) - 2px);
  --_nav-brand-font-family: var(--ui-font-display);

  position: sticky;
  top: var(--nav-sticky-offset);
  z-index: 20;
  margin-top: var(--nav-sticky-offset);
  view-transition-name: app-nav;
}

me > [data-nav] {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  align-items: center;
  position: relative;
  isolation: isolate;
  gap: var(--space-2) var(--space-4);
  padding-block: var(--_nav-shell-padding-block);
  padding-inline: var(--_nav-shell-padding-inline);
  border-radius: var(--_nav-shell-radius);
  border: 1px solid var(--border-default);
  background: var(--surface-fill-panel);
  box-shadow: var(--shadow-panel);
  overflow: visible;
}

me [data-nav-list] {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

me [data-nav-link] {
  margin-bottom: 0;
  padding-block: var(--_nav-link-padding-block);
  padding-inline: var(--_nav-link-padding-inline);
  border-radius: var(--_nav-link-radius);
  font-size: var(--_nav-link-font-size);
  position: relative;
  z-index: 0;
  color: var(--text-muted);
  text-decoration: none;
  transition:
    color var(--motion-fast),
    background-color var(--motion-fast),
    transform var(--motion-fast);
}

me [data-nav-link]:focus-visible {
  outline: none;
  color: var(--text-strong);
  background: var(--accent-signal-soft);
  z-index: 1;
}

me [data-nav-brand] {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  min-width: 0;
}

me [data-nav-brand-link] {
  display: inline-flex;
  align-items: center;
  gap: var(--_nav-brand-gap);
  position: relative;
  z-index: 0;
  color: var(--text-strong);
  text-decoration: none;
  transition:
    opacity var(--motion-fast),
    transform var(--motion-fast);
}

me [data-nav-brand-link]:focus-visible {
  outline: 2px solid color-mix(in srgb, var(--accent-signal) 64%, white);
  outline-offset: 0.35rem;
  border-radius: var(--_nav-brand-focus-radius);
  z-index: 1;
}

me [data-nav-brand-picture] {
  display: flex;
  position: relative;
  z-index: 1;
}

me [data-nav-brand-mark-wrap] {
  --_logo-glow-red: rgb(218 89 85 / 0.88);
  --_logo-glow-blue: rgb(38 125 255 / 0.88);

  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  isolation: isolate;
}

me [data-nav-brand-mark-wrap]::before {
  content: "";
  position: absolute;
  inset: auto;
  inline-size: 2.6rem;
  block-size: 2.6rem;
  top: 50%;
  left: 50%;
  z-index: 0;
  border-radius: 0.85rem;
  background-image: linear-gradient(
    -45deg,
    var(--_logo-glow-red) 50%,
    var(--_logo-glow-blue) 50%
  );
  filter: blur(0.7rem);
  opacity: 0.1;
  transform: translate(-50%, -50%);
}

me [data-nav-brand-mark] {
  display: block;
  inline-size: 2.1rem;
  block-size: 2.1rem;
  flex: none;
  filter: drop-shadow(0.18rem 0.24rem 0.8rem color-mix(in srgb, black 18%, transparent));
}

me [data-nav-brand-text] {
  font-family: var(--_nav-brand-font-family);
  font-size: 1.08rem;
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1;
}

me [data-nav-list='primary'] {
  flex: 1;
  min-width: 0;
  justify-content: center;
  flex-wrap: wrap;
}

me [data-nav-list='auth'] {
  min-width: 0;
  justify-content: flex-end;
  flex-wrap: wrap;
  justify-self: end;
}

me [data-nav-list='auth'] li {
  min-width: 0;
}

me [data-nav-auth-text] {
  font-size: 0.84rem;
  color: var(--text-muted);
}

me [data-nav-list='auth'] form {
  margin: 0;
}

me [data-nav-list='auth'] :where(button, [data-nav-link]) {
  margin-bottom: 0;
}

me [data-nav-auth-action] {
  --_button-padding-block: var(--_nav-action-padding-block);
  --_button-padding-inline: var(--_nav-action-padding-inline);
  --_button-font-size: var(--_nav-action-font-size);
}

@media (hover: hover) {
  me [data-nav-link]:hover {
    color: var(--text-strong);
    background: var(--accent-signal-soft);
    z-index: 1;
  }

  me [data-nav-brand-link]:hover {
    opacity: 0.9;
  }
}

@media (max-width: 48rem) {
  me {
    top: 0.35rem;
    --_nav-shell-padding-block: 0.65rem;
    --_nav-shell-padding-inline: 0.8rem;
    --_nav-link-font-size: 0.78rem;
  }

  me > [data-nav] {
    grid-template-columns: minmax(0, 1fr) auto;
    border-radius: var(--_nav-shell-radius-compact);
    gap: 0.45rem 0.75rem;
  }

  me [data-nav-list='primary'] {
    grid-column: 1 / -1;
    justify-content: flex-start;
    overflow-x: auto;
    flex-wrap: nowrap;
    padding-bottom: 0.15rem;
    scrollbar-width: thin;
  }

  me [data-nav-auth-text] {
    max-inline-size: min(42vw, 12rem);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  me [data-nav-brand-mark] {
    inline-size: 1.85rem;
    block-size: 1.85rem;
  }

  me [data-nav-brand-text] {
    font-size: 0.96rem;
  }
}

@media (max-width: 38rem) {
  me > [data-nav] {
    grid-template-columns: 1fr;
  }

  me [data-nav-list='auth'] {
    justify-self: stretch;
    justify-content: flex-start;
  }

  me [data-nav-auth-text] {
    flex-basis: 100%;
    max-inline-size: none;
  }
}
"#
);

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
                    a data-nav-link href=(&self.href) target="_blank" rel="noopener noreferrer" {
                        (&self.label)
                    }
                } @else {
                    a data-nav-link href=(&self.href) {
                        (&self.label)
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum NavLinkListRole {
    #[default]
    Primary,
    Auth,
}

impl NavLinkListRole {
    fn as_attr(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Auth => "auth",
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavLinkList {
    #[builder(default)]
    pub role: NavLinkListRole,
    pub children: Vec<NavLink>,
}

impl Render for NavLinkList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul data-nav-list=(self.role.as_attr()) {
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
        maud::html! {
            ul data-nav-list="auth" {
                li {
                    span data-nav-auth-text { "Signed in as " (&self.username) }
                }
                li {
                    a data-nav-link href=(&self.account_href) { "Account" }
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
    pub brand: NavBrand,
    pub links: NavLinkList,
    pub auth: NavAuth,
}

impl Render for NavBar {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="u-container" data-nav-shell {
                (css())
                nav data-nav {
                    (&self.brand)
                    (&self.links)
                    (&self.auth)
                }
            }
        }
    }
}
