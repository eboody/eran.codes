use bon::Builder;
use maud::Render;

use crate::views::page::UserNav;

#[derive(Builder)]
pub struct Register<'a> {
    pub message: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Register<'_> {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                article {
                    header {
                        h1 { "Create account" }
                    }
                    p { "Pick a username and password to get started." }

                    @if let Some(message) = self.message {
                        p role="alert" { (message) }
                    }

                    form method="post" action="/register" {
                        label {
                            "Username"
                            input type="text" name="username" required;
                        }
                        label {
                            "Email"
                            input type="email" name="email" required;
                        }
                        label {
                            "Password"
                            input type="password" name="password" required;
                        }
                        button type="submit" { "Create account" }
                    }

                    p {
                        "Already have an account? "
                        a href="/login" { "Sign in" }
                    }
                }
            }
        };

        crate::views::page::Layout::builder()
            .title("Create account")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
