use bon::Builder;
use maud::Render;
use urlencoding;

use crate::paths::Route;
use crate::views::page::UserNav;

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
        let content = maud::html! {
            main class="container ui-auth-main" {
                article class="ui-surface-card ui-auth-card" {
                    header class="ui-auth-header" {
                        h1 { "Create account" }
                        p class="ui-auth-summary" { "Pick a username and password to get started." }
                    }

                    @if let Some(message) = self.message {
                        p class="ui-demo-result ui-error-alert" role="alert" { (message) }
                    }

                    form class="ui-auth-form" method="post" action=(Route::Register) {
                        @if let Some(next) = self.next {
                            input type="hidden" name="next" value=(next);
                        }
                        label class="ui-auth-field" {
                            span { "Username" }
                            input type="text" name="username" required;
                        }
                        label class="ui-auth-field" {
                            span { "Email" }
                            input type="email" name="email" required;
                        }
                        label class="ui-auth-field" {
                            span { "Password" }
                            input type="password" name="password" required;
                        }
                        button class="ui-auth-submit" type="submit" { "Create account" }
                    }

                    p class="ui-auth-note" {
                        "Already have an account? "
                        @if let Some(next) = next_query {
                            a href=(format!("{}?next={}", Route::Login, next)) { "Sign in" }
                        } @else {
                            a href=(Route::Login) { "Sign in" }
                        }
                    }
                }
            }
        };

        crate::views::page::Layout::builder()
            .title("Create account")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
