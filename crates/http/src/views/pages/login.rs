use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::views::page::UserNav;

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
        let content = maud::html! {
            main class="container ui-auth-main" {
                article class="ui-surface-card ui-auth-card" {
                    header class="ui-auth-header" {
                        h1 { "Sign in" }
                        p class="ui-auth-summary" { "Use your email address to continue." }
                    }

                    @if let Some(message) = self.message {
                        p class="ui-demo-result ui-error-alert" role="alert" { (message) }
                    }

                    form class="ui-auth-form" method="post" action=(Route::Login) {
                        @if let Some(next) = self.next {
                            input type="hidden" name="next" value=(next);
                        }
                        label class="ui-auth-field" {
                            span { "Email" }
                            input type="email" name="email" required;
                        }
                        label class="ui-auth-field" {
                            span { "Password" }
                            input type="password" name="password" required;
                        }
                        button class="ui-auth-submit" type="submit" { "Sign in" }
                    }
                    p class="ui-auth-note" {
                        "Need an account? "
                        @if let Some(next) = next_query {
                            a href=(format!("{}?next={}", Route::Register, next)) { "Create one" }
                        } @else {
                            a href=(Route::Register) { "Create one" }
                        }
                    }
                }
            }
        };

        crate::views::page::Layout::builder()
            .title("Sign in")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
