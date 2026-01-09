// http/error.rs
use derive_more::From;
use maud::Render;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    User(app::user::Error),
    Json(axum::extract::rejection::JsonRejection),
}

#[derive(Debug)]
pub struct PartialResponse {
    status: axum::http::StatusCode,
    view: crate::views::partials::Error,
}

impl axum::response::IntoResponse for PartialResponse {
    fn into_response(self) -> axum::response::Response {
        let content = self.view.render().into_string();
        (self.status, axum::response::Html(content)).into_response()
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self, "request failed");
        self.into_partial_response().into_response()
    }
}

impl Error {
    pub fn into_partial_response(&self) -> PartialResponse {
        let (status, _title, message) = match self {
            Error::Json(_) => (
                axum::http::StatusCode::BAD_REQUEST,
                "Bad request",
                "Invalid request body.",
            ),

            Error::User(app::user::Error::Domain(_)) => (
                axum::http::StatusCode::BAD_REQUEST,
                "Invalid input",
                "Invalid input.",
            ),

            Error::User(app::user::Error::EmailTaken) => (
                axum::http::StatusCode::CONFLICT,
                "Email already in use",
                "Email already in use.",
            ),

            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
            ),
        };

        PartialResponse {
            status,
            view: crate::views::partials::Error { message },
        }
    }
}
