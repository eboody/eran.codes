use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use maud::Markup;

pub struct Html(pub Markup);

impl IntoResponse for Html {
    fn into_response(self) -> Response {
        let mut response = (StatusCode::OK, self.0.into_string()).into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        response
    }
}
