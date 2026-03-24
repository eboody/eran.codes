use bon::Builder;
use maud::{Markup, Render};

use crate::paths::Route;
use crate::types::{SseTabId, Text};
use crate::views::partials;

const APP_CSS_ASSET_URL: &str = "/static/app.css?v=20260320-tabs";
const LOCAL_TABS_ASSET_URL: &str = "/static/local-tabs.js?v=20260320-tabs";

crate::views::scoped::inline_css!(
    r#"
me > [data-page-section] > :where(header, section, article, div) {
  transition:
    opacity var(--motion-standard),
    transform var(--motion-standard),
    border-color var(--motion-standard),
    box-shadow var(--motion-standard);
}

@starting-style {
  me > [data-page-section] > :where(header, section, article, div) {
    opacity: 0;
    transform: translateY(0.8rem);
  }
}

@media (prefers-reduced-motion: reduce) {
  me > [data-page-section] > :where(header, section, article, div) {
    transition: none;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct UserNav {
    pub username: Text,
    pub email: Text,
}

impl Render for UserNav {
    fn render(&self) -> Markup {
        partials::components::NavSignedIn::builder()
            .username(self.username.clone())
            .account_href(Text::from(Route::Protected.as_str()))
            .logout_action(Text::from(Route::Logout.as_str()))
            .build()
            .render()
    }
}

#[derive(Debug, Builder)]
pub struct Frame {
    pub content: Markup,
}

impl Render for Frame {
    fn render(&self) -> Markup {
        maud::html! {
            main class="u-container" data-page-frame {
                (css())
                (&self.content)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SseMode {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum NavMode {
    #[default]
    App,
    Portfolio,
}

#[derive(Builder)]
pub struct Layout<'a> {
    pub title: &'a str,
    pub content: Markup,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
    #[builder(default)]
    pub sse_mode: SseMode,
    #[builder(default)]
    pub nav_mode: NavMode,
    pub current_route: Option<Route>,
    pub sse_tab_id: Option<SseTabId>,
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        let sse_tab_id = self
            .sse_tab_id
            .clone()
            .unwrap_or_else(|| crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string()));
        let global_signals = match self.sse_mode {
            SseMode::Enabled => format!(
                "{{sseTabId: '{sse_tab_id}', sseConnected: false, transportErrorSource: '', transportErrorKind: '', transportErrorTitle: '', transportErrorMessage: '', transportErrorStatus: 0, transportRetrying: false}}"
            ),
            SseMode::Disabled => {
                "{transportErrorSource: '', transportErrorKind: '', transportErrorTitle: '', transportErrorMessage: '', transportErrorStatus: 0, transportRetrying: false}".to_string()
            }
        };
        let brand = partials::components::NavBrand::builder()
            .label(Text::from("eran.codes"))
            .href(Text::from(Route::Home.as_str()))
            .light_logo_src(Text::from("/static/eran.codes-light.svg"))
            .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
            .build();
        let current_route = self.current_route;
        let portfolio_links = partials::components::NavLinkList::builder()
            .role(partials::components::NavLinkListRole::Primary)
            .children(portfolio_nav_children(current_route))
            .build();
        let auth = match self.nav_mode {
            NavMode::Portfolio => partials::components::NavAuth::Hidden,
            NavMode::App => match &self.user {
                Some(user) => partials::components::NavAuth::SignedIn(
                    partials::components::NavSignedIn::builder()
                        .username(user.username.clone())
                        .account_href(Text::from(Route::Protected.as_str()))
                        .logout_action(Text::from(Route::Logout.as_str()))
                        .build(),
                ),
                None => partials::components::NavAuth::Guest(
                    partials::components::NavLinkList::builder()
                        .role(partials::components::NavLinkListRole::Auth)
                        .children(vec![
                            partials::components::NavLink::builder()
                                .label(Text::from("Sign in"))
                                .href(Text::from(Route::Login.as_str()))
                                .active(current_route == Some(Route::Login))
                                .build(),
                            partials::components::NavLink::builder()
                                .label(Text::from("Create account"))
                                .href(Text::from(Route::Register.as_str()))
                                .active(current_route == Some(Route::Register))
                                .build(),
                        ])
                        .build(),
                ),
            },
        };
        let nav_bar = partials::components::NavBar::builder()
            .brand(brand)
            .links(portfolio_links)
            .auth(auth)
            .build();
        let body_content = maud::html! {
            (nav_bar)
            (crate::views::partials::Error)
            (self.content.clone())
        };
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { (self.title) }
                    link
                        rel="icon"
                        type="image/svg+xml"
                        media="(prefers-color-scheme: light)"
                        href="/static/eran.codes-light.svg";
                    link
                        rel="icon"
                        type="image/svg+xml"
                        media="(prefers-color-scheme: dark)"
                        href="/static/eran.codes-dark.svg";
                    link
                        rel="icon"
                        type="image/png"
                        sizes="1024x1024"
                        href="/static/eran.codes-favicon.png";
                    link rel="apple-touch-icon" sizes="1024x1024" href="/static/eran.codes.png";
                    link rel="preconnect" href="https://fonts.googleapis.com";
                    link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                    link
                        rel="stylesheet"
                        href="https://fonts.googleapis.com/css2?family=Newsreader:opsz,wght@6..72,500;6..72,600;6..72,700&family=Space+Grotesk:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500;600&display=swap";
                    link rel="stylesheet" href="/static/open-props.min.css";
                    link rel="stylesheet" href=(APP_CSS_ASSET_URL);
                    link
                        rel="stylesheet"
                        href="https://cdn.jsdelivr.net/gh/iconoir-icons/iconoir@main/css/iconoir.css";
                    (crate::views::partials::components::head_styles())
                    script src="/static/css-scope-inline.js" {}
                    script type="module" src="/static/datastar.js" {}
                    script src=(LOCAL_TABS_ASSET_URL) {}
                    script type="module" src="/static/transport-errors.js" {}
                    script src="/static/surreal.js" {}
                }
                @match self.sse_mode {
                    SseMode::Enabled => {
                        body data-signals=(global_signals) data-init=(events_init_action()) { (body_content) }
                    }
                    SseMode::Disabled => {
                        body data-signals=(global_signals) { (body_content) }
                    }
                }
            }
        }
    }
}

fn portfolio_nav_children(current_route: Option<Route>) -> Vec<partials::components::NavLink> {
    partials::components::portfolio::content::portfolio_nav_links()
        .iter()
        .map(|link| {
            let href = link.href.to_string();
            let active = !link.kind.is_external() && portfolio_link_is_active(current_route, &href);

            partials::components::NavLink::builder()
                .label(link.label.clone())
                .href(link.href.clone())
                .external(link.kind.is_external())
                .active(active)
                .build()
        })
        .collect()
}

fn portfolio_link_is_active(current_route: Option<Route>, href: &str) -> bool {
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

fn events_init_action() -> String {
    format!(
        "@get('{}', {{filterSignals: {{include: /^(sseTabId|operations_filter_query)$/}}}})",
        Route::Events
    )
}

#[derive(Debug, Builder)]
pub struct Error {
    pub title: &'static str,
    pub message: &'static str,
    pub status: u16,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
}

impl Render for Error {
    fn render(&self) -> Markup {
        let content = Frame::builder()
            .content(maud::html! {
                div data-page-section {
                    article {
                        header {
                            h1 { (self.title) }
                        }
                        p { (self.message) }
                        p { "Status: " (self.status) }
                    }
                }
            })
            .build()
            .render();

        Layout::builder()
            .title(self.title)
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_init_action_limits_signals_to_transport_contract() {
        assert_eq!(
            events_init_action(),
            "@get('/events', {filterSignals: {include: /^(sseTabId|operations_filter_query)$/}})"
        );
    }

    #[test]
    fn layout_includes_local_tabs_controller() {
        let markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .build()
            .render()
            .into_string();

        assert!(markup.contains("/static/local-tabs.js"));
    }

    #[test]
    fn layout_uses_proof_pivot_nav_labels() {
        let markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .current_route(Route::Home)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("Current Proof"));
        assert!(markup.contains("Supporting Proof"));
        assert!(markup.contains("Live Proof"));
        assert!(markup.contains("/resume.txt"));
        assert!(!markup.contains(">Work<"));
        assert!(!markup.contains("Live Lab"));
    }

    #[test]
    fn current_proof_and_supporting_proof_nav_entries_activate_separately() {
        let current_proof_markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .current_route(Route::WorkSensitiveSync)
            .build()
            .render()
            .into_string();
        let supporting_markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .current_route(Route::WorkChatRealtime)
            .build()
            .render()
            .into_string();

        assert!(current_proof_markup.contains("Current Proof"));
        assert!(supporting_markup.contains("Supporting Proof"));
        assert!(portfolio_link_is_active(
            Some(Route::WorkSensitiveSync),
            Route::WorkSensitiveSync.as_str()
        ));
        assert!(portfolio_link_is_active(
            Some(Route::WorkChatRealtime),
            Route::Work.as_str()
        ));
        assert!(!portfolio_link_is_active(
            Some(Route::WorkChatRealtime),
            Route::WorkSensitiveSync.as_str()
        ));
    }

    #[test]
    fn frame_renders_page_container() {
        let markup = Frame::builder()
            .content(maud::html! {
                div data-page-section {
                    section { "example" }
                }
            })
            .build()
            .render()
            .into_string();

        assert!(markup.contains("data-page-frame"));
        assert!(markup.contains("data-page-section"));
    }
}
