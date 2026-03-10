use bon::Builder;
use maud::{Markup, Render};

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::components::{
    NavAuth, NavBar, NavBrand, NavLink, NavLinkList, NavSignedIn,
};

pub(crate) const PORTFOLIO_RESUME_URL: &str = "/static/resume.txt";
pub(crate) const PORTFOLIO_GITHUB_URL: &str = "https://github.com/eboody/eran.codes";
pub(crate) const PORTFOLIO_LINKEDIN_URL: &str =
    "https://www.linkedin.com/search/results/all/?keywords=Eran%20Boodnero";
pub(crate) const PORTFOLIO_CONTACT_URL: &str = "mailto:eboodnero@gmail.com";

#[derive(Clone, Debug, Builder)]
pub struct UserNav {
    pub username: Text,
    pub email: Text,
}

impl Render for UserNav {
    fn render(&self) -> Markup {
        maud::html! {
            ul class="ui-nav-list ui-nav-auth" {
                li {
                    span class="ui-nav-auth-text" { "Signed in as " (&self.username) }
                }
                li {
                    a class="ui-nav-link" href=(Route::Protected) { "Account" }
                }
                li {
                    form method="post" action=(Route::Logout) {
                        button type="submit" class="secondary ui-nav-auth-action" { "Sign out" }
                    }
                }
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
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        let sse_tab_id = crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string());
        let brand = NavBrand::builder()
            .label(Text::from("eran.codes"))
            .href(Text::from(Route::Home.as_str()))
            .light_logo_src(Text::from("/static/eran.codes-light.svg"))
            .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
            .build();
        let portfolio_links = NavLinkList::builder()
            .class_name(Text::from("ui-nav-list ui-nav-links"))
            .children(vec![
                NavLink::builder()
                    .label(Text::from("Work"))
                    .href(Text::from(Route::Work.as_str()))
                    .build(),
                NavLink::builder()
                    .label(Text::from("Live Lab"))
                    .href(Text::from(Route::Lab.as_str()))
                    .build(),
                NavLink::builder()
                    .label(Text::from("Resume"))
                    .href(Text::from(PORTFOLIO_RESUME_URL))
                    .build(),
                NavLink::builder()
                    .label(Text::from("GitHub"))
                    .href(Text::from(PORTFOLIO_GITHUB_URL))
                    .external(true)
                    .build(),
                NavLink::builder()
                    .label(Text::from("LinkedIn"))
                    .href(Text::from(PORTFOLIO_LINKEDIN_URL))
                    .external(true)
                    .build(),
                NavLink::builder()
                    .label(Text::from("Contact"))
                    .href(Text::from(PORTFOLIO_CONTACT_URL))
                    .build(),
            ])
            .build();
        let auth = match self.nav_mode {
            NavMode::Portfolio => NavAuth::Hidden,
            NavMode::App => match &self.user {
                Some(user) => NavAuth::SignedIn(
                    NavSignedIn::builder()
                        .username(user.username.clone())
                        .account_href(Text::from(Route::Protected.as_str()))
                        .logout_action(Text::from(Route::Logout.as_str()))
                        .build(),
                ),
                None => NavAuth::Guest(
                    NavLinkList::builder()
                        .class_name(Text::from("ui-nav-list ui-nav-auth"))
                        .children(vec![
                            NavLink::builder()
                                .label(Text::from("Sign in"))
                                .href(Text::from(Route::Login.as_str()))
                                .build(),
                            NavLink::builder()
                                .label(Text::from("Create account"))
                                .href(Text::from(Route::Register.as_str()))
                                .build(),
                        ])
                        .build(),
                ),
            },
        };
        let nav_bar = NavBar::builder()
            .brand(brand)
            .links(portfolio_links)
            .auth(auth)
            .build();
        let body_content = maud::html! {
            (nav_bar)
            div id="error-target" {}
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
                    link rel="stylesheet" href="/static/app.css";
                    link
                        rel="stylesheet"
                        href="https://cdn.jsdelivr.net/gh/iconoir-icons/iconoir@main/css/iconoir.css";
                    script type="module" src="/static/datastar.js" {}
                    script src="/static/surreal.js" {}
                }
                @match self.sse_mode {
                    SseMode::Enabled => {
                        body
                            data-signals=(format!("{{sseTabId: '{}'}}", sse_tab_id))
                            data-init=(format!("@get('{}')", Route::Events))
                        { (body_content) }
                    }
                    SseMode::Disabled => {
                        body { (body_content) }
                    }
                }
            }
        }
    }
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
        let content = maud::html! {
            main class="container" {
                article {
                    header {
                        h1 { (self.title) }
                    }
                    p { (self.message) }
                    p { "Status: " (self.status) }
                }
            }
        };

        Layout::builder()
            .title(self.title)
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
