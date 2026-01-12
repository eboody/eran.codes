use maud::{Markup, Render};

#[derive(Clone, Debug)]
pub struct UserNav {
    pub username: String,
    pub email: String,
}

pub struct Layout<'a> {
    pub title: &'a str,
    pub content: Markup,
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
                body data-on:load__window="@get('/events')" {
                    header class="container" {
                        nav {
                            ul {
                                li {
                                    a href="/" { "eran.codes" }
                                }
                            }
                            @match &self.user {
                                Some(user) => {
                                    ul {
                                        li { span { "Signed in as " (user.username) } }
                                        li { a href="/protected" { "Account" } }
                                        li {
                                            form method="post" action="/logout" {
                                                button type="submit" class="secondary" { "Sign out" }
                                            }
                                        }
                                    }
                                }
                                None => {
                                    ul {
                                        li { a href="/login" { "Sign in" } }
                                        li { a href="/register" { "Create account" } }
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

#[derive(Debug)]
pub struct Error {
    pub title: &'static str,
    pub message: &'static str,
    pub status: u16,
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

        Layout {
            title: self.title,
            content,
            user: self.user.clone(),
        }
        .render()
    }
}
