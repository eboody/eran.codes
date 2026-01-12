use maud::Render;

pub struct Login<'a> {
    pub message: Option<&'a str>,
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
                }
            }
        };

        crate::views::page::Layout {
            title: "Sign in",
            content,
        }
        .render()
    }
}
