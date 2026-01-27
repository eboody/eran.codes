use bon::Builder;
use maud::Render;
use urlencoding;

use crate::views::page::UserNav;

#[derive(Builder)]
pub struct Login<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Login<'_> {
    fn render(&self) -> maud::Markup {
        let next_query =
            self.next.map(|value| urlencoding::encode(value).to_string());
        let content = maud::html! {
            main class="container" {
                article {
                    header {
                        h1 { "Sign in" }
                    }
                    p { "Use your email address to continue." }

                    @if let Some(message) = self.message {
                        p role="alert" { (message) }
                    }

                    form method="post" action="/login" {
                        @if let Some(next) = self.next {
                            input type="hidden" name="next" value=(next);
                        }
                        label {
                            "Email"
                            input type="email" name="email" required;
                        }
                        label {
                            "Password"
                            input type="password" name="password" required;
                        }
                        button type="submit" { "Sign in" }
                    }
                    p {
                        "Need an account? "
                        @if let Some(next) = next_query {
                            a href=(format!("/register?next={}", next)) { "Create one" }
                        } @else {
                            a href="/register" { "Create one" }
                        }
                    }
                }
            }
        };

        crate::views::page::Layout::builder()
            .title("Sign in")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
