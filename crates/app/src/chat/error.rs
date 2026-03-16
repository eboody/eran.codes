use snafu::prelude::*;
use strum_macros::Display;

pub type Result<T> = core::result::Result<T, Error>;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Domain { source: domain::chat::Error },
    #[snafu(display("{source}"))]
    Repository { source: RepositoryError },
    #[snafu(display("invalid room id: {source}"))]
    InvalidRoomId { source: uuid::Error },
    #[snafu(display("invalid message id: {source}"))]
    InvalidMessageId { source: uuid::Error },
    #[snafu(display("invalid moderation decision: {decision}"))]
    InvalidModerationDecision { decision: String },
    #[snafu(display("invalid moderation reason: {source}"))]
    InvalidModerationReason {
        source: super::ModerationReasonError,
    },
    #[snafu(display("invalid stored message status: {status}"))]
    InvalidStoredMessageStatus { status: String },
    #[snafu(display("invalid stored moderation status: {status}"))]
    InvalidStoredModerationStatus { status: String },
    #[snafu(display("message moderation state changed"))]
    ModerationStateConflict,
    #[snafu(display("chat rate limit exceeded"))]
    RateLimited,
    #[snafu(display("chat room not found"))]
    RoomNotFound,
    #[snafu(display("chat message not found"))]
    MessageNotFound,
    #[snafu(display("user is not a member of this room"))]
    NotMember,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum RepositoryOperation {
    #[strum(serialize = "create room")]
    CreateRoom,
    #[strum(serialize = "find room by id")]
    FindRoomById,
    #[strum(serialize = "find room by name")]
    FindRoomByName,
    #[strum(serialize = "list room messages")]
    ListRoomMessages,
    #[strum(serialize = "find message by id")]
    FindMessageById,
    #[strum(serialize = "insert message")]
    InsertMessage,
    #[strum(serialize = "add room membership")]
    AddRoomMembership,
    #[strum(serialize = "check room membership")]
    CheckRoomMembership,
    #[strum(serialize = "update message status")]
    UpdateMessageStatus,
    #[strum(serialize = "enqueue moderation item")]
    EnqueueModerationItem,
    #[strum(serialize = "list moderation queue")]
    ListModerationQueue,
    #[strum(serialize = "complete moderation item")]
    CompleteModerationItem,
    #[strum(serialize = "check chat rate limit")]
    CheckChatRateLimit,
    #[strum(serialize = "record chat audit entry")]
    RecordChatAuditEntry,
}

#[derive(Debug, Snafu)]
pub enum RepositoryError {
    #[snafu(display("chat repository query failed while {operation}: {source}"))]
    Query {
        operation: RepositoryOperation,
        source: BoxError,
    },
    #[snafu(display("failed to decode room name: {source}"))]
    DecodeRoomName {
        source: domain::chat::room::NameError,
    },
    #[snafu(display("failed to decode client id: {source}"))]
    DecodeClientId { source: domain::chat::ClientIdError },
    #[snafu(display("failed to decode message body: {source}"))]
    DecodeMessageBody {
        source: domain::chat::message::BodyError,
    },
    #[snafu(display("failed to decode moderation room name: {source}"))]
    DecodeModerationRoomName {
        source: domain::chat::room::NameError,
    },
    #[snafu(display("failed to decode moderation message body: {source}"))]
    DecodeModerationMessageBody {
        source: domain::chat::message::BodyError,
    },
    #[snafu(display("failed to decode moderation reason: {source}"))]
    DecodeModerationReason {
        source: super::ModerationReasonError,
    },
    #[snafu(display("failed to decode moderation timestamp: {source}"))]
    DecodeModerationTimestamp { source: super::TimestampTextError },
}

impl From<domain::chat::Error> for Error {
    fn from(error: domain::chat::Error) -> Self {
        Self::Domain { source: error }
    }
}

fn box_error(source: impl std::error::Error + Send + Sync + 'static) -> BoxError {
    Box::new(source)
}

impl Error {
    pub fn query_repository(
        operation: RepositoryOperation,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Repository {
            source: RepositoryError::Query {
                operation,
                source: box_error(source),
            },
        }
    }

    pub fn decode_room_name(source: domain::chat::room::NameError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeRoomName { source },
        }
    }

    pub fn decode_client_id(source: domain::chat::ClientIdError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeClientId { source },
        }
    }

    pub fn decode_message_body(source: domain::chat::message::BodyError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeMessageBody { source },
        }
    }

    pub fn decode_moderation_room_name(source: domain::chat::room::NameError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeModerationRoomName { source },
        }
    }

    pub fn decode_moderation_message_body(
        source: domain::chat::message::BodyError,
    ) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeModerationMessageBody { source },
        }
    }

    pub fn decode_moderation_reason(source: super::ModerationReasonError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeModerationReason { source },
        }
    }

    pub fn decode_moderation_timestamp(source: super::TimestampTextError) -> Self {
        Self::Repository {
            source: RepositoryError::DecodeModerationTimestamp { source },
        }
    }

    pub fn invalid_room_id(source: uuid::Error) -> Self {
        Self::InvalidRoomId { source }
    }

    pub fn invalid_message_id(source: uuid::Error) -> Self {
        Self::InvalidMessageId { source }
    }

    pub fn invalid_moderation_decision(decision: impl Into<String>) -> Self {
        Self::InvalidModerationDecision {
            decision: decision.into(),
        }
    }

    pub fn invalid_moderation_reason(source: super::ModerationReasonError) -> Self {
        Self::InvalidModerationReason { source }
    }

    pub fn invalid_stored_message_status(status: impl Into<String>) -> Self {
        Self::InvalidStoredMessageStatus {
            status: status.into(),
        }
    }

    pub fn invalid_stored_moderation_status(status: impl Into<String>) -> Self {
        Self::InvalidStoredModerationStatus {
            status: status.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn repo_error_preserves_source() {
        let error = Error::query_repository(
            RepositoryOperation::FindRoomById,
            std::io::Error::other("db down"),
        );

        assert_eq!(
            error
                .source()
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("chat repository query failed while find room by id: db down"),
        );
        assert_eq!(
            error
                .source()
                .and_then(|source| source.source())
                .map(std::string::ToString::to_string)
                .as_deref(),
            Some("db down"),
        );
    }

    #[test]
    fn invalid_room_id_preserves_uuid_source() {
        let source = "not-a-uuid"
            .parse::<uuid::Uuid>()
            .expect_err("invalid uuid");
        let error = Error::invalid_room_id(source);

        assert!(error.source().is_some());
    }

    #[test]
    fn invalid_moderation_reason_preserves_source() {
        let source = super::super::ModerationReason::try_new("x".repeat(201))
            .expect_err("invalid moderation reason");
        let error = Error::invalid_moderation_reason(source);

        assert!(error.source().is_some());
    }
}
