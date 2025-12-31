use maud::{Markup, html};

use crate::views::{components, layout, users::UserVm};

pub fn page(users: &[UserVm]) -> Markup {
    let content = html! {
        (components::site_header())
        main class="container" {
            section class="card" {
                h2 { "Create user" }
                form
                    hx-post="/users"
                    hx-target="#users"
                    hx-swap="beforeend"
                    method="post"
                    class="user-form" {
                    label { "Username" }
                    input type="text" name="username" placeholder="ada" required;
                    label { "Email" }
                    input type="email" name="email" placeholder="ada@example.com" required;
                    button type="submit" { "Create" }
                }
            }

            section class="card" {
                h2 { "Users" }
                ul id="users" class="user-list" {
                    @for user in users {
                        (crate::views::fragments::user_row::render(user))
                    }
                }
            }
        }
    };

    layout::page("User Lab", content)
}
