use bon::Builder;
use maud::Render;

use crate::views::page::UserNav;
use crate::paths::Route;

#[derive(Builder)]
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
                    form method="post" action=(Route::Logout.as_str()) {
                        button type="submit" { "Sign out" }
                    }
                }
            }
        };

        crate::views::page::Layout::builder()
            .title("Protected")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
