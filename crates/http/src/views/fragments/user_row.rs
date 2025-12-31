use maud::{Markup, html};

use crate::views::users::UserVm;

pub fn render(user: &UserVm) -> Markup {
    html! {
        li class="user-row" {
            strong { (user.username) }
            span { (user.email) }
        }
    }
}
