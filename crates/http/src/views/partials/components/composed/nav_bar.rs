use bon::Builder;
use maud::Render;

use super::button;
use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  --_nav-shell-padding: 0.75rem 0.85rem;
  --_nav-link-font-size: var(--text-size-meta-md);
  position: sticky;
  top: var(--nav-sticky-offset);
  z-index: 20;
  margin-top: var(--nav-sticky-offset);
  margin-bottom: clamp(0.8rem, 0.55rem + 0.8vw, 1.25rem);
  view-transition-name: app-nav;
}

me > [data-nav] {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  align-items: center;
  position: relative;
  isolation: isolate;
  gap: var(--space-2) var(--space-4);
  padding: var(--_nav-shell-padding);
  border-radius: var(--ui-radius-md);
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
  gap: 0.35rem;
}

me [data-nav-link] {
  margin-bottom: 0;
  padding-block: calc(var(--control-padding-block) - 0.25rem);
  padding-inline: calc(var(--control-padding-inline) - 0.45rem);
  border-radius: calc(var(--control-radius) - 2px);
  border: 1px solid transparent;
  font-size: var(--_nav-link-font-size);
  white-space: nowrap;
  position: relative;
  z-index: 0;
  color: var(--text-muted);
  text-decoration: none;
  transition:
    color var(--motion-fast),
    background-color var(--motion-fast),
    transform var(--motion-fast);
}

me [data-nav-trailing] {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  justify-self: end;
  padding-inline-start: var(--space-3);
  border-inline-start: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
}

me [data-nav-link-label='compact'] {
  display: none;
}

me [data-nav-link][aria-current="page"] {
  color: var(--text-strong);
  border-color: color-mix(in srgb, var(--accent-signal) 30%, var(--border-default));
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 42%, transparent),
      transparent 78%
    ),
    color-mix(in srgb, var(--surface-panel) 94%, var(--accent-signal-soft));
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    0 0 0 1px color-mix(in srgb, var(--accent-signal) 10%, transparent);
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
  gap: var(--control-gap);
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
  outline-offset: var(--interactive-bleed);
  border-radius: calc(var(--control-radius) - 2px);
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
  font-family: var(--ui-font-display);
  font-size: var(--text-size-body-xl);
  font-weight: 600;
  letter-spacing: var(--text-track-tight);
  line-height: var(--text-line-flat);
}

me [data-nav-list='primary'] {
  flex: 1;
  min-width: 0;
  justify-content: center;
  flex-wrap: wrap;
}

me [data-nav-list='meta'] {
  gap: 0.1rem;
}

me [data-nav-list='meta'] [data-nav-link] {
  color: color-mix(in srgb, var(--text-muted) 80%, var(--text-body) 20%);
  font-size: var(--text-size-meta-sm);
}

me [data-nav-list='auth'] {
  min-width: 0;
  justify-content: flex-end;
  flex-wrap: wrap;
  gap: 0.2rem;
}

me [data-nav-list='auth'] li {
  min-width: 0;
}

me [data-nav-auth-text] {
  font-size: var(--_nav-link-font-size);
  color: var(--text-muted);
  max-inline-size: 8.75rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

me [data-nav-list='auth'] form {
  margin: 0;
}

me [data-nav-list='auth'] :where(button, [data-nav-link]) {
  margin-bottom: 0;
}

me [data-nav-list='auth'] [data-nav-link] {
  padding-inline: calc(var(--control-padding-inline) - 0.55rem);
}

me [data-nav-auth-action] {
  --_button-padding-block: var(--control-padding-block-compact);
  --_button-padding-inline: var(--control-padding-inline-compact);
  --_button-font-size: var(--control-font-size-compact);
}

me [data-nav-list='auth'] [data-nav-link-cta='true'] {
  color: var(--ui-text-on-accent);
  border-color: color-mix(in srgb, var(--ui-accent-primary) 56%, var(--border-default));
  background: var(--ui-accent-primary);
  box-shadow:
    inset 0 1px 0 var(--control-edge-accent),
    0 10px 24px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
}

@media (hover: hover) {
  me [data-nav-link]:not([aria-current="page"]):hover {
    color: var(--text-strong);
    background: var(--accent-signal-soft);
    z-index: 1;
  }

  me [data-nav-list='meta'] [data-nav-link]:hover {
    color: var(--text-strong);
  }

  me [data-nav-brand-link]:hover {
    opacity: 0.9;
  }

  me [data-nav-list='auth'] [data-nav-link-cta='true']:hover {
    color: var(--ui-text-on-accent);
    transform: translateY(-1px);
    box-shadow:
      inset 0 1px 0 var(--control-edge-accent-hover),
      0 14px 28px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
  }
}

@media (max-width: 48rem) {
  me {
    --_nav-shell-padding: 0.65rem 0.8rem;
    position: static;
    top: auto;
    --_nav-link-font-size: var(--text-size-meta-xs);
    margin-top: var(--space-2);
    margin-bottom: var(--space-4);
  }

  me > [data-nav] {
    grid-template-columns: minmax(0, 1fr) auto;
    padding: var(--_nav-shell-padding);
    border-radius: var(--ui-radius-md-inset);
    gap: var(--space-2) var(--space-3);
  }

  me [data-nav-list='primary'] {
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: max-content;
    grid-column: 1 / -1;
    justify-content: flex-start;
    overflow-x: auto;
    overscroll-behavior-x: contain;
    padding-bottom: calc(var(--interactive-bleed) * 0.5);
    scrollbar-width: thin;
  }

  me [data-nav-trailing] {
    display: contents;
  }

  me [data-nav-list='primary'] [data-nav-link-label='full'] {
    display: none;
  }

  me [data-nav-list='primary'] [data-nav-link-label='compact'] {
    display: inline;
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
    display: none;
  }

  me [data-nav-link][aria-current="page"] {
    box-shadow: inset 0 1px 0 var(--surface-edge-default);
  }

  me [data-nav-list='auth'] {
    gap: 0.35rem;
  }

  me [data-nav-list='meta'] {
    display: none;
  }
}

@media (max-width: 38rem) {
  me > [data-nav] {
    grid-template-columns: 1fr;
  }

  me [data-nav-list='primary'] li[data-nav-link-item-kind='external'] {
    display: none;
  }

  me [data-nav-list='auth'] {
    align-items: center;
    gap: var(--space-2);
    padding-top: var(--space-2);
    justify-self: stretch;
    justify-content: flex-end;
    border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
  }

  me [data-nav-auth-text] {
    flex-basis: 100%;
    max-inline-size: none;
    text-align: right;
  }
}
"#
);

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
        let kind = if self.external { "external" } else { "internal" };

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
                        span data-nav-link-label="full" { (&self.label) }
                        @if let Some(compact_label) = &self.compact_label {
                            span data-nav-link-label="compact" { (compact_label) }
                        }
                    }
                } @else {
                    @if self.active {
                        a
                            data-nav-link
                            data-nav-link-cta=(self.cta)
                            href=(&self.href)
                            aria-current="page"
                        {
                            span data-nav-link-label="full" { (&self.label) }
                            @if let Some(compact_label) = &self.compact_label {
                                span data-nav-link-label="compact" { (compact_label) }
                            }
                        }
                    } @else {
                        a data-nav-link data-nav-link-cta=(self.cta) href=(&self.href) {
                            span data-nav-link-label="full" { (&self.label) }
                            @if let Some(compact_label) = &self.compact_label {
                                span data-nav-link-label="compact" { (compact_label) }
                            }
                        }
                    }
                }
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
    #[strum(serialize = "auth")]
    Auth,
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
    Guest(NavLinkList),
    SignedIn(NavSignedIn),
}

impl Render for NavAuth {
    fn render(&self) -> maud::Markup {
        match self {
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
    pub meta_links: Option<NavLinkList>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_nav_preserves_scrolling_chip_contract() {
        let markup = NavBar::builder()
            .brand(
                NavBrand::builder()
                    .label(Text::from("eran.codes"))
                    .href(Text::from("/"))
                    .light_logo_src(Text::from("/static/eran.codes-light.svg"))
                    .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
                    .build(),
            )
            .links(
                NavLinkList::builder()
                    .children(vec![
                        NavLink::builder()
                            .label(Text::from("Live Proof"))
                            .maybe_compact_label(Some(Text::from("Live")))
                            .href(Text::from("/lab"))
                            .build(),
                        NavLink::builder()
                            .label(Text::from("Current Proof"))
                            .maybe_compact_label(Some(Text::from("Current")))
                            .href(Text::from("/work/sensitive-sync"))
                            .build(),
                    ])
                    .build(),
            )
            .meta_links(
                NavLinkList::builder()
                    .role(NavLinkListRole::Meta)
                    .children(vec![
                        NavLink::builder()
                            .label(Text::from("GitHub"))
                            .maybe_compact_label(Some(Text::from("GitHub")))
                            .href(Text::from("https://github.com/eboody"))
                            .external(true)
                            .build(),
                    ])
                    .build(),
            )
            .auth(NavAuth::Guest(
                NavLinkList::builder()
                    .role(NavLinkListRole::Auth)
                    .children(vec![NavLink::builder()
                        .label(Text::from("Sign in"))
                        .href(Text::from("/login"))
                        .build()])
                    .build(),
            ))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("white-space: nowrap;"));
        assert!(markup.contains("overscroll-behavior-x: contain;"));
        assert!(markup.contains("data-nav-list=\"meta\""));
        assert!(markup.contains("data-nav-link-label=\"compact\""));
    }

    #[test]
    fn guest_nav_marks_create_account_as_cta() {
        let markup = NavBar::builder()
            .brand(
                NavBrand::builder()
                    .label(Text::from("eran.codes"))
                    .href(Text::from("/"))
                    .light_logo_src(Text::from("/static/eran.codes-light.svg"))
                    .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
                    .build(),
            )
            .links(
                NavLinkList::builder()
                    .children(vec![NavLink::builder()
                        .label(Text::from("Live Proof"))
                        .href(Text::from("/lab"))
                        .build()])
                    .build(),
            )
            .auth(NavAuth::Guest(
                NavLinkList::builder()
                    .role(NavLinkListRole::Auth)
                    .children(vec![
                        NavLink::builder()
                            .label(Text::from("Sign in"))
                            .href(Text::from("/login"))
                            .build(),
                        NavLink::builder()
                            .label(Text::from("Create account"))
                            .href(Text::from("/register"))
                            .cta(true)
                            .build(),
                    ])
                    .build(),
            ))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-nav-link-cta=\"true\""));
        assert!(markup.contains(">Create account<"));
    }
}
