mod error;

use std::sync::Arc;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use bon::{bon, Builder};

use domain::chat;
pub use error::{Error, Result};

#[derive(Clone, Debug, Builder)]
pub struct PostMessage {
    pub room_id: String,
    pub user_id: String,
    pub body: String,
    pub client_id: Option<String>,
}

#[derive(Clone, Debug, Builder)]
pub struct ListMessages {
    pub room_id: String,
    pub user_id: String,
    #[builder(default = 50)]
    pub limit: usize,
}

#[derive(Clone, Debug, Builder)]
pub struct CreateRoom {
    pub name: String,
    pub created_by: String,
}

#[derive(Clone, Debug, Builder)]
pub struct JoinRoom {
    pub room_id: String,
    pub user_id: String,
    #[builder(default = "member".to_string())]
    pub role: String,
}

#[derive(Clone, Debug, Builder)]
pub struct ModerateMessage {
    pub message_id: String,
    pub reviewer_id: String,
    pub decision: ModerationDecision,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Builder)]
pub struct ModerationItem {
    pub message_id: chat::MessageId,
    pub room_id: chat::RoomId,
    pub room_name: chat::RoomName,
    pub user_id: chat::UserId,
    pub body: chat::MessageBody,
    pub queue_status: String,
    pub reason: String,
    pub created_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModerationDecision {
    Approve,
    Remove,
}

#[derive(Clone, Debug, Builder)]
pub struct AuditEntry {
    pub room_id: chat::RoomId,
    pub actor_id: chat::UserId,
    pub action: String,
    pub metadata: Vec<(String, String)>,
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_room(
        &self,
        room: &chat::Room,
    ) -> Result<()>;
    async fn find_room(
        &self,
        room_id: &chat::RoomId,
    ) -> Result<Option<chat::Room>>;
    async fn find_room_by_name(
        &self,
        name: &chat::RoomName,
    ) -> Result<Option<chat::Room>>;
    async fn list_messages(
        &self,
        room_id: &chat::RoomId,
        limit: usize,
    ) -> Result<Vec<chat::Message>>;
    async fn find_message(
        &self,
        message_id: &chat::MessageId,
    ) -> Result<Option<chat::Message>>;
    async fn insert_message(
        &self,
        message: &chat::Message,
    ) -> Result<()>;
    async fn add_membership(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
        role: &str,
    ) -> Result<()>;
    async fn is_member(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
    ) -> Result<bool>;
    async fn update_message_status(
        &self,
        message_id: &chat::MessageId,
        status: chat::MessageStatus,
    ) -> Result<()>;
}

#[async_trait]
pub trait ModerationQueue: Send + Sync {
    async fn enqueue(
        &self,
        message_id: &chat::MessageId,
        reason: &str,
    ) -> Result<()>;
    async fn list_pending(
        &self,
        limit: usize,
    ) -> Result<Vec<ModerationItem>>;
    async fn complete(
        &self,
        message_id: &chat::MessageId,
        reviewer_id: &chat::UserId,
        decision: ModerationDecision,
        reason: Option<String>,
    ) -> Result<()>;
}

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn check(
        &self,
        room_id: &chat::RoomId,
        user_id: &chat::UserId,
    ) -> Result<()>;
}

#[async_trait]
pub trait AuditLog: Send + Sync {
    async fn record(
        &self,
        entry: AuditEntry,
    ) -> Result<()>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> std::time::SystemTime;
}

pub trait IdGenerator: Send + Sync {
    fn new_room_id(&self) -> chat::RoomId;
    fn new_message_id(&self) -> chat::MessageId;
}

#[derive(Clone)]
pub struct Service {
    repo: Arc<dyn Repository>,
    moderation: Arc<dyn ModerationQueue>,
    rate_limiter: Arc<dyn RateLimiter>,
    audit: Arc<dyn AuditLog>,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
}

impl Service {
    pub fn new(
        repo: Arc<dyn Repository>,
        moderation: Arc<dyn ModerationQueue>,
        rate_limiter: Arc<dyn RateLimiter>,
        audit: Arc<dyn AuditLog>,
        clock: Arc<dyn Clock>,
        ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            repo,
            moderation,
            rate_limiter,
            audit,
            clock,
            ids,
        }
    }

    pub async fn create_room(
        &self,
        command: CreateRoom,
    ) -> Result<chat::Room> {
        let created_by = parse_user_id(&command.created_by)?;
        let name = chat::RoomName::try_new(command.name)
            .map_err(domain::chat::Error::from)?;

        let room = chat::Room {
            id: self.ids.new_room_id(),
            name,
            created_by,
        };

        self.repo.create_room(&room).await?;
        self.repo
            .add_membership(&room.id, &created_by, "owner")
            .await?;
        self.audit
            .record(self.audit_entry(
                room.id,
                created_by,
                "chat.room.create",
                vec![("room_id".to_string(), room.id.as_uuid().to_string())],
            ))
            .await?;

        Ok(room)
    }

    pub async fn join_room(
        &self,
        command: JoinRoom,
    ) -> Result<()> {
        let room_id = parse_room_id(&command.room_id)?;
        let user_id = parse_user_id(&command.user_id)?;

        let Some(_) = self.repo.find_room(&room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        self.repo
            .add_membership(&room_id, &user_id, &command.role)
            .await?;
        self.audit
            .record(self.audit_entry(
                room_id,
                user_id,
                "chat.room.join",
                vec![("role".to_string(), command.role)],
            ))
            .await?;

        Ok(())
    }

    pub async fn list_messages(
        &self,
        command: ListMessages,
    ) -> Result<Vec<chat::Message>> {
        let room_id = parse_room_id(&command.room_id)?;
        let user_id = parse_user_id(&command.user_id)?;

        let Some(_) = self.repo.find_room(&room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        let is_member = self.repo.is_member(&room_id, &user_id).await?;
        if !is_member {
            return Err(Error::NotMember);
        }

        self.repo.list_messages(&room_id, command.limit).await
    }

    pub async fn list_moderation_queue(
        &self,
        limit: usize,
    ) -> Result<Vec<ModerationItem>> {
        self.moderation.list_pending(limit).await
    }

    pub async fn find_room_by_name(
        &self,
        name: String,
    ) -> Result<Option<chat::Room>> {
        let name =
            chat::RoomName::try_new(name).map_err(domain::chat::Error::from)?;
        self.repo.find_room_by_name(&name).await
    }

    pub async fn post_message(
        &self,
        command: PostMessage,
    ) -> Result<chat::Message> {
        let room_id = parse_room_id(&command.room_id)?;
        let user_id = parse_user_id(&command.user_id)?;

        let Some(_) = self.repo.find_room(&room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        let is_member = self.repo.is_member(&room_id, &user_id).await?;
        if !is_member {
            return Err(Error::NotMember);
        }

        self.rate_limiter.check(&room_id, &user_id).await?;

        let body = chat::MessageBody::try_new(command.body)
            .map_err(domain::chat::Error::from)?;
        let requires_moderation = should_moderate(&body);
        let status = if requires_moderation {
            chat::MessageStatus::Pending
        } else {
            chat::MessageStatus::Visible
        };

        let message = chat::Message {
            id: self.ids.new_message_id(),
            room_id,
            user_id,
            body,
            status,
            client_id: command.client_id,
        };

        self.repo.insert_message(&message).await?;

        if requires_moderation {
            self.moderation
                .enqueue(&message.id, "auto")
                .await?;
        }

        self.audit
            .record(self.audit_entry(
                room_id,
                user_id,
                "chat.message.post",
                vec![
                    ("message_id".to_string(), message.id.as_uuid().to_string()),
                    ("status".to_string(), format!("{:?}", status)),
                ],
            ))
            .await?;

        Ok(message)
    }

    pub async fn moderate_message(
        &self,
        command: ModerateMessage,
    ) -> Result<()> {
        let message_id = parse_message_id(&command.message_id)?;
        let reviewer_id = parse_user_id(&command.reviewer_id)?;

        let Some(message) = self.repo.find_message(&message_id).await? else {
            return Err(Error::MessageNotFound);
        };

        let status = match command.decision {
            ModerationDecision::Approve => chat::MessageStatus::Visible,
            ModerationDecision::Remove => chat::MessageStatus::Removed,
        };

        self.repo
            .update_message_status(&message_id, status)
            .await?;

        self.moderation
            .complete(
                &message_id,
                &reviewer_id,
                command.decision,
                command.reason.clone(),
            )
            .await?;

        self.audit
            .record(self.audit_entry(
                message.room_id,
                reviewer_id,
                "chat.message.moderate",
                vec![
                    ("message_id".to_string(), message_id.as_uuid().to_string()),
                    ("decision".to_string(), format!("{:?}", command.decision)),
                    (
                        "reason".to_string(),
                        command.reason.unwrap_or_default(),
                    ),
                ],
            ))
            .await?;

        Ok(())
    }
}

fn parse_room_id(value: &str) -> Result<chat::RoomId> {
    let id = value
        .parse::<uuid::Uuid>()
        .map_err(|error| Error::InvalidId(error.to_string()))?;
    Ok(chat::RoomId::from_uuid(id))
}

fn parse_user_id(value: &str) -> Result<chat::UserId> {
    let id = value
        .parse::<uuid::Uuid>()
        .map_err(|error| Error::InvalidId(error.to_string()))?;
    Ok(chat::UserId::from_uuid(id))
}

fn parse_message_id(value: &str) -> Result<chat::MessageId> {
    let id = value
        .parse::<uuid::Uuid>()
        .map_err(|error| Error::InvalidId(error.to_string()))?;
    Ok(chat::MessageId::from_uuid(id))
}

fn should_moderate(
    body: &chat::MessageBody,
) -> bool {
    let value = body.to_string();
    value.len() > 300 || value.contains("http://") || value.contains("https://")
}

impl Service {
    fn audit_entry(
        &self,
        room_id: chat::RoomId,
        actor_id: chat::UserId,
        action: &str,
        mut metadata: Vec<(String, String)>,
    ) -> AuditEntry {
        let timestamp = self
            .clock
            .now()
            .duration_since(UNIX_EPOCH)
            .map(|value| value.as_millis().to_string())
            .unwrap_or_else(|_| "0".to_string());

        metadata.push(("timestamp_ms".to_string(), timestamp));

        AuditEntry::builder()
            .room_id(room_id)
            .actor_id(actor_id)
            .action(action.to_string())
            .metadata(metadata)
            .build()
    }
}

#[bon]
impl Service {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_repo))] repo: Arc<dyn Repository>,
        #[builder(setters(name = with_moderation_queue))]
        moderation: Arc<dyn ModerationQueue>,
        #[builder(setters(name = with_rate_limiter))]
        rate_limiter: Arc<dyn RateLimiter>,
        #[builder(setters(name = with_audit_log))] audit: Arc<dyn AuditLog>,
        #[builder(setters(name = with_clock))] clock: Arc<dyn Clock>,
        #[builder(setters(name = with_id_generator))] ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self::new(repo, moderation, rate_limiter, audit, clock, ids)
    }
}
