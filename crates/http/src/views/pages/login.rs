use maud::Render;

use crate::views::page::UserNav;

pub struct Login<'a> {
    pub message: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Login<'_> {
    fn render(&self) -> maud::Markup {
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
                        a href="/register" { "Create one" }
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
