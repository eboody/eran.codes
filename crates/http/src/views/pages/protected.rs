use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

#[derive(Builder)]
pub struct Protected {
    pub username: Text,
    pub email: Text,
    pub user: Option<page::UserNav>,
}

impl Render for Protected {
    fn render(&self) -> maud::Markup {
        let body = maud::html! {
            form data-account-actions method="post" action=(Route::Logout) {
                (partials::button::Button::builder()
                    .label(Text::from("Sign out"))
                    .variant(partials::button::Variant::Secondary)
                    .role(partials::button::Role::submit())
                    .data_attrs(vec![partials::button::DataAttr::flag("data-auth-submit")])
                    .build())
            }
        };
        let content = maud::html! {
            (partials::AuthShell::builder()
                .title(Text::from(format!("Welcome, {}", self.username)))
                .summary(Text::from(format!("Signed in as {}.", self.email)))
                .body(body)
                .variant(partials::AuthShellVariant::Account)
                .build())
        };

        page::Layout::builder()
            .title("Protected")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
