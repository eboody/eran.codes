use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

#[derive(Builder)]
pub struct Register<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<page::UserNav>,
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
                (partials::button::Button::builder()
                    .label(Text::from("Create account"))
                    .variant(partials::button::Variant::Primary)
                    .role(partials::button::Role::submit())
                    .data_attrs(vec![partials::button::DataAttr::flag("data-auth-submit")])
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
            (partials::AuthShell::builder()
                .title(Text::from("Create account"))
                .summary(Text::from(
                    "Create an account to inspect session-backed auth in the live app.",
                ))
                .maybe_message(self.message.map(Text::from))
                .body(body)
                .footer(footer)
                .build())
        };
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title("Create account")
            .content(content)
            .nav_mode(page::NavMode::Auth)
            .current_route(Route::Register)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn register_page_uses_compact_auth_nav_switch() {
        let markup = Register::builder().build().render().into_string();

        assert!(markup.contains("data-nav-layout=\"split\""));
        assert!(markup.contains("<a data-nav-link data-nav-auth-switch href=\"/login\">Sign in</a>"));
        assert!(!markup.contains("<a data-nav-link data-nav-auth-switch href=\"/register\">Create account</a>"));
    }
}
