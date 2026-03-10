use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;

#[derive(Builder)]
pub struct Protected {
    pub username: Text,
    pub email: Text,
    pub user: Option<UserNav>,
}

impl Render for Protected {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container ui-auth-main ui-account-main" {
                article class="ui-surface-card ui-auth-card ui-account-card" {
                    header class="ui-auth-header ui-account-header" {
                        h1 { "Welcome, " (&self.username) }
                    }
                    p class="ui-auth-summary ui-account-summary" { "Signed in as " (&self.email) "." }
                    form class="ui-account-actions" method="post" action=(Route::Logout) {
                        button class="secondary ui-auth-submit" type="submit" { "Sign out" }
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
