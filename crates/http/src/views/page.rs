use bon::Builder;
use maud::{Markup, Render};
use crate::paths::Route;
use crate::types::Text;

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
}

impl Render for Layout<'_> {
    fn render(&self) -> Markup {
        maud::html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    title { (self.title) }
                    link rel="icon" type="image/png" sizes="1024x1024" href="/static/eran.codes.png";
                    link rel="apple-touch-icon" sizes="1024x1024" href="/static/eran.codes.png";
                    link rel="stylesheet" href="/static/pico.min.css";
                    link rel="stylesheet" href="/static/app.css";
                    script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.7/bundles/datastar.js" {}
                    script src="/static/css-scope-inline.js" {}
                }
                body data-init=(format!("@get('{}')", Route::Events)) {
                    header class="container" {
                        nav {
                            ul {
                                li {
                                    a href=(Route::Home) { "eran.codes" }
                                }
                            }
                            @match &self.user {
                                Some(user) => {
                                    ul {
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
                                    ul {
                                        li { a href=(Route::Login) { "Sign in" } }
                                        li { a href=(Route::Register) { "Create account" } }
                                    }
                                }
                            }
                        }
                    }
                    div id="error-target" {}
                    (self.content.clone())
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
