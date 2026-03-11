// http/error.rs
use axum::http::{
    HeaderValue,
    header::{CACHE_CONTROL, CONTENT_TYPE},
};
use datastar::prelude::PatchSignals;
use derive_more::From;
use maud::Render;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    User(app::user::Error),
    Auth(app::auth::Error),
    Chat(app::chat::Error),
    Json(axum::extract::rejection::JsonRejection),
    Internal,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::User(error) => write!(f, "{error}"),
            Self::Auth(error) => write!(f, "{error}"),
            Self::Chat(error) => write!(f, "{error}"),
            Self::Json(error) => write!(f, "{error}"),
            Self::Internal => write!(f, "internal server error"),
        }
    }
}

#[derive(Debug)]
enum Response {
    Page {
        status: axum::http::StatusCode,
        view: crate::views::page::Error,
    },
    Datastar {
        error: TransportError,
    },
}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Page { status, view } => {
                let content = view.render().into_string();
                (status, axum::response::Html(content)).into_response()
            }
            Self::Datastar { error } => {
                let mut body = String::new();
                body.push_str(
                    &PatchSignals::new(
                        serde_json::json!({
                            "transportErrorSource": "server",
                            "transportErrorKind": error.kind,
                            "transportErrorTitle": error.title,
                            "transportErrorMessage": error.message,
                            "transportErrorStatus": error.status.as_u16(),
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
        self.into_render_response(crate::request::current_kind())
            .into_response()
    }
}

impl From<axum_login::Error<crate::auth::Backend>> for Error {
    fn from(value: axum_login::Error<crate::auth::Backend>) -> Self {
        match value {
            axum_login::Error::Backend(error) => Error::Auth(error),
            axum_login::Error::Session(_) => Error::Internal,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct TransportError {
    kind: &'static str,
    status: axum::http::StatusCode,
    title: &'static str,
    message: &'static str,
}

impl Error {
    fn into_render_response(&self, kind: crate::request::Kind) -> Response {
        let error = match self {
            Error::Json(_) => (
                "validation",
                axum::http::StatusCode::BAD_REQUEST,
                "Bad request",
                "Invalid request body.",
            ),

            Error::User(app::user::Error::Domain { .. }) => (
                "validation",
                axum::http::StatusCode::BAD_REQUEST,
                "Invalid input",
                "Invalid input.",
            ),

            Error::User(app::user::Error::EmailTaken) => (
                "conflict",
                axum::http::StatusCode::CONFLICT,
                "Email already in use",
                "Email already in use.",
            ),
            Error::User(app::user::Error::Hashing { .. })
            | Error::User(app::user::Error::Repo { .. }) => (
                "internal",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
            ),
            Error::Auth(_) => (
                "auth",
                axum::http::StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Unable to authenticate.",
            ),
            Error::Chat(app::chat::Error::RateLimited) => (
                "rate_limit",
                axum::http::StatusCode::TOO_MANY_REQUESTS,
                "Too many messages",
                "Slow down and try again.",
            ),
            Error::Chat(app::chat::Error::RoomNotFound)
            | Error::Chat(app::chat::Error::MessageNotFound) => (
                "not_found",
                axum::http::StatusCode::NOT_FOUND,
                "Not found",
                "The chat room or message was not found.",
            ),
            Error::Chat(app::chat::Error::NotMember) => (
                "forbidden",
                axum::http::StatusCode::FORBIDDEN,
                "Access denied",
                "You are not a member of this room.",
            ),
            Error::Chat(app::chat::Error::ModerationStateConflict) => (
                "conflict",
                axum::http::StatusCode::CONFLICT,
                "Moderation conflict",
                "Message moderation state changed. Refresh and retry.",
            ),
            Error::Chat(app::chat::Error::InvalidRoomId { .. })
            | Error::Chat(app::chat::Error::InvalidMessageId { .. })
            | Error::Chat(app::chat::Error::InvalidModerationDecision { .. })
            | Error::Chat(app::chat::Error::InvalidModerationReason { .. })
            | Error::Chat(app::chat::Error::Domain { .. }) => (
                "validation",
                axum::http::StatusCode::BAD_REQUEST,
                "Invalid input",
                "Invalid chat request.",
            ),
            Error::Chat(app::chat::Error::Repo { .. }) => (
                "internal",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
            ),
            Error::Chat(app::chat::Error::InvalidStoredMessageStatus { .. })
            | Error::Chat(app::chat::Error::InvalidStoredModerationStatus { .. }) => (
                "internal",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
            ),

            Error::Internal => (
                "internal",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                "Internal server error.",
            ),
        };
        let error = TransportError {
            kind: error.0,
            status: error.1,
            title: error.2,
            message: error.3,
        };

        match kind {
            crate::request::Kind::Datastar => Response::Datastar { error },
            crate::request::Kind::Page => Response::Page {
                status: error.status,
                view: crate::views::page::Error::builder()
                    .title(error.title)
                    .message(error.message)
                    .status(error.status.as_u16())
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
        let response = Error::User(app::user::Error::EmailTaken)
            .into_render_response(crate::request::Kind::Page);

        match response {
            Response::Page { status, view } => {
                assert_eq!(status, StatusCode::CONFLICT);
                assert_eq!(view.title, "Email already in use");
                assert_eq!(view.message, "Email already in use.");
                assert_eq!(view.status, StatusCode::CONFLICT.as_u16());
            }
            Response::Datastar { .. } => panic!("expected page response"),
        }
    }

    #[tokio::test]
    async fn datastar_errors_use_signal_patch_contract() {
        let response = Error::Chat(app::chat::Error::NotMember)
            .into_render_response(crate::request::Kind::Datastar)
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
            Error::Chat(app::chat::Error::invalid_moderation_decision("bad room"))
                .into_render_response(crate::request::Kind::Page);

        match response {
            Response::Page { status, view } => {
                assert_eq!(status, StatusCode::BAD_REQUEST);
                assert_eq!(view.title, "Invalid input");
                assert_eq!(view.message, "Invalid chat request.");
            }
            Response::Datastar { .. } => panic!("expected page response"),
        }
    }
}
