use maud::{Markup, html};

pub fn site_header() -> Markup {
    html! {
        header class="site-header" {
            h1 { "User Lab" }
            p { "HTMX + Maud demo flow." }
        }
    }
}
