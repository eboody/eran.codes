mod error;

use std::sync::Arc;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use bon::{bon, Builder};
use nutype::nutype;
use strum_macros::{Display, EnumString};

use domain::chat;
pub use error::{Error, InvalidIdText, RepoErrorText, Result};

#[derive(Clone, Debug, Builder)]
pub struct PostMessage {
    pub room_id: chat::RoomId,
    pub user_id: chat::UserId,
    pub body: chat::MessageBody,
    pub client_id: Option<chat::ClientId>,
}

#[derive(Clone, Debug, Builder)]
pub struct ListMessages {
    pub room_id: chat::RoomId,
    pub user_id: chat::UserId,
    #[builder(default = 50)]
    pub limit: usize,
}

#[derive(Clone, Debug, Builder)]
pub struct CreateRoom {
    pub name: chat::RoomName,
    pub created_by: chat::UserId,
}

#[derive(Clone, Debug, Builder)]
pub struct JoinRoom {
    pub room_id: chat::RoomId,
    pub user_id: chat::UserId,
    #[builder(default = RoomRole::Member)]
    pub role: RoomRole,
}

#[derive(Clone, Debug, Builder)]
pub struct ModerateMessage {
    pub message_id: chat::MessageId,
    pub reviewer_id: chat::UserId,
    pub decision: ModerationDecision,
    pub reason: Option<ModerationReason>,
}

#[derive(Clone, Debug, Builder)]
pub struct ModerationItem {
    pub message_id: chat::MessageId,
    pub room_id: chat::RoomId,
    pub room_name: chat::RoomName,
    pub user_id: chat::UserId,
    pub body: chat::MessageBody,
    pub queue_status: ModerationQueueStatus,
    pub reason: ModerationReason,
    pub created_at: TimestampText,
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
    pub action: AuditAction,
    pub metadata: Vec<(AuditKey, AuditValue)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum RoomRole {
    #[strum(serialize = "member")]
    Member,
    #[strum(serialize = "owner")]
    Owner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum ModerationQueueStatus {
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "approved")]
    Approved,
    #[strum(serialize = "removed")]
    Removed,
}

#[nutype(
    sanitize(trim),
    validate(len_char_max = 200),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct ModerationReason(String);

#[nutype(
    sanitize(trim),
    validate(len_char_max = 32),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct TimestampText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum AuditAction {
    #[strum(serialize = "chat.room.create")]
    RoomCreate,
    #[strum(serialize = "chat.room.join")]
    RoomJoin,
    #[strum(serialize = "chat.message.post")]
    MessagePost,
    #[strum(serialize = "chat.message.moderate")]
    MessageModerate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum AuditKey {
    #[strum(serialize = "room_id")]
    RoomId,
    #[strum(serialize = "message_id")]
    MessageId,
    #[strum(serialize = "status")]
    Status,
    #[strum(serialize = "decision")]
    Decision,
    #[strum(serialize = "reason")]
    Reason,
    #[strum(serialize = "timestamp_ms")]
    TimestampMs,
    #[strum(serialize = "role")]
    Role,
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct AuditValue(String);

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
        role: RoomRole,
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
        reason: &ModerationReason,
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
        reason: Option<ModerationReason>,
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
        let room = chat::Room {
            id: self.ids.new_room_id(),
            name: command.name,
            created_by: command.created_by,
        };

        self.repo.create_room(&room).await?;
        self.repo
            .add_membership(&room.id, &room.created_by, RoomRole::Owner)
            .await?;
        self.audit
            .record(self.audit_entry(
                room.id,
                room.created_by,
                AuditAction::RoomCreate,
                vec![(
                    AuditKey::RoomId,
                    AuditValue::new(room.id.as_uuid().to_string())
                        ,
                )],
            ))
            .await?;

        Ok(room)
    }

    pub async fn join_room(
        &self,
        command: JoinRoom,
    ) -> Result<()> {
        let Some(_) = self.repo.find_room(&command.room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        self.repo
            .add_membership(&command.room_id, &command.user_id, command.role)
            .await?;
        self.audit
            .record(self.audit_entry(
                command.room_id,
                command.user_id,
                AuditAction::RoomJoin,
                vec![(
                    AuditKey::Role,
                    AuditValue::new(command.role.to_string())
                        ,
                )],
            ))
            .await?;

        Ok(())
    }

    pub async fn list_messages(
        &self,
        command: ListMessages,
    ) -> Result<Vec<chat::Message>> {
        let Some(_) = self.repo.find_room(&command.room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        let is_member = self
            .repo
            .is_member(&command.room_id, &command.user_id)
            .await?;
        if !is_member {
            return Err(Error::NotMember);
        }

        self.repo
            .list_messages(&command.room_id, command.limit)
            .await
    }

    pub async fn list_moderation_queue(
        &self,
        limit: usize,
    ) -> Result<Vec<ModerationItem>> {
        self.moderation.list_pending(limit).await
    }

    pub async fn find_room_by_name(
        &self,
        name: chat::RoomName,
    ) -> Result<Option<chat::Room>> {
        self.repo.find_room_by_name(&name).await
    }

    pub async fn post_message(
        &self,
        command: PostMessage,
    ) -> Result<chat::Message> {
        let Some(_) = self.repo.find_room(&command.room_id).await? else {
            return Err(Error::RoomNotFound);
        };

        let is_member = self
            .repo
            .is_member(&command.room_id, &command.user_id)
            .await?;
        if !is_member {
            return Err(Error::NotMember);
        }

        self.rate_limiter
            .check(&command.room_id, &command.user_id)
            .await?;

        let requires_moderation = should_moderate(&command.body);
        let status = if requires_moderation {
            chat::MessageStatus::Pending
        } else {
            chat::MessageStatus::Visible
        };

        let message = chat::Message {
            id: self.ids.new_message_id(),
            room_id: command.room_id,
            user_id: command.user_id,
            body: command.body,
            status,
            client_id: command.client_id,
            created_at: self.clock.now(),
        };

        self.repo.insert_message(&message).await?;

        if requires_moderation {
            self.moderation
                .enqueue(
                    &message.id,
                    &ModerationReason::try_new("auto")
                        .expect("moderation reason"),
                )
                .await?;
        }

        self.audit
            .record(self.audit_entry(
                message.room_id,
                message.user_id,
                AuditAction::MessagePost,
                vec![
                    (
                        AuditKey::MessageId,
                        AuditValue::new(
                            message.id.as_uuid().to_string(),
                        )
                        ,
                    ),
                    (
                        AuditKey::Status,
                        AuditValue::new(format!("{:?}", status))
                            ,
                    ),
                ],
            ))
            .await?;

        Ok(message)
    }

    pub async fn moderate_message(
        &self,
        command: ModerateMessage,
    ) -> Result<()> {
        let Some(message) =
            self.repo.find_message(&command.message_id).await?
        else {
            return Err(Error::MessageNotFound);
        };

        let status = match command.decision {
            ModerationDecision::Approve => chat::MessageStatus::Visible,
            ModerationDecision::Remove => chat::MessageStatus::Removed,
        };

        self.repo
            .update_message_status(&command.message_id, status)
            .await?;

        self.moderation
            .complete(
                &command.message_id,
                &command.reviewer_id,
                command.decision,
                command.reason.clone(),
            )
            .await?;

        self.audit
            .record(self.audit_entry(
                message.room_id,
                command.reviewer_id,
                AuditAction::MessageModerate,
                vec![
                    (
                        AuditKey::MessageId,
                        AuditValue::new(
                            message.id.as_uuid().to_string(),
                        )
                        ,
                    ),
                    (
                        AuditKey::Decision,
                        AuditValue::new(format!(
                            "{:?}",
                            command.decision
                        ))
                        ,
                    ),
                    (
                        AuditKey::Reason,
                        command
                            .reason
                            .clone()
                            .map(|reason| {
                                AuditValue::new(reason.to_string())
                                    
                            })
                            .unwrap_or_else(|| {
                                AuditValue::new("")
                                    
                            }),
                    ),
                ],
            ))
            .await?;

        Ok(())
    }
}

fn should_moderate(
    body: &chat::MessageBody,
) -> bool {
    let value = body.to_string();
    value.len() > 300 || LinkPrefix::is_present(&value)
}

#[derive(Clone, Copy, Debug)]
enum LinkPrefix {
    Http,
    Https,
}

impl LinkPrefix {
    fn as_str(self) -> &'static str {
        match self {
            LinkPrefix::Http => "http://",
            LinkPrefix::Https => "https://",
        }
    }

    fn is_present(value: &str) -> bool {
        [Self::Http, Self::Https]
            .iter()
            .any(|prefix| value.contains(prefix.as_str()))
    }
}

impl Service {
    fn audit_entry(
        &self,
        room_id: chat::RoomId,
        actor_id: chat::UserId,
        action: AuditAction,
        mut metadata: Vec<(AuditKey, AuditValue)>,
    ) -> AuditEntry {
        let timestamp = self
            .clock
            .now()
            .duration_since(UNIX_EPOCH)
            .map(|value| value.as_millis().to_string())
            .unwrap_or_else(|_| "0".to_string());

        metadata.push((
            AuditKey::TimestampMs,
            AuditValue::new(timestamp),
        ));

        AuditEntry::builder()
            .room_id(room_id)
            .actor_id(actor_id)
            .action(action)
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
