// http/error.rs
use axum::http::{
    HeaderValue,
    header::{CACHE_CONTROL, CONTENT_TYPE},
};
use datastar::prelude::PatchSignals;
use maud::Render;
use snafu::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    User { source: app::user::Error },
    #[snafu(display("{source}"))]
    Auth { source: app::auth::Error },
    #[snafu(display("{source}"))]
    Chat { source: app::chat::Error },
    #[snafu(display("{source}"))]
    ExtractJson {
        source: axum::extract::rejection::JsonRejection,
    },
    #[snafu(display("internal server error"))]
    Internal,
}

impl From<app::user::Error> for Error {
    fn from(source: app::user::Error) -> Self {
        Self::User { source }
    }
}

#[derive(Debug)]
enum ErrorResponse {
    Page {
        status: axum::http::StatusCode,
        view: crate::views::page::Error,
    },
    Datastar {
        presentation: ErrorPresentation,
    },
}

impl From<app::auth::Error> for Error {
    fn from(source: app::auth::Error) -> Self {
        Self::Auth { source }
    }
}

impl From<app::chat::Error> for Error {
    fn from(source: app::chat::Error) -> Self {
        Self::Chat { source }
    }
}

impl From<axum::extract::rejection::JsonRejection> for Error {
    fn from(source: axum::extract::rejection::JsonRejection) -> Self {
        Self::ExtractJson { source }
    }
}

impl axum::response::IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Page { status, view } => {
                let content = view.render().into_string();
                (status, axum::response::Html(content)).into_response()
            }
            Self::Datastar { presentation } => {
                let mut body = String::new();
                body.push_str(
                    &PatchSignals::new(
                        serde_json::json!({
                            "transportErrorSource": "server",
                            "transportErrorKind": presentation.kind,
                            "transportErrorTitle": presentation.title,
                            "transportErrorMessage": presentation.message,
                            "transportErrorStatus": presentation.status.as_u16(),
                            "transportRetrying": false,
                        })
                        .to_string(),
                    )
                    .into_datastar_event()
                    .to_string(),
                );
                (
                    [
                        (CONTENT_TYPE, HeaderValue::from_static("text/event-stream")),
                        (
                            CACHE_CONTROL,
                            HeaderValue::from_static("no-cache, no-transform"),
                        ),
                    ],
                    body,
                )
                    .into_response()
            }
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(error = %self, error_debug = ?self, "request failed");
        self.into_error_response(crate::request::current_kind())
            .into_response()
    }
}

impl From<axum_login::Error<crate::auth::Backend>> for Error {
    fn from(value: axum_login::Error<crate::auth::Backend>) -> Self {
        match value {
            axum_login::Error::Backend(error) => Self::from(error),
            axum_login::Error::Session(_) => Error::Internal,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ErrorPresentation {
    kind: &'static str,
    status: axum::http::StatusCode,
    title: &'static str,
    message: &'static str,
}

impl Error {
    fn presentation(&self) -> ErrorPresentation {
        match self {
            Error::ExtractJson { .. } => ErrorPresentation {
                kind: "validation",
                status: axum::http::StatusCode::BAD_REQUEST,
                title: "Bad request",
                message: "Invalid request body.",
            },
            Error::User {
                source: app::user::Error::Domain { .. },
            } => ErrorPresentation {
                kind: "validation",
                status: axum::http::StatusCode::BAD_REQUEST,
                title: "Invalid input",
                message: "Invalid input.",
            },
            Error::User {
                source: app::user::Error::EmailTaken,
            } => ErrorPresentation {
                kind: "conflict",
                status: axum::http::StatusCode::CONFLICT,
                title: "Email already in use",
                message: "Email already in use.",
            },
            Error::User {
                source:
                    app::user::Error::HashPassword { .. } | app::user::Error::Repository { .. },
            } => ErrorPresentation {
                kind: "internal",
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                title: "Internal server error",
                message: "Internal server error.",
            },
            Error::Auth { .. } => ErrorPresentation {
                kind: "auth",
                status: axum::http::StatusCode::UNAUTHORIZED,
                title: "Unauthorized",
                message: "Unable to authenticate.",
            },
            Error::Chat {
                source: app::chat::Error::RateLimited,
            } => ErrorPresentation {
                kind: "rate_limit",
                status: axum::http::StatusCode::TOO_MANY_REQUESTS,
                title: "Too many messages",
                message: "Slow down and try again.",
            },
            Error::Chat {
                source: app::chat::Error::RoomNotFound | app::chat::Error::MessageNotFound,
            } => ErrorPresentation {
                kind: "not_found",
                status: axum::http::StatusCode::NOT_FOUND,
                title: "Not found",
                message: "The chat room or message was not found.",
            },
            Error::Chat {
                source: app::chat::Error::NotMember,
            } => ErrorPresentation {
                kind: "forbidden",
                status: axum::http::StatusCode::FORBIDDEN,
                title: "Access denied",
                message: "You are not a member of this room.",
            },
            Error::Chat {
                source: app::chat::Error::ModerationStateConflict,
            } => ErrorPresentation {
                kind: "conflict",
                status: axum::http::StatusCode::CONFLICT,
                title: "Moderation conflict",
                message: "Message moderation state changed. Refresh and retry.",
            },
            Error::Chat {
                source:
                    app::chat::Error::InvalidRoomId { .. }
                    | app::chat::Error::InvalidMessageId { .. }
                    | app::chat::Error::InvalidModerationDecision { .. }
                    | app::chat::Error::InvalidModerationReason { .. }
                    | app::chat::Error::Domain { .. },
            } => ErrorPresentation {
                kind: "validation",
                status: axum::http::StatusCode::BAD_REQUEST,
                title: "Invalid input",
                message: "Invalid chat request.",
            },
            Error::Chat {
                source:
                    app::chat::Error::Repository { .. }
                    | app::chat::Error::InvalidStoredMessageStatus { .. }
                    | app::chat::Error::InvalidStoredModerationStatus { .. },
            }
            | Error::Internal => ErrorPresentation {
                kind: "internal",
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                title: "Internal server error",
                message: "Internal server error.",
            },
        }
    }

    fn into_error_response(&self, kind: crate::request::Kind) -> ErrorResponse {
        let presentation = self.presentation();
        match kind {
            crate::request::Kind::Datastar => ErrorResponse::Datastar { presentation },
            crate::request::Kind::Page => ErrorResponse::Page {
                status: presentation.status,
                view: crate::views::page::Error::builder()
                    .title(presentation.title)
                    .message(presentation.message)
                    .status(presentation.status.as_u16())
                    .maybe_with_user(None)
                    .build(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::to_bytes,
        http::{StatusCode, header::CONTENT_TYPE},
        response::IntoResponse,
    };

    #[test]
    fn page_errors_keep_http_status_codes() {
        let response = Error::from(app::user::Error::EmailTaken)
            .into_error_response(crate::request::Kind::Page);

        match response {
            ErrorResponse::Page { status, view } => {
                assert_eq!(status, StatusCode::CONFLICT);
                assert_eq!(view.title, "Email already in use");
                assert_eq!(view.message, "Email already in use.");
                assert_eq!(view.status, StatusCode::CONFLICT.as_u16());
            }
            ErrorResponse::Datastar { .. } => panic!("expected page response"),
        }
    }

    #[tokio::test]
    async fn datastar_errors_use_signal_patch_contract() {
        let response = Error::from(app::chat::Error::NotMember)
            .into_error_response(crate::request::Kind::Datastar)
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some("text/event-stream"),
        );

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8_lossy(&body);

        assert!(body.contains("transportErrorSource"));
        assert!(body.contains("\"server\""));
        assert!(body.contains("transportErrorKind"));
        assert!(body.contains("\"forbidden\""));
        assert!(body.contains("transportErrorTitle"));
        assert!(body.contains("Access denied"));
        assert!(body.contains("transportErrorMessage"));
        assert!(body.contains("You are not a member of this room."));
        assert!(body.contains("transportErrorStatus"));
        assert!(body.contains("403"));
    }

    #[test]
    fn validation_errors_map_to_bad_request_for_pages() {
        let response =
            Error::from(app::chat::Error::invalid_moderation_decision("bad room"))
                .into_error_response(crate::request::Kind::Page);

        match response {
            ErrorResponse::Page { status, view } => {
                assert_eq!(status, StatusCode::BAD_REQUEST);
                assert_eq!(view.title, "Invalid input");
                assert_eq!(view.message, "Invalid chat request.");
            }
            ErrorResponse::Datastar { .. } => panic!("expected page response"),
        }
    }
}
