use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

#[derive(Builder)]
pub struct Login<'a> {
    pub message: Option<&'a str>,
    pub next: Option<&'a str>,
    pub user: Option<page::UserNav>,
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
                (partials::button::Button::builder()
                    .label(Text::from("Sign in"))
                    .variant(partials::button::Variant::Primary)
                    .role(partials::button::Role::submit())
                    .data_attrs(vec![partials::button::DataAttr::flag("data-auth-submit")])
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
            (partials::AuthShell::builder()
                .title(Text::from("Sign in"))
                .summary(Text::from(
                    "Use your email address to continue into the live app.",
                ))
                .maybe_message(self.message.map(Text::from))
                .body(body)
                .footer(footer)
                .build())
        };
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title("Sign in")
            .content(content)
            .nav_mode(page::NavMode::Auth)
            .current_route(Route::Login)
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
    fn login_page_uses_compact_auth_nav_switch() {
        let markup = Login::builder().build().render().into_string();

        assert!(markup.contains("data-nav-layout=\"split\""));
        assert!(markup.contains("<a data-nav-link data-nav-auth-switch href=\"/register\">Create account</a>"));
        assert!(!markup.contains("<a data-nav-link data-nav-auth-switch href=\"/login\">Sign in</a>"));
    }
}
