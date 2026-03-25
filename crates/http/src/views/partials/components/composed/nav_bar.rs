use bon::Builder;
use maud::Render;

use super::button;
use crate::types::Text;

crate::views::scoped::inline_css!(
    r#"
me {
  --_nav-shell-padding: 0.75rem 0.85rem;
  --_nav-link-font-size: var(--text-size-meta-md);
  --_nav-brand-wrap-size: 2.6rem;
  --_nav-brand-mark-size: 2.1rem;
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
  inline-size: var(--_nav-brand-wrap-size);
  block-size: var(--_nav-brand-wrap-size);
  flex: none;
  isolation: isolate;
}

me [data-nav-brand-mark-wrap]::before {
  content: "";
  position: absolute;
  inset: auto;
  inline-size: 100%;
  block-size: 100%;
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
  inline-size: var(--_nav-brand-mark-size);
  block-size: var(--_nav-brand-mark-size);
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
  display: inline-flex;
  align-items: center;
  gap: 0.28rem;
  font-size: var(--_nav-link-font-size);
  color: var(--text-muted);
  min-width: 0;
  max-inline-size: 8.75rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

me [data-nav-auth-prefix] {
  flex: none;
  white-space: nowrap;
}

me [data-nav-auth-name] {
  min-width: 0;
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

me [data-nav-guest-auth] {
  min-width: 0;
  display: grid;
  justify-items: end;
}

me [data-nav-guest-auth] .ui-button-row {
  --button-row-gap: 0.35rem;
  --button-row-item-min-inline-size: 7.4rem;
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

@media (max-width: 96rem) and (min-width: 48.001rem) {
  me {
    --_nav-shell-padding: 0.68rem 0.78rem;
    --_nav-link-font-size: var(--text-size-meta-sm);
  }

  me > [data-nav] {
    gap: var(--space-2) var(--space-3);
  }

  me [data-nav-brand-text] {
    font-size: var(--text-size-body-lg);
  }

  me [data-nav-list='primary'] {
    flex-wrap: nowrap;
    gap: 0.15rem;
  }

  me [data-nav-list='primary'] [data-nav-link-label='full'] {
    display: none;
  }

  me [data-nav-list='primary'] [data-nav-link-label='compact'] {
    display: inline;
  }

  me [data-nav-link] {
    padding-inline: calc(var(--control-padding-inline) - 0.62rem);
  }

  me [data-nav-trailing] {
    gap: var(--space-2);
    padding-inline-start: var(--space-2);
  }

  me [data-nav-list='meta'] {
    gap: 0;
  }

  me [data-nav-list='meta'] [data-nav-link] {
    font-size: var(--text-size-label-md);
    padding-inline: calc(var(--control-padding-inline) - 0.7rem);
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-gap: 0.3rem;
    --button-row-item-min-inline-size: 6.8rem;
  }
}

@media (max-width: 48rem) {
  me {
    --_nav-shell-padding: 0.58rem 0.72rem;
    --_nav-brand-wrap-size: 2.25rem;
    --_nav-brand-mark-size: 1.85rem;
    position: static;
    top: auto;
    --_nav-link-font-size: var(--text-size-meta-xs);
    margin-top: var(--space-2);
    margin-bottom: var(--space-2);
  }

  me > [data-nav] {
    grid-template-columns: 1fr;
    padding: var(--_nav-shell-padding);
    border-radius: var(--ui-radius-md-inset);
    gap: var(--space-2);
  }

  me [data-nav-brand] {
    justify-content: flex-start;
  }

  me [data-nav-list='primary'] {
    display: flex;
    flex-wrap: wrap;
    grid-column: auto;
    justify-content: flex-start;
    row-gap: 0.3rem;
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

  me [data-nav-brand-text] {
    display: none;
  }

  me [data-nav-link][aria-current="page"] {
    box-shadow: inset 0 1px 0 var(--surface-edge-default);
  }

  me [data-nav-list='auth'] {
    grid-column: auto;
    gap: 0.25rem;
  }

  me [data-nav-guest-auth] {
    width: 100%;
  }

  me [data-nav-guest-auth] .ui-button-row {
    width: 100%;
    --button-row-grid-template: repeat(2, minmax(0, 1fr));
  }

  me [data-nav-list='meta'] {
    display: none;
  }
}

@media (max-width: 38rem) {
  me [data-nav-list='primary'] li[data-nav-link-item-kind='external'] {
    display: none;
  }

  me [data-nav-list='auth'] {
    align-items: center;
    gap: var(--space-1);
    padding-top: var(--space-1);
    justify-self: start;
    justify-content: flex-end;
    border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
  }

  me [data-nav-auth-text] {
    flex-basis: 100%;
    max-inline-size: none;
    justify-content: flex-end;
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-gap: var(--space-1);
  }
}

@media (max-width: 26rem) {
  me [data-nav-list='primary'] {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    width: 100%;
  }

  me [data-nav-list='primary'] li {
    min-width: 0;
  }

  me [data-nav-list='primary'] [data-nav-link] {
    display: flex;
    justify-content: center;
    width: 100%;
    text-align: center;
  }

  me [data-nav-list='auth'] {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    width: 100%;
    gap: var(--space-1);
  }

  me [data-nav-list='auth'] li,
  me [data-nav-list='auth'] form,
  me [data-nav-list='auth'] [data-nav-link],
  me [data-nav-list='auth'] [data-button] {
    min-width: 0;
    width: 100%;
  }

  me [data-nav-auth-text] {
    grid-column: 1 / -1;
    justify-content: flex-start;
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-grid-template: repeat(2, minmax(0, 1fr));
  }

  me [data-nav-list='auth'] [data-nav-link] {
    padding-inline: calc(var(--control-padding-inline-compact) - 0.1rem);
  }

  me [data-nav-auth-action] {
    --_button-padding-inline: calc(var(--control-padding-inline-compact) - 0.1rem);
  }
}

@media (max-width: 23rem) {
  me [data-nav-auth-prefix] {
    display: none;
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-grid-template: 1fr;
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
        let auth_label = format!("Signed in as {}", self.username);

        maud::html! {
            ul data-nav-list="auth" {
                li {
                    span data-nav-auth-text aria-label=(auth_label) {
                        span data-nav-auth-prefix { "Signed in as" }
                        span data-nav-auth-name title=(&self.username) { (&self.username) }
                    }
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
                    .data_attrs(vec![button::DataAttr::flag("data-nav-auth-action")])
                    .build(),
                button::Button::builder()
                    .label(Text::from("Create account"))
                    .variant(self.create_account_variant.clone())
                    .role(button::Role::link(self.create_account_href.clone()))
                    .data_attrs(vec![button::DataAttr::flag("data-nav-auth-action")])
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

#[derive(Clone, Debug)]
pub enum NavAuth {
    Guest(NavGuestAuth),
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
    fn mobile_nav_wraps_compact_links_without_horizontal_scrolling() {
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
                NavGuestAuth::builder()
                    .sign_in_href(Text::from("/login"))
                    .create_account_href(Text::from("/register"))
                    .build(),
            ))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("white-space: nowrap;"));
        assert!(markup.contains("flex-wrap: wrap;"));
        assert!(!markup.contains("overscroll-behavior-x: contain;"));
        assert!(markup.contains("grid-template-columns: repeat(3, minmax(0, 1fr));"));
        assert!(markup.contains("grid-template-columns: repeat(2, minmax(0, 1fr));"));
        assert!(markup.contains("data-nav-list=\"meta\""));
        assert!(markup.contains("data-nav-link-label=\"compact\""));
        assert!(markup.contains("data-nav-guest-auth"));
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
                NavGuestAuth::builder()
                    .sign_in_href(Text::from("/login"))
                    .create_account_href(Text::from("/register"))
                    .build(),
            ))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-nav-guest-auth"));
        assert!(markup.contains("class=\"button secondary\""));
        assert!(markup.contains("href=\"/register\""));
        assert!(markup.contains(">Create account<"));
    }

    #[test]
    fn signed_in_nav_exposes_split_auth_status() {
        let markup = NavSignedIn::builder()
            .username(Text::from("responsiveaudit"))
            .account_href(Text::from("/protected"))
            .logout_action(Text::from("/logout"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-nav-auth-prefix"));
        assert!(markup.contains("data-nav-auth-name"));
        assert!(markup.contains("aria-label=\"Signed in as responsiveaudit\""));
    }
}
