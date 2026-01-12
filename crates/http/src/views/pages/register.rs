use maud::Render;

pub struct Register<'a> {
    pub message: Option<&'a str>,
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

        crate::views::page::Layout {
            title: "Create account",
            content,
        }
        .render()
    }
}
