mod assets;
mod nav;

use bon::Builder;
use maud::{Markup, Render};

use crate::paths::Route;
use crate::types::{SseTabId, Text};

#[derive(Clone, Debug, Builder)]
pub struct UserNav {
    pub username: Text,
    pub email: Text,
}

impl From<&crate::auth::User> for UserNav {
    fn from(user: &crate::auth::User) -> Self {
        Self::builder()
            .username(Text::from(user.username.to_string()))
            .email(Text::from(user.email.to_string()))
            .build()
    }
}

impl Render for UserNav {
    fn render(&self) -> Markup {
        nav::signed_in(self).render()
    }
}

#[derive(Debug, Builder)]
pub struct Frame {
    pub content: Markup,
    #[builder(default)]
    pub width: FrameWidth,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum FrameWidth {
    #[default]
    Standard,
    Wide,
}

impl FrameWidth {
    fn class_name(self) -> &'static str {
        match self {
            Self::Standard => "u-container",
            Self::Wide => "u-container u-container--wide",
        }
    }
}

impl Render for Frame {
    fn render(&self) -> Markup {
        maud::html! {
            main class=(self.width.class_name()) data-page-frame {
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
    Auth,
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
        let global_signals = global_signals(self.sse_mode, self.sse_tab_id.clone());
        let nav_bar = nav::bar(self.nav_mode, self.current_route, self.user.as_ref());
        let body_content = maud::html! {
            (nav_bar)
            (crate::views::partials::transport_error::Error)
            (self.content.clone())
        };
        maud::html! {
            (maud::DOCTYPE)
            html {
                (assets::head(self.title))
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

fn global_signals(sse_mode: SseMode, sse_tab_id: Option<SseTabId>) -> String {
    match sse_mode {
        SseMode::Enabled => {
            let sse_tab_id = sse_tab_id
                .unwrap_or_else(|| crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string()));
            format!(
                "{{sseTabId: '{sse_tab_id}', sseConnected: false, transportErrorSource: '', transportErrorKind: '', transportErrorTitle: '', transportErrorMessage: '', transportErrorStatus: 0, transportRetrying: false}}"
            )
        }
        SseMode::Disabled => {
            "{transportErrorSource: '', transportErrorKind: '', transportErrorTitle: '', transportErrorMessage: '', transportErrorStatus: 0, transportRetrying: false}".to_string()
        }
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
    use super::nav::portfolio_link_is_active;

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
        assert!(markup.contains("Archive"));
        assert!(markup.contains("Live Proof"));
        assert!(markup.contains("/resume.txt"));
        assert!(!markup.contains("GitHub"));
        assert!(!markup.contains("LinkedIn"));
        assert!(!markup.contains("Contact"));
        assert!(!markup.contains(">Work<"));
        assert!(!markup.contains("Live Lab"));
    }

    #[test]
    fn portfolio_nav_mode_keeps_auth_slot_visible() {
        let markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .nav_mode(NavMode::Portfolio)
            .current_route(Route::Home)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("Sign in"));
        assert!(markup.contains("Create account"));
    }

    #[test]
    fn auth_nav_mode_hides_portfolio_links() {
        let markup = Layout::builder()
            .title("Example")
            .content(maud::html! { main {} })
            .nav_mode(NavMode::Auth)
            .current_route(Route::Login)
            .build()
            .render()
            .into_string();

        assert!(!markup.contains("Live Proof"));
        assert!(!markup.contains("Current Proof"));
        assert!(!markup.contains("/resume.txt"));
        assert!(markup.contains("data-nav-layout=\"split\""));
        assert!(markup.contains("data-nav-auth-switch"));
        assert!(markup.contains("href=\"/register\""));
        assert!(!markup.contains("<a class=\"button\" data-button href=\"/login\" data-nav-auth-action>Sign in</a>"));
        assert!(!markup.contains("<a class=\"button\" data-button href=\"/register\" data-nav-auth-action>Create account</a>"));
    }

    #[test]
    fn current_proof_and_archive_nav_entries_activate_separately() {
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
        assert!(supporting_markup.contains("Archive"));
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

    #[test]
    fn wide_frame_renders_wide_container_class() {
        let markup = Frame::builder()
            .content(maud::html! { div {} })
            .width(FrameWidth::Wide)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("u-container u-container--wide"));
    }

    #[test]
    fn global_signals_only_includes_sse_tab_id_when_enabled() {
        let signals = global_signals(
            SseMode::Enabled,
            Some(SseTabId::new("tab-123".to_string())),
        );

        assert!(signals.contains("sseTabId: 'tab-123'"));
        assert!(signals.contains("transportErrorSource"));
    }
}
