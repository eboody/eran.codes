use maud::Render;

use crate::views::page::UserNav;

pub struct Protected {
    pub username: String,
    pub email: String,
    pub user: Option<UserNav>,
}

impl Render for Protected {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                article {
                    header {
                        h1 { "Welcome, " (self.username) }
                    }
                    p { "Signed in as " (self.email) "." }
                    form method="post" action="/logout" {
                        button type="submit" { "Sign out" }
                    }
                }
            }
        };

        crate::views::page::Layout {
            title: "Protected",
            content,
            user: self.user.clone(),
        }
        .render()
    }
}
