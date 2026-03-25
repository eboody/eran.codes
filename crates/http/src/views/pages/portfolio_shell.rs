use maud::{Markup, Render};

use crate::paths::Route;
use crate::views::page;

pub(super) fn render(
    title: &str,
    content: Markup,
    current_route: Route,
    user: Option<page::UserNav>,
) -> Markup {
    render_with_frame_width(
        title,
        content,
        current_route,
        user,
        page::FrameWidth::Standard,
    )
}

pub(super) fn render_with_frame_width(
    title: &str,
    content: Markup,
    current_route: Route,
    user: Option<page::UserNav>,
    frame_width: page::FrameWidth,
) -> Markup {
    let page_content = page::Frame::builder()
        .content(content)
        .width(frame_width)
        .build()
        .render();

    page::Layout::builder()
        .title(title)
        .content(page_content)
        .nav_mode(page::NavMode::Portfolio)
        .current_route(current_route)
        .maybe_with_user(user)
        .build()
        .render()
}
