// http/error.rs
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    User(service::user::Error),
    Json(axum::extract::rejection::JsonRejection),
}

#[derive(Debug, serde::Serialize)]
pub struct ClientError {
    pub code: &'static str,
    pub message: &'static str,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self, "request failed");

        let (status, body) = match &self {
            Error::Json(_) => (
                axum::http::StatusCode::BAD_REQUEST,
                ClientError {
                    code: "bad_request",
                    message: "Invalid request body.",
                },
            ),

            Error::User(service::user::Error::Domain(_)) => (
                axum::http::StatusCode::BAD_REQUEST,
                ClientError {
                    code: "invalid_input",
                    message: "Invalid input.",
                },
            ),

            Error::User(service::user::Error::EmailTaken) => (
                axum::http::StatusCode::CONFLICT,
                ClientError {
                    code: "email_taken",
                    message: "Email already in use.",
                },
            ),

            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                ClientError {
                    code: "internal",
                    message: "Internal server error.",
                },
            ),
        };

        (status, axum::Json(body)).into_response()
    }
}
