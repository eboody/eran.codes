use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::views::page::UserNav;

#[derive(Builder)]
pub struct Register<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Register<'_> {
    fn render(&self) -> maud::Markup {
        let next_query =
            self.next.map(|value| urlencoding::encode(value).to_string());
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

                    form method="post" action=(Route::Register.as_str()) {
                        @if let Some(next) = self.next {
                            input type="hidden" name="next" value=(next);
                        }
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
                        @if let Some(next) = next_query {
                            a href=(format!("{}?next={}", Route::Login.as_str(), next)) { "Sign in" }
                        } @else {
                            a href=(Route::Login.as_str()) { "Sign in" }
                        }
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
