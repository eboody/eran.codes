use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{button, AuthShell, AuthShellVariant};

#[derive(Builder)]
pub struct Protected {
    pub username: Text,
    pub email: Text,
    pub user: Option<UserNav>,
}

impl Render for Protected {
    fn render(&self) -> maud::Markup {
        let body = maud::html! {
            form data-account-actions method="post" action=(Route::Logout) {
                (button::Button::builder()
                    .label(Text::from("Sign out"))
                    .variant(button::Variant::Secondary)
                    .role(button::Role::submit())
                    .data_attrs(vec![button::DataAttr::flag("data-auth-submit")])
                    .build())
            }
        };
        let content = maud::html! {
            (AuthShell::builder()
                .title(Text::from(format!("Welcome, {}", self.username)))
                .summary(Text::from(format!("Signed in as {}.", self.email)))
                .body(body)
                .variant(AuthShellVariant::Account)
                .build())
        };

        crate::views::page::Layout::builder()
            .title("Protected")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
