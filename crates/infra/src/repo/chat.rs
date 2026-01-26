pub use SqlxChatAuditLog as AuditLog;
pub use SqlxChatModerationQueue as ModerationQueue;
pub use SqlxChatRateLimiter as RateLimiter;
pub use SqlxChatRepository as Repository;

use app::chat::{AuditEntry, Error, Result};
use async_trait::async_trait;
use domain::chat;
use sqlx::{PgPool, Row};

const RATE_LIMIT_WINDOW_SECS: i64 = 10;
const RATE_LIMIT_MAX: i64 = 5;

pub struct SqlxChatRepository {
    pg: PgPool,
}

impl SqlxChatRepository {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    fn status_from_db(
        value: &str,
    ) -> Result<chat::MessageStatus> {
        match value {
            "visible" => Ok(chat::MessageStatus::Visible),
            "pending" => Ok(chat::MessageStatus::Pending),
            "removed" => Ok(chat::MessageStatus::Removed),
            _ => Err(Error::Repo(format!(
                "unknown message status: {}",
                value
            ))),
        }
    }

    fn status_to_db(
        status: chat::MessageStatus,
    ) -> &'static str {
        match status {
            chat::MessageStatus::Visible => "visible",
            chat::MessageStatus::Pending => "pending",
            chat::MessageStatus::Removed => "removed",
        }
    }
}

#[async_trait]
impl app::chat::Repository for SqlxChatRepository {
    async fn create_room(
        &self,
        room: &chat::Room,
    ) -> Result<()> {
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }

    async fn find_room(
        &self,
        room_id: &chat::RoomId,
    ) -> Result<Option<chat::Room>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, name, created_by FROM chat_rooms WHERE id = $1"
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        let Some(row) = record else {
            return Ok(None);
        };

        let name = row.get::<String, _>("name");
        let name = chat::RoomName::try_new(name)
            .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(Some(chat::Room {
            id: chat::RoomId::from_uuid(row.get::<uuid::Uuid, _>("id")),
            name,
            created_by: chat::UserId::from_uuid(
                row.get::<uuid::Uuid, _>("created_by"),
            ),
        }))
    }

    async fn find_room_by_name(
        &self,
        name: &chat::RoomName,
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        let Some(row) = record else {
            return Ok(None);
        };

        let name = row.get::<String, _>("name");
        let name = chat::RoomName::try_new(name)
            .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(Some(chat::Room {
            id: chat::RoomId::from_uuid(row.get::<uuid::Uuid, _>("id")),
            name,
            created_by: chat::UserId::from_uuid(
                row.get::<uuid::Uuid, _>("created_by"),
            ),
        }))
    }

    async fn list_messages(
        &self,
        room_id: &chat::RoomId,
        limit: usize,
    ) -> Result<Vec<chat::Message>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, room_id, user_id, body, status, client_id FROM chat_messages WHERE room_id = $1 ORDER BY created_at DESC LIMIT $2"
        );
        let rows = sqlx::query(
            r#"
            SELECT id, room_id, user_id, body, status, client_id
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        let mut messages = Vec::with_capacity(rows.len());
        for row in rows {
            let body = row.get::<String, _>("body");
            let body = chat::MessageBody::try_new(body)
                .map_err(|error| Error::Repo(error.to_string()))?;
            let status =
                Self::status_from_db(row.get::<String, _>("status").as_str())?;

            messages.push(chat::Message {
                id: chat::MessageId::from_uuid(
                    row.get::<uuid::Uuid, _>("id"),
                ),
                room_id: chat::RoomId::from_uuid(
                    row.get::<uuid::Uuid, _>("room_id"),
                ),
                user_id: chat::UserId::from_uuid(
                    row.get::<uuid::Uuid, _>("user_id"),
                ),
                body,
                status,
                client_id: row.get::<Option<String>, _>("client_id"),
            });
        }

        Ok(messages)
    }

    async fn find_message(
        &self,
        message_id: &chat::MessageId,
    ) -> Result<Option<chat::Message>> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT id, room_id, user_id, body, status, client_id FROM chat_messages WHERE id = $1"
        );
        let row = sqlx::query(
            r#"
            SELECT id, room_id, user_id, body, status, client_id
            FROM chat_messages
            WHERE id = $1
            "#,
        )
        .bind(message_id.as_uuid())
        .fetch_optional(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let body = row.get::<String, _>("body");
        let body = chat::MessageBody::try_new(body)
            .map_err(|error| Error::Repo(error.to_string()))?;
        let status =
            Self::status_from_db(row.get::<String, _>("status").as_str())?;

        Ok(Some(chat::Message {
            id: chat::MessageId::from_uuid(row.get::<uuid::Uuid, _>("id")),
            room_id: chat::RoomId::from_uuid(
                row.get::<uuid::Uuid, _>("room_id"),
            ),
            user_id: chat::UserId::from_uuid(
                row.get::<uuid::Uuid, _>("user_id"),
            ),
            body,
            status,
            client_id: row.get::<Option<String>, _>("client_id"),
        }))
    }

    async fn insert_message(
        &self,
        message: &chat::Message,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_messages (id, room_id, user_id, body, status, client_id) VALUES ($1, $2, $3, $4, $5, $6)"
        );
        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, room_id, user_id, body, status, client_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(message.id.as_uuid())
        .bind(message.room_id.as_uuid())
        .bind(message.user_id.as_uuid())
        .bind(message.body.to_string())
        .bind(Self::status_to_db(message.status))
        .bind(message.client_id.as_ref())
        .execute(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }

    async fn add_membership(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
        role: &str,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_room_memberships (room_id, user_id, role) VALUES ($1, $2, $3)"
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
        .bind(role)
        .execute(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }

    async fn is_member(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
    ) -> Result<bool> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "SELECT 1 FROM chat_room_memberships WHERE room_id = $1 AND user_id = $2"
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
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(row.is_some())
    }

    async fn update_message_status(
        &self,
        message_id: &chat::MessageId,
        status: chat::MessageStatus,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "UPDATE chat_messages SET status = $2 WHERE id = $1"
        );
        sqlx::query(
            r#"
            UPDATE chat_messages
            SET status = $2
            WHERE id = $1
            "#,
        )
        .bind(message_id.as_uuid())
        .bind(Self::status_to_db(status))
        .execute(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }
}

pub struct SqlxChatModerationQueue {
    pg: PgPool,
}

impl SqlxChatModerationQueue {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl app::chat::ModerationQueue for SqlxChatModerationQueue {
    async fn enqueue(
        &self,
        message_id: &chat::MessageId,
        reason: &str,
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
        .bind(reason)
        .execute(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }
}

pub struct SqlxChatRateLimiter {
    pg: PgPool,
}

impl SqlxChatRateLimiter {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl app::chat::RateLimiter for SqlxChatRateLimiter {
    async fn check(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
    ) -> Result<()> {
        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "UPSERT chat_rate_limits"
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
                        WHEN chat_rate_limits.count < $4
                            THEN chat_rate_limits.count + 1
                        ELSE chat_rate_limits.count
                    END
                RETURNING window_start, count
            )
            SELECT (window_start >= now() - ($3 || ' seconds')::interval AND count <= $4) AS allowed
            FROM updated
            "#,
        )
        .bind(room_id.as_uuid())
        .bind(user_id.as_uuid())
        .bind(RATE_LIMIT_WINDOW_SECS)
        .bind(RATE_LIMIT_MAX)
        .fetch_one(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        let allowed = row.get::<bool, _>("allowed");
        if allowed {
            Ok(())
        } else {
            Err(Error::RateLimited)
        }
    }
}

pub struct SqlxChatAuditLog {
    pg: PgPool,
}

impl SqlxChatAuditLog {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl app::chat::AuditLog for SqlxChatAuditLog {
    async fn record(
        &self,
        entry: AuditEntry,
    ) -> Result<()> {
        let metadata = entry
            .metadata
            .into_iter()
            .map(|(key, value)| (key, serde_json::Value::String(value)))
            .collect::<serde_json::Map<String, serde_json::Value>>();
        let metadata = serde_json::Value::Object(metadata);

        tracing::info!(
            target: "demo.db",
            message = "db query",
            db_statement = "INSERT INTO chat_audit_log (room_id, actor_user_id, action, metadata_json) VALUES ($1, $2, $3, $4)"
        );
        sqlx::query(
            r#"
            INSERT INTO chat_audit_log (room_id, actor_user_id, action, metadata_json)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(entry.room_id.as_uuid())
        .bind(entry.actor_id.as_uuid())
        .bind(entry.action)
        .bind(metadata)
        .execute(&self.pg)
        .await
        .map_err(|error| Error::Repo(error.to_string()))?;

        Ok(())
    }
}
