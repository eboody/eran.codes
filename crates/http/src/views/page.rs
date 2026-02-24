use bon::Builder;
use maud::{Markup, Render};
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

#[derive(Builder)]
pub struct Layout<'a> {
    pub title: &'a str,
    pub content: Markup,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
    #[builder(default)]
    pub enable_sse: bool,
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        let sse_tab_id = crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string());
        let body_content = maud::html! {
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
                            ul class="site-auth-links" {
                                li { span { "Signed in as " (&user.username) } }
                                li { a href=(Route::Protected) { "Account" } }
                                li {
                                    form method="post" action=(Route::Logout) {
                                        button type="submit" class="secondary" { "Sign out" }
                                    }
                                }
                            }
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
                    script type="module" {
                        (maud::PreEscaped(r#"
document.addEventListener("DOMContentLoaded", async () => {
  await import("https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js");
});
                        "#))
                    }
                    script src="/static/css-scope-inline.js" {}
                }
                @if self.enable_sse {
                    body
                        class="portfolio-shell"
                        data-signals=(format!("{{sseTabId: '{}'}}", sse_tab_id))
                        data-init=(format!("@get('{}')", Route::Events))
                    {
                        (body_content)
                    }
                } @else {
                    body class="portfolio-shell" {
                        (body_content)
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
