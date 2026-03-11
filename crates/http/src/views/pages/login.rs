use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::UserNav;
use crate::views::partials::{button, AuthShell};

#[derive(Builder)]
pub struct Login<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<UserNav>,
}

impl Render for Login<'_> {
    fn render(&self) -> maud::Markup {
        let next_query = self
            .next
            .map(|value| urlencoding::encode(value).to_string());
        let body = maud::html! {
            form data-auth-form method="post" action=(Route::Login) {
                @if let Some(next) = self.next {
                    input type="hidden" name="next" value=(next);
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
                    .label(Text::from("Sign in"))
                    .variant(button::Variant::Primary)
                    .role(button::Role::submit())
                    .data_attrs(vec![button::DataAttr::flag("data-auth-submit")])
                    .build())
            }
        };
        let footer = maud::html! {
            p data-auth-note {
                "Need an account? "
                @if let Some(next) = next_query {
                    a href=(format!("{}?next={}", Route::Register, next)) { "Create one" }
                } @else {
                    a href=(Route::Register) { "Create one" }
                }
            }
        };
        let content = maud::html! {
            (AuthShell::builder()
                .title(Text::from("Sign in"))
                .summary(Text::from("Use your email address to continue."))
                .maybe_message(self.message.map(Text::from))
                .body(body)
                .footer(footer)
                .build())
        };

        crate::views::page::Layout::builder()
            .title("Sign in")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
