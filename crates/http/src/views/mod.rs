// Views: pages are full documents, partials are Datastar fragments.
moddef::moddef!(
    pub mod { page, pages, partials }
);

pub fn render(
    view: impl maud::Render,
) -> axum::response::Html<String> {
    axum::response::Html(view.render().into_string())
}
