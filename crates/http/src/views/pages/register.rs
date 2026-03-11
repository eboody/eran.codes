use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{button, AuthShell};

#[derive(Builder)]
pub struct Register<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Register<'_> {
    fn render(&self) -> maud::Markup {
        let next_query = self
            .next
            .map(|value| urlencoding::encode(value).to_string());
        let body = maud::html! {
            form data-auth-form method="post" action=(Route::Register) {
                @if let Some(next) = self.next {
                    input type="hidden" name="next" value=(next);
                }
                label data-auth-field {
                    span { "Username" }
                    input type="text" name="username" required;
                }
                label data-auth-field {
                    span { "Email" }
                    input type="email" name="email" required;
                }
                label data-auth-field {
                    span { "Password" }
                    input type="password" name="password" required;
                }
                (button::Button::builder()
                    .label(Text::from("Create account"))
                    .variant(button::Variant::Primary)
                    .role(button::Role::submit())
                    .data_attrs(vec![button::DataAttr::flag("data-auth-submit")])
                    .build())
            }
        };
        let footer = maud::html! {
            p data-auth-note {
                "Already have an account? "
                @if let Some(next) = next_query {
                    a href=(format!("{}?next={}", Route::Login, next)) { "Sign in" }
                } @else {
                    a href=(Route::Login) { "Sign in" }
                }
            }
        };
        let content = maud::html! {
            (AuthShell::builder()
                .title(Text::from("Create account"))
                .summary(Text::from("Pick a username and password to get started."))
                .maybe_message(self.message.map(Text::from))
                .body(body)
                .footer(footer)
                .build())
        };

        crate::views::page::Layout::builder()
            .title("Create account")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
