// Views: pages are full documents, partials are HTMX fragments.
pub mod layout;
pub mod pages;
pub mod partials;

pub fn render(markup: maud::Markup) -> axum::response::Html<String> {
    axum::response::Html(markup.into_string())
}
