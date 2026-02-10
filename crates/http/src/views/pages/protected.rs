use bon::Builder;
use maud::Render;

use crate::views::page::UserNav;
use crate::paths::Route;
use crate::types::Text;

#[derive(Builder)]
pub struct Protected {
    pub username: Text,
    pub email: Text,
    pub user: Option<UserNav>,
}

impl Render for Protected {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                article {
                    header {
                        h1 { "Welcome, " (self.username.to_string()) }
                    }
                    p { "Signed in as " (self.email.to_string()) "." }
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
