use app::chat::{
    PendingMutationResult, RepositoryOperation, Result, RoomRole, audit, moderation,
};
use async_trait::async_trait;
use domain::chat;
use snafu::prelude::*;
use sqlx::types::time;
use sqlx::{PgPool, Row, postgres::PgRow};

const RATE_LIMIT_WINDOW_SECS: i64 = 10;
const RATE_LIMIT_MAX: i64 = 5;

type PersistenceResult<T> = std::result::Result<T, ChatPersistenceError>;

#[derive(Debug, Snafu)]
enum ChatPersistenceError {
    #[snafu(display("chat persistence query failed while {operation}"))]
    Query {
        operation: RepositoryOperation,
        source: sqlx::Error,
    },
    #[snafu(display("failed to decode room name"))]
    DecodeRoomName { source: chat::room::name::Error },
    #[snafu(display("failed to decode client id"))]
    DecodeClientId { source: chat::client::IdError },
    #[snafu(display("failed to decode message body"))]
    DecodeMessageBody { source: chat::message::BodyError },
    #[snafu(display("invalid stored message status: {status}"))]
    InvalidStoredMessageStatus { status: String },
    #[snafu(display("failed to decode moderation room name"))]
    DecodeModerationRoomName { source: chat::room::name::Error },
    #[snafu(display("failed to decode moderation message body"))]
    DecodeModerationMessageBody { source: chat::message::BodyError },
    #[snafu(display("invalid stored moderation status: {status}"))]
    InvalidStoredModerationStatus { status: String },
    #[snafu(display("failed to decode moderation reason"))]
    DecodeModerationReason {
        source: app::chat::moderation::ReasonError,
    },
    #[snafu(display("failed to decode moderation timestamp"))]
    DecodeModerationTimestamp {
        source: app::chat::TimestampTextError,
    },
}

impl From<ChatPersistenceError> for app::chat::Error {
    fn from(error: ChatPersistenceError) -> Self {
        match error {
            ChatPersistenceError::Query { operation, source } => {
                app::chat::Error::query_repository(operation, source)
            }
            ChatPersistenceError::DecodeRoomName { source } => {
                app::chat::Error::decode_room_name(source)
            }
            ChatPersistenceError::DecodeClientId { source } => {
                app::chat::Error::decode_client_id(source)
            }
            ChatPersistenceError::DecodeMessageBody { source } => {
                app::chat::Error::decode_message_body(source)
            }
            ChatPersistenceError::InvalidStoredMessageStatus { status } => {
                app::chat::Error::invalid_stored_message_status(status)
            }
            ChatPersistenceError::DecodeModerationRoomName { source } => {
                app::chat::Error::decode_moderation_room_name(source)
            }
            ChatPersistenceError::DecodeModerationMessageBody { source } => {
                app::chat::Error::decode_moderation_message_body(source)
            }
            ChatPersistenceError::InvalidStoredModerationStatus { status } => {
                app::chat::Error::invalid_stored_moderation_status(status)
            }
            ChatPersistenceError::DecodeModerationReason { source } => {
                app::chat::Error::decode_moderation_reason(source)
            }
            ChatPersistenceError::DecodeModerationTimestamp { source } => {
                app::chat::Error::decode_moderation_timestamp(source)
            }
        }
    }
}

pub struct Repository {
    pg: PgPool,
}

impl Repository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    fn status_from_db(value: &str) -> PersistenceResult<chat::message::Status> {
        value.parse::<chat::message::Status>().map_err(|_| {
            ChatPersistenceError::InvalidStoredMessageStatus {
                status: value.to_owned(),
            }
        })
    }

    fn status_to_db(status: chat::message::Status) -> &'static str {
        match status {
            chat::message::Status::Visible => "visible",
            chat::message::Status::Pending => "pending",
            chat::message::Status::Removed => "removed",
        }
    }

    fn room_from_row(row: &PgRow) -> PersistenceResult<chat::Room> {
        let name = row.get::<String, _>("name");
        let name = chat::room::Name::try_new(name).context(DecodeRoomNameSnafu)?;

        Ok(chat::Room {
            id: chat::room::Id::from_uuid(row.get::<uuid::Uuid, _>("id")),
            name,
            created_by: chat::UserId::from_uuid(row.get::<uuid::Uuid, _>("created_by")),
        })
    }

    fn client_id_from_db(
        value: Option<String>,
    ) -> PersistenceResult<Option<chat::client::Id>> {
        value
            .map(|client_id| {
                chat::client::Id::try_new(client_id).context(DecodeClientIdSnafu)
            })
            .transpose()
    }

    fn message_from_row(row: &PgRow) -> PersistenceResult<chat::Message> {
        let body = row.get::<String, _>("body");
        let body = chat::message::Body::try_new(body).context(DecodeMessageBodySnafu)?;
        let status = Self::status_from_db(row.get::<String, _>("status").as_str())?;

        Ok(chat::Message {
            id: chat::message::Id::from_uuid(row.get::<uuid::Uuid, _>("id")),
            room_id: chat::room::Id::from_uuid(row.get::<uuid::Uuid, _>("room_id")),
            user_id: chat::UserId::from_uuid(row.get::<uuid::Uuid, _>("user_id")),
            body,
            status,
            client_id: Self::client_id_from_db(row.get::<Option<String>, _>("client_id"))?,
            created_at: offset_to_system_time(
                row.get::<time::OffsetDateTime, _>("created_at"),
            ),
        })
    }
}

#[async_trait]
impl app::chat::Repository for Repository {
    async fn create_room(&self, room: &chat::Room) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_rooms (id, name, created_by) VALUES ($1, $2, $3)"
        );
        sqlx::query(
            r#"
            INSERT INTO chat_rooms (id, name, created_by)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(room.id.as_uuid())
        .bind(room.name.to_string())
        .bind(room.created_by.as_uuid())
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::CreateRoom,
        })?;

        Ok(())
    }

    async fn find_room(&self, room_id: &chat::room::Id) -> Result<Option<chat::Room>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, name, created_by FROM chat_rooms WHERE id = $1",
            db_bind_1 = %room_id.as_uuid()
        );
        let record = sqlx::query(
            r#"
            SELECT id, name, created_by
            FROM chat_rooms
            WHERE id = $1
            "#,
        )
        .bind(room_id.as_uuid())
        .fetch_optional(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::FindRoomById,
        })?;

        Ok(record.as_ref().map(Self::room_from_row).transpose()?)
    }

    async fn find_room_by_name(
        &self,
        name: &chat::room::Name,
    ) -> Result<Option<chat::Room>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, name, created_by FROM chat_rooms WHERE name = $1"
        );
        let record = sqlx::query(
            r#"
            SELECT id, name, created_by
            FROM chat_rooms
            WHERE name = $1
            "#,
        )
        .bind(name.to_string())
        .fetch_optional(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::FindRoomByName,
        })?;

        Ok(record.as_ref().map(Self::room_from_row).transpose()?)
    }

    async fn list_messages(
        &self,
        room_id: &chat::room::Id,
        limit: usize,
    ) -> Result<Vec<chat::Message>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, room_id, user_id, body, status, client_id, created_at FROM chat_messages WHERE room_id = $1 ORDER BY created_at DESC LIMIT $2"
        );
        let rows = sqlx::query(
            r#"
            SELECT id, room_id, user_id, body, status, client_id, created_at
            FROM chat_messages
            WHERE room_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(room_id.as_uuid())
        .bind(limit as i64)
        .fetch_all(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::ListRoomMessages,
        })?;

        Ok(rows
            .iter()
            .map(Self::message_from_row)
            .collect::<PersistenceResult<_>>()?)
    }

    async fn find_message(
        &self,
        message_id: &chat::message::Id,
    ) -> Result<Option<chat::Message>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, room_id, user_id, body, status, client_id, created_at FROM chat_messages WHERE id = $1"
        );
        let row = sqlx::query(
            r#"
            SELECT id, room_id, user_id, body, status, client_id, created_at
            FROM chat_messages
            WHERE id = $1
            "#,
        )
        .bind(message_id.as_uuid())
        .fetch_optional(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::FindMessageById,
        })?;

        Ok(row.as_ref().map(Self::message_from_row).transpose()?)
    }

    async fn insert_message(&self, message: &chat::Message) -> Result<()> {
        let status = Self::status_to_db(message.status);
        let client_id = message
            .client_id
            .as_ref()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "NULL".to_string());
        let created_at = time::OffsetDateTime::from(message.created_at).to_string();
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_messages (id, room_id, user_id, body, status, client_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            db_bind_1 = %message.id.as_uuid(),
            db_bind_2 = %message.room_id.as_uuid(),
            db_bind_3 = %message.user_id.as_uuid(),
            db_bind_4 = %message.body,
            db_bind_5 = status,
            db_bind_6 = client_id,
            db_bind_7 = created_at
        );
        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, room_id, user_id, body, status, client_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(message.id.as_uuid())
        .bind(message.room_id.as_uuid())
        .bind(message.user_id.as_uuid())
        .bind(message.body.to_string())
        .bind(status)
        .bind(message.client_id.as_ref().map(|value| value.to_string()))
        .bind(time::OffsetDateTime::from(message.created_at))
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::InsertMessage,
        })?;

        Ok(())
    }

    async fn add_membership(
        &self,
        room_id: &chat::room::Id,
        user_id: &chat::UserId,
        role: RoomRole,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_room_memberships (room_id, user_id, role) VALUES ($1, $2, $3)",
            db_bind_1 = %room_id.as_uuid(),
            db_bind_2 = %user_id.as_uuid(),
            db_bind_3 = %role
        );
        sqlx::query(
            r#"
            INSERT INTO chat_room_memberships (room_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (room_id, user_id) DO NOTHING
            "#,
        )
        .bind(room_id.as_uuid())
        .bind(user_id.as_uuid())
        .bind(role.to_string())
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::AddRoomMembership,
        })?;

        Ok(())
    }

    async fn is_member(
        &self,
        room_id: &chat::room::Id,
        user_id: &chat::UserId,
    ) -> Result<bool> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT 1 FROM chat_room_memberships WHERE room_id = $1 AND user_id = $2",
            db_bind_1 = %room_id.as_uuid(),
            db_bind_2 = %user_id.as_uuid()
        );
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM chat_room_memberships
            WHERE room_id = $1 AND user_id = $2
            "#,
        )
        .bind(room_id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_optional(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::CheckRoomMembership,
        })?;

        Ok(row.is_some())
    }

    async fn update_message_status(
        &self,
        message_id: &chat::message::Id,
        status: chat::message::Status,
    ) -> Result<PendingMutationResult> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "UPDATE chat_messages SET status = $2 WHERE id = $1 AND status = 'pending'"
        );
        let result = sqlx::query(
            r#"
            UPDATE chat_messages
            SET status = $2
            WHERE id = $1
              AND status = 'pending'
            "#,
        )
        .bind(message_id.as_uuid())
        .bind(Self::status_to_db(status))
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::UpdateMessageStatus,
        })?;

        Ok(if result.rows_affected() == 1 {
            PendingMutationResult::Applied
        } else {
            PendingMutationResult::NotPendingOrMissing
        })
    }
}

pub struct ModerationQueue {
    pg: PgPool,
}

impl ModerationQueue {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl app::chat::moderation::Queue for ModerationQueue {
    async fn enqueue(
        &self,
        message_id: &chat::message::Id,
        reason: &moderation::Reason,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_moderation_queue (message_id, reason) VALUES ($1, $2)"
        );
        sqlx::query(
            r#"
            INSERT INTO chat_moderation_queue (message_id, reason)
            VALUES ($1, $2)
            "#,
        )
        .bind(message_id.as_uuid())
        .bind(reason.to_string())
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::EnqueueModerationItem,
        })?;

        Ok(())
    }

    async fn list_pending(&self, limit: usize) -> Result<Vec<app::chat::moderation::Item>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT queue entries"
        );
        let rows = sqlx::query(
            r#"
            SELECT q.message_id,
                   q.reason,
                   q.status,
                   q.reviewed_at,
                   m.room_id,
                   m.user_id,
                   m.body,
                   r.name AS room_name,
                   m.created_at::text AS created_at
            FROM chat_moderation_queue q
            JOIN chat_messages m ON m.id = q.message_id
            JOIN chat_rooms r ON r.id = m.room_id
            WHERE q.status = 'pending'
            ORDER BY m.created_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::ListModerationQueue,
        })?;

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let room_name = row.get::<String, _>("room_name");
            let room_name = chat::room::Name::try_new(room_name)
                .context(DecodeModerationRoomNameSnafu)?;
            let body = row.get::<String, _>("body");
            let body = chat::message::Body::try_new(body)
                .context(DecodeModerationMessageBodySnafu)?;
            let queue_status = row.get::<String, _>("status");
            let queue_status =
                queue_status
                    .parse::<moderation::QueueStatus>()
                    .map_err(|_| ChatPersistenceError::InvalidStoredModerationStatus {
                        status: queue_status.clone(),
                    })?;
            let reason = moderation::Reason::try_new(row.get::<String, _>("reason"))
                .context(DecodeModerationReasonSnafu)?;
            let created_at =
                app::chat::TimestampText::try_new(row.get::<String, _>("created_at"))
                    .context(DecodeModerationTimestampSnafu)?;

            items.push(
                app::chat::moderation::Item::builder()
                    .message_id(chat::message::Id::from_uuid(
                        row.get::<uuid::Uuid, _>("message_id"),
                    ))
                    .room_id(chat::room::Id::from_uuid(
                        row.get::<uuid::Uuid, _>("room_id"),
                    ))
                    .room_name(room_name)
                    .user_id(chat::UserId::from_uuid(row.get::<uuid::Uuid, _>("user_id")))
                    .body(body)
                    .queue_status(queue_status)
                    .reason(reason)
                    .created_at(created_at)
                    .build(),
            );
        }

        Ok(items)
    }

    async fn complete_if_pending(
        &self,
        message_id: &chat::message::Id,
        reviewer_id: &chat::UserId,
        decision: app::chat::moderation::Decision,
        reason: Option<moderation::Reason>,
    ) -> Result<PendingMutationResult> {
        let status = match decision {
            app::chat::moderation::Decision::Approve => moderation::QueueStatus::Approved,
            app::chat::moderation::Decision::Remove => moderation::QueueStatus::Removed,
        };

        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "UPDATE chat_moderation_queue (pending only)"
        );
        let result = sqlx::query(
            r#"
            UPDATE chat_moderation_queue
            SET status = $2,
                reviewer_id = $3,
                reviewed_at = now(),
                reason = COALESCE($4, reason)
            WHERE message_id = $1
              AND status = 'pending'
            "#,
        )
        .bind(message_id.as_uuid())
        .bind(status.to_string())
        .bind(reviewer_id.as_uuid())
        .bind(reason.map(|value| value.to_string()))
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::CompleteModerationItem,
        })?;

        Ok(if result.rows_affected() == 1 {
            PendingMutationResult::Applied
        } else {
            PendingMutationResult::NotPendingOrMissing
        })
    }
}

pub struct RateLimiter {
    pg: PgPool,
}

impl RateLimiter {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

fn offset_to_system_time(value: time::OffsetDateTime) -> std::time::SystemTime {
    let seconds = value.unix_timestamp();
    let nanos = value.nanosecond();
    if seconds >= 0 {
        std::time::SystemTime::UNIX_EPOCH
            + std::time::Duration::from_secs(seconds as u64)
            + std::time::Duration::from_nanos(nanos as u64)
    } else {
        std::time::SystemTime::UNIX_EPOCH
            - std::time::Duration::from_secs(seconds.unsigned_abs())
            - std::time::Duration::from_nanos(nanos as u64)
    }
}

#[async_trait]
impl app::chat::RateLimiter for RateLimiter {
    async fn check(&self, room_id: &chat::room::Id, user_id: &chat::UserId) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "UPSERT chat_rate_limits",
            db_bind_1 = %room_id.as_uuid(),
            db_bind_2 = %user_id.as_uuid(),
            db_bind_3 = RATE_LIMIT_WINDOW_SECS,
            db_bind_4 = RATE_LIMIT_MAX
        );
        let row = sqlx::query(
            r#"
            WITH updated AS (
                INSERT INTO chat_rate_limits (room_id, user_id, window_start, count)
                VALUES ($1, $2, now(), 1)
                ON CONFLICT (room_id, user_id) DO UPDATE
                SET window_start = CASE
                        WHEN chat_rate_limits.window_start < now() - ($3 || ' seconds')::interval
                            THEN now()
                        ELSE chat_rate_limits.window_start
                    END,
                    count = CASE
                        WHEN chat_rate_limits.window_start < now() - ($3 || ' seconds')::interval
                            THEN 1
                        ELSE chat_rate_limits.count
                            + 1
                    END
                RETURNING window_start, count
            )
            SELECT (count <= $4) AS allowed
            FROM updated
            "#,
        )
        .bind(room_id.as_uuid())
        .bind(user_id.as_uuid())
        .bind(RATE_LIMIT_WINDOW_SECS)
        .bind(RATE_LIMIT_MAX)
        .fetch_one(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::CheckChatRateLimit,
        })?;

        let allowed = row.get::<bool, _>("allowed");
        if allowed {
            Ok(())
        } else {
            Err(app::chat::Error::RateLimited)
        }
    }
}

pub struct AuditLog {
    pg: PgPool,
}

impl AuditLog {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl app::chat::audit::Log for AuditLog {
    async fn record(&self, entry: audit::Entry) -> Result<()> {
        let room_id = entry.room_id.as_uuid().to_string();
        let actor_id = entry.actor_id.as_uuid().to_string();
        let action = entry.action.to_string();
        let metadata = entry
            .metadata
            .into_iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                )
            })
            .collect::<serde_json::Map<String, serde_json::Value>>();
        let metadata = serde_json::Value::Object(metadata);

        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_audit_log (room_id, actor_user_id, action, metadata_json) VALUES ($1, $2, $3, $4)",
            db_bind_1 = room_id,
            db_bind_2 = actor_id,
            db_bind_3 = action,
            db_bind_4 = %metadata
        );
        sqlx::query(
            r#"
            INSERT INTO chat_audit_log (room_id, actor_user_id, action, metadata_json)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(entry.room_id.as_uuid())
        .bind(entry.actor_id.as_uuid())
        .bind(entry.action.to_string())
        .bind(metadata)
        .execute(&self.pg)
        .await
        .context(QuerySnafu {
            operation: RepositoryOperation::RecordChatAuditEntry,
        })?;

        Ok(())
    }
}
