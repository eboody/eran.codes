use maud::{Markup, Render};

use crate::paths::Route;
use crate::views::page;

pub(super) fn render(
    title: &str,
    content: Markup,
    current_route: Route,
    user: Option<page::UserNav>,
) -> Markup {
    let page_content = page::Frame::builder().content(content).build().render();

    page::Layout::builder()
        .title(title)
        .content(page_content)
        .nav_mode(page::NavMode::Portfolio)
        .current_route(current_route)
        .maybe_with_user(user)
        .build()
        .render()
}
