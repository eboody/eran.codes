// http/error.rs
use axum::response::IntoResponse;
use derive_more::From;
use maud::Render;

pub type Result<T> = core::result::Result<T, ContextualError>;

#[derive(Debug, From)]
pub enum Error {
    User(app::user::Error),
    Json(axum::extract::rejection::JsonRejection),
}

#[derive(Debug)]
pub enum Response {
    Page {
        status: axum::http::StatusCode,
        view: crate::views::page::Error,
    },
    Partial {
        status: axum::http::StatusCode,
        view: crate::views::partials::Error,
    },
}

impl Response {
    pub fn into_response(self) -> axum::response::Response {
        match self {
            Self::Page { status, view } => {
                let content = view.render().into_string();
                (status, axum::response::Html(content)).into_response()
            }
            Self::Partial { status, view } => {
                let content = view.render().into_string();
                (status, axum::response::Html(content)).into_response()
            }
        }
    }
}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        Response::into_response(self)
    }
}

#[derive(Debug)]
pub struct ContextualError {
    response: Response,
    source: Error,
}

impl ContextualError {
    pub fn new(response: Response, source: Error) -> Self {
        Self { response, source }
    }
}

impl From<(crate::request::Kind, Error)> for ContextualError {
    fn from(value: (crate::request::Kind, Error)) -> Self {
        let (kind, err) = value;
        Self::new(err.into_response(kind), err)
    }
}

impl From<(crate::request::Kind, app::user::Error)> for ContextualError {
    fn from(value: (crate::request::Kind, app::user::Error)) -> Self {
        let (kind, err) = value;
        let error = Error::from(err);
        Self::new(error.into_response(kind), error)
    }
}

impl
    From<(
        crate::request::Kind,
        axum::extract::rejection::JsonRejection,
    )> for ContextualError
{
    fn from(
        value: (
            crate::request::Kind,
            axum::extract::rejection::JsonRejection,
        ),
    ) -> Self {
        let (kind, err) = value;
        let error = Error::from(err);
        Self::new(error.into_response(kind), error)
    }
}

impl axum::response::IntoResponse for ContextualError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self.source, "request failed");
        self.response.into_response()
    }
}

impl Error {
    pub fn into_response(&self, kind: crate::request::Kind) -> Response {
        let (status, title, message) = match self {
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

        match kind {
            crate::request::Kind::Datastar => Response::Partial {
                status,
                view: crate::views::partials::Error { message },
            },
            crate::request::Kind::Page => Response::Page {
                status,
                view: crate::views::page::Error {
                    title,
                    message,
                    status: status.as_u16(),
                },
            },
        }
    }
}
