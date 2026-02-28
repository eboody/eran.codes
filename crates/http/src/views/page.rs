use bon::Builder;
use maud::{Markup, Render};
use maud_extensions::css;

use crate::paths::Route;
use crate::types::Text;

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
            ul class="site-auth-links" {
                li { span { "Signed in as " (&self.username) } }
                li { a href=(Route::Protected) { "Account" } }
                li {
                    form method="post" action=(Route::Logout) {
                        button type="submit" class="secondary" { "Sign out" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct AppShellStyles;

impl Render for AppShellStyles {
    fn render(&self) -> Markup {
        maud::html! {
            ({
                css! {
                    me {
                      font-family: "Space Grotesk", "Avenir Next", "Segoe UI", sans-serif;
                      font-size: 17px;
                      line-height: 1.58;
                      color: color-mix(in srgb, var(--pico-color) 96%, white 4%);
                      background:
                        radial-gradient(circle at 2% 0%, var(--portfolio-accent-a), transparent 45%),
                        radial-gradient(circle at 100% 12%, var(--portfolio-accent-b), transparent 46%),
                        linear-gradient(180deg, var(--portfolio-bg), var(--portfolio-bg-alt));
                      min-height: 100vh;
                    }
                    me .container {
                      max-width: min(1120px, 94vw);
                    }
                    me main.container {
                      padding-top: 0.6rem;
                      padding-bottom: 3.5rem;
                    }
                    me h1,
                    me h2,
                    me h3 {
                      letter-spacing: -0.02rem;
                    }
                    me .site-nav-wrap {
                      position: sticky;
                      top: 0.55rem;
                      z-index: 20;
                      margin-top: 0.45rem;
                    }
                    me .site-nav {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      justify-content: space-between;
                      gap: 0.55rem 1rem;
                      border-radius: var(--ui-radius-md);
                      border: 1px solid color-mix(in srgb, var(--pico-muted-color) 20%, transparent);
                      background: color-mix(in srgb, var(--pico-card-background-color) 96%, transparent);
                      backdrop-filter: blur(10px);
                      box-shadow: 0 4px 14px color-mix(in srgb, black 14%, transparent);
                      padding: 0.5rem 0.85rem;
                    }
                    me .site-nav ul {
                      list-style: none;
                      margin: 0;
                      padding: 0;
                    }
                    me .site-brand-links a {
                      font-size: 0.86rem;
                      font-weight: 700;
                      letter-spacing: 0.03rem;
                      text-transform: uppercase;
                    }
                    me .portfolio-nav-links {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      gap: 0.22rem;
                      flex: 1;
                      justify-self: center;
                      justify-content: center;
                    }
                    me .site-auth-links {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      gap: 0.24rem;
                      margin-left: auto;
                      justify-self: end;
                    }
                    me .site-auth-links span {
                      font-size: 0.84rem;
                      color: var(--pico-muted-color);
                    }
                    me .site-auth-links form {
                      margin: 0;
                    }
                    me .site-auth-links button,
                    me .portfolio-nav-links a,
                    me .site-brand-links a {
                      margin-bottom: 0;
                    }
                    me .site-auth-links button {
                      padding: 0.34rem 0.72rem;
                      font-size: 0.8rem;
                    }
                    me .portfolio-nav-links a {
                      font-size: 0.84rem;
                      color: var(--pico-muted-color);
                    }
                    me .portfolio-nav-links a:hover {
                      color: var(--pico-color);
                    }
                    me button,
                    me .button {
                      border-radius: 9px;
                      font-weight: 600;
                    }
                    me a.button {
                      display: inline-flex;
                      align-items: center;
                      justify-content: center;
                      padding: 0.52rem 1.05rem;
                      background: linear-gradient(
                        180deg,
                        color-mix(in srgb, var(--pico-primary) 86%, white 14%),
                        color-mix(in srgb, var(--pico-primary) 88%, black 12%)
                      );
                      color: var(--pico-primary-inverse);
                      text-decoration: none;
                      border: 1px solid color-mix(in srgb, var(--pico-primary) 56%, transparent);
                      box-shadow: 0 4px 12px color-mix(in srgb, black 20%, transparent);
                    }
                    me a.button.secondary {
                      background: color-mix(in srgb, var(--pico-card-background-color) 80%, transparent);
                      color: var(--pico-color);
                      border: 1px solid var(--ui-border-soft);
                    }
                    me button.secondary,
                    me .button.secondary {
                      background: color-mix(in srgb, var(--pico-card-background-color) 80%, transparent);
                      border: 1px solid var(--ui-border-soft);
                    }
                    me .muted,
                    me [data-muted] {
                      color: color-mix(in srgb, var(--pico-muted-color) 94%, var(--pico-color) 6%);
                    }
                    me ::selection {
                      background: hsl(217 91% 60% / 0.35);
                      color: hsl(210 40% 98%);
                    }
                    me ::-moz-selection {
                      background: hsl(217 91% 60% / 0.35);
                      color: hsl(210 40% 98%);
                    }
                    me .pill {
                      display: inline-flex;
                      align-items: center;
                      padding: 0.1rem 0.4rem;
                      border-radius: 999px;
                      border: 1px solid var(--pico-muted-border-color);
                      font-size: 0.72rem;
                      font-weight: 600;
                      letter-spacing: 0.01rem;
                      line-height: 1.15;
                      color: var(--pill-accent, inherit);
                      border-color: var(--pill-accent, var(--pico-muted-border-color));
                    }
                    me .pill.method {
                      text-transform: uppercase;
                    }
                    me .pill.path {
                      font-family: var(--ui-font-mono);
                      font-size: 0.72rem;
                      letter-spacing: 0.01rem;
                    }
                    me .method-get {
                      border-color: rgba(120, 190, 255, 0.6);
                      color: rgba(140, 210, 255, 0.95);
                    }
                    me .method-post {
                      border-color: rgba(140, 210, 140, 0.7);
                      color: rgba(160, 220, 160, 0.95);
                    }
                    me .method-put,
                    me .method-patch {
                      border-color: rgba(255, 196, 80, 0.7);
                      color: rgba(255, 196, 80, 0.95);
                    }
                    me .method-delete {
                      border-color: rgba(255, 120, 120, 0.75);
                      color: rgba(255, 140, 140, 0.95);
                    }
                    me .method-other {
                      border-color: rgba(180, 180, 200, 0.6);
                      color: rgba(200, 200, 220, 0.9);
                    }
                    me .pill.status {
                      font-variant-numeric: tabular-nums;
                    }
                    me .status-2xx {
                      border-color: rgba(120, 210, 140, 0.7);
                      color: rgba(150, 220, 160, 0.95);
                    }
                    me .status-3xx {
                      border-color: rgba(120, 190, 255, 0.6);
                      color: rgba(140, 210, 255, 0.95);
                    }
                    me .status-4xx {
                      border-color: rgba(255, 196, 80, 0.7);
                      color: rgba(255, 196, 80, 0.95);
                    }
                    me .status-5xx {
                      border-color: rgba(255, 120, 120, 0.75);
                      color: rgba(255, 140, 140, 0.95);
                    }
                    me .status-unknown {
                      border-color: rgba(180, 180, 200, 0.6);
                      color: rgba(200, 200, 220, 0.9);
                    }
                    me .log-level-info {
                      border-color: rgba(80, 160, 255, 0.6);
                      color: rgba(120, 190, 255, 0.9);
                    }
                    me .log-level-warn {
                      border-color: rgba(255, 196, 80, 0.7);
                      color: rgba(255, 196, 80, 0.95);
                    }
                    me .log-level-error {
                      border-color: rgba(255, 96, 96, 0.7);
                      color: rgba(255, 128, 128, 0.95);
                    }
                    me .log-level-debug {
                      border-color: rgba(140, 210, 140, 0.65);
                      color: rgba(160, 220, 160, 0.9);
                    }
                    me .log-level-trace {
                      border-color: rgba(160, 160, 180, 0.6);
                      color: rgba(180, 180, 200, 0.85);
                    }
                    me .log-target {
                      background: color-mix(
                        in srgb,
                        var(--pico-card-background-color) 86%,
                        var(--pico-muted-color) 14%
                      );
                      border-color: color-mix(in srgb, var(--pico-muted-color) 30%, transparent);
                      color: var(--pico-muted-color);
                    }
                    me .log-fields {
                      color: rgb(148, 163, 184);
                      border-color: rgb(148, 163, 184);
                    }
                    me .pill.badge-secondary {
                      background: transparent;
                      color: var(--pico-color);
                      border: 1px solid var(--pico-muted-border-color);
                    }
                    me .pill.badge-you {
                      background: #0f766e;
                      color: #f8fafc;
                      border-color: #0f766e;
                    }
                    me .pill.badge-demo {
                      background: #f59e0b;
                      color: #1f2937;
                      border-color: #f59e0b;
                    }
                    @media (hover: hover) {
                      me a.button:hover,
                      me button:hover {
                        filter: brightness(1.03);
                      }
                    }
                    @media (max-width: 768px) {
                      me .site-nav-wrap {
                        top: 0.35rem;
                      }
                      me .site-nav {
                        border-radius: 16px;
                        padding: 0.55rem 0.7rem;
                        gap: 0.45rem 0.75rem;
                      }
                      me .site-auth-links {
                        margin-left: auto;
                      }
                      me .portfolio-nav-links {
                        order: 3;
                        flex: 0 0 100%;
                        justify-content: flex-start;
                        overflow-x: auto;
                        flex-wrap: nowrap;
                        padding-bottom: 0.15rem;
                        scrollbar-width: thin;
                      }
                      me .site-brand-links a {
                        font-size: 0.8rem;
                      }
                      me .portfolio-nav-links a,
                      me .site-auth-links a {
                        font-size: 0.78rem;
                      }
                      me .pill.path {
                        max-width: 100%;
                        overflow-wrap: anywhere;
                      }
                    }
                }
            })
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SseMode {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Builder)]
pub struct Layout<'a> {
    pub title: &'a str,
    pub content: Markup,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
    #[builder(default)]
    pub sse_mode: SseMode,
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        let sse_tab_id = crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string());
        let body_content = maud::html! {
            (AppShellStyles.render())
            header class="container site-nav-wrap" {
                nav class="site-nav" {
                    ul class="site-brand-links" {
                        li {
                            a href=(Route::Home) { "eran.codes" }
                        }
                    }
                    ul class="portfolio-nav-links" {
                        li {
                            a href=(PORTFOLIO_RESUME_URL) { "Resume" }
                        }
                        li {
                            a
                                href=(PORTFOLIO_GITHUB_URL)
                                target="_blank"
                                rel="noopener noreferrer"
                            {
                                "GitHub"
                            }
                        }
                        li {
                            a
                                href=(PORTFOLIO_LINKEDIN_URL)
                                target="_blank"
                                rel="noopener noreferrer"
                            {
                                "LinkedIn"
                            }
                        }
                        li {
                            a href=(PORTFOLIO_CONTACT_URL) { "Contact" }
                        }
                    }
                    @match &self.user {
                        Some(user) => {
                            (user.render())
                        }
                        None => {
                            ul class="site-auth-links" {
                                li { a href=(Route::Login) { "Sign in" } }
                                li { a href=(Route::Register) { "Create account" } }
                            }
                        }
                    }
                }
            }
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
                    link rel="icon" type="image/png" sizes="1024x1024" href="/static/eran.codes.png";
                    link rel="apple-touch-icon" sizes="1024x1024" href="/static/eran.codes.png";
                    link rel="preconnect" href="https://fonts.googleapis.com";
                    link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                    link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500;600&display=swap";
                    link rel="stylesheet" href="/static/pico.min.css";
                    link rel="stylesheet" href="/static/app.css";
                    script type="module" src="/static/datastar.js" {}
                    script src="/static/css-scope-inline.js" {}
                }
                @match self.sse_mode {
                    SseMode::Enabled => {
                        body
                            data-signals=(format!("{{sseTabId: '{}'}}", sse_tab_id))
                            data-init=(format!("@get('{}')", Route::Events))
                        {
                            (body_content)
                        }
                    }
                    SseMode::Disabled => {
                        body {
                            (body_content)
                        }
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
