// http/error.rs
use derive_more::From;
use maud::Render;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    User(app::user::Error),
    Json(axum::extract::rejection::JsonRejection),
    Internal,
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

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
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

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = ?self, "request failed");
        self.into_render_response(crate::request::current_kind())
            .into_response()
    }
}

impl Error {
    pub fn into_render_response(&self, kind: crate::request::Kind) -> Response {
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

            Error::Internal => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
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
