// Views: pages are full documents, partials are Datastar fragments.
moddef::moddef!(
    mod { layout },
    pub mod { pages, partials }
);

pub use layout::PageLayout;

pub fn render(
    view: impl maud::Render,
) -> axum::response::Html<String> {
    axum::response::Html(view.render().into_string())
}
