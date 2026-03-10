mod create_room_flow;
mod error;
mod join_room_flow;
mod list_messages_flow;
mod moderate_message_flow;
mod post_message_flow;

use std::sync::Arc;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use bon::{Builder, bon};
use nutype::nutype;
use strum_macros::{Display, EnumString};

use domain::chat;
pub use error::{Error, InvalidIdText, InvalidInputText, RepoErrorText, Result};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum ModerationDecision {
    #[strum(serialize = "approve")]
    Approve,
    #[strum(serialize = "remove")]
    Remove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PendingMutationResult {
    Applied,
    NotPendingOrMissing,
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

impl ModerationReason {
    pub fn auto() -> Self {
        Self::try_new("auto").expect("valid static moderation reason")
    }
}

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

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct AuditValue(String);

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_room(&self, room: &chat::Room) -> Result<()>;
    async fn find_room(&self, room_id: &chat::RoomId) -> Result<Option<chat::Room>>;
    async fn find_room_by_name(&self, name: &chat::RoomName) -> Result<Option<chat::Room>>;
    async fn list_messages(
        &self,
        room_id: &chat::RoomId,
        limit: usize,
    ) -> Result<Vec<chat::Message>>;
    async fn find_message(
        &self,
        message_id: &chat::MessageId,
    ) -> Result<Option<chat::Message>>;
    async fn insert_message(&self, message: &chat::Message) -> Result<()>;
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
    ) -> Result<PendingMutationResult>;
}

#[async_trait]
pub trait ModerationQueue: Send + Sync {
    async fn enqueue(
        &self,
        message_id: &chat::MessageId,
        reason: &ModerationReason,
    ) -> Result<()>;
    async fn list_pending(&self, limit: usize) -> Result<Vec<ModerationItem>>;
    async fn complete_if_pending(
        &self,
        message_id: &chat::MessageId,
        reviewer_id: &chat::UserId,
        decision: ModerationDecision,
        reason: Option<ModerationReason>,
    ) -> Result<PendingMutationResult>;
}

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn check(&self, room_id: &chat::RoomId, user_id: &chat::UserId) -> Result<()>;
}

#[async_trait]
pub trait AuditLog: Send + Sync {
    async fn record(&self, entry: AuditEntry) -> Result<()>;
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

    #[tracing::instrument(skip(self))]
    pub async fn create_room(&self, command: CreateRoom) -> Result<chat::Room> {
        let incoming = create_room_flow::IncomingFlow::from_command(command);
        let materialized = incoming.materialize_room(self.ids.new_room_id());

        self.repo.create_room(materialized.room()).await?;
        let persisted = materialized.mark_room_persisted();
        self.repo
            .add_membership(
                &persisted.room_id(),
                &persisted.owner_id(),
                persisted.owner_role(),
            )
            .await?;
        let owner_added = persisted.mark_owner_membership_added();

        let room = owner_added.room();
        self.audit
            .record(self.audit_entry(
                room.id,
                room.created_by,
                AuditAction::RoomCreate,
                vec![(
                    AuditKey::RoomId,
                    AuditValue::new(room.id.as_uuid().to_string()),
                )],
            ))
            .await?;
        let audited = owner_added.mark_audited();

        Ok(audited.into_room())
    }

    #[tracing::instrument(skip(self))]
    pub async fn join_room(&self, command: JoinRoom) -> Result<()> {
        let incoming = join_room_flow::IncomingFlow::from_command(command);
        let room_exists = self.repo.find_room(&incoming.room_id()).await?.is_some();
        let room_verified = incoming.classify_room_lookup(room_exists).require_room()?;

        self.repo
            .add_membership(
                &room_verified.room_id(),
                &room_verified.user_id(),
                room_verified.role(),
            )
            .await?;
        let membership_added = room_verified.mark_membership_added();

        self.audit
            .record(self.audit_entry(
                membership_added.room_id(),
                membership_added.user_id(),
                AuditAction::RoomJoin,
                vec![(
                    AuditKey::Role,
                    AuditValue::new(membership_added.role().to_string()),
                )],
            ))
            .await?;
        let _ = membership_added.mark_audited();

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn list_messages(&self, command: ListMessages) -> Result<Vec<chat::Message>> {
        let incoming = list_messages_flow::IncomingFlow::from_command(command);
        let room_exists = self.repo.find_room(incoming.room_id()).await?.is_some();
        let room_verified = incoming.classify_room_lookup(room_exists).require_room()?;

        let is_member = self
            .repo
            .is_member(room_verified.room_id(), room_verified.user_id())
            .await?;
        let membership_verified = room_verified
            .classify_membership(is_member)
            .require_member()?;

        let messages = self
            .repo
            .list_messages(membership_verified.room_id(), membership_verified.limit())
            .await?;
        let loaded = membership_verified.attach_messages(messages);

        Ok(loaded.into_messages())
    }

    #[tracing::instrument(skip(self))]
    pub async fn list_moderation_queue(&self, limit: usize) -> Result<Vec<ModerationItem>> {
        self.moderation.list_pending(limit).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_room_by_name(
        &self,
        name: chat::RoomName,
    ) -> Result<Option<chat::Room>> {
        self.repo.find_room_by_name(&name).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn post_message(&self, command: PostMessage) -> Result<chat::Message> {
        let flow = post_message_flow::IncomingFlow::from_command(command);
        let requires_moderation = should_moderate(flow.body());

        let room_exists = self.repo.find_room(flow.room_id()).await?.is_some();
        let room_verified = flow.classify_room_lookup(room_exists).require_room()?;

        let is_member = self
            .repo
            .is_member(room_verified.room_id(), room_verified.user_id())
            .await?;
        let membership_verified = room_verified
            .classify_membership(is_member)
            .require_member()?;

        self.rate_limiter
            .check(membership_verified.room_id(), membership_verified.user_id())
            .await?;
        let rate_limited = membership_verified.rate_limit_passed();

        let built = rate_limited.build(
            self.ids.new_message_id(),
            self.clock.now(),
            requires_moderation,
        );
        let ready_for_audit = built
            .persist(self.repo.as_ref())
            .await?
            .enqueue_if_pending(self.moderation.as_ref())
            .await?;

        self.record_message_post_audit(ready_for_audit.message())
            .await?;
        let audited = ready_for_audit.mark_audited();
        Ok(audited.into_message())
    }

    #[tracing::instrument(skip(self))]
    pub async fn moderate_message(&self, command: ModerateMessage) -> Result<()> {
        let incoming = moderate_message_flow::IncomingFlow::from_command(command);
        let message_lookup = self.repo.find_message(&incoming.message_id()).await?;
        let loaded = incoming.load_lookup(message_lookup)?;
        let pending = loaded.classify_pending().require_pending()?;
        let resolved = pending.resolve();

        let message_update = self
            .repo
            .update_message_status(&resolved.message_id(), resolved.message_status())
            .await?;
        let message_status_applied = resolved
            .classify_message_status_update(message_update)
            .require_applied()?;

        let queue_update = self
            .moderation
            .complete_if_pending(
                &message_status_applied.message_id(),
                &message_status_applied.reviewer_id(),
                message_status_applied.decision(),
                message_status_applied.reason().cloned(),
            )
            .await?;
        let queue_completion_applied = message_status_applied
            .classify_queue_completion_update(queue_update)
            .require_applied()?;
        let audit_prepared = queue_completion_applied.prepare_audit();

        self.audit
            .record(self.audit_entry(
                audit_prepared.room_id(),
                audit_prepared.reviewer_id(),
                AuditAction::MessageModerate,
                audit_prepared.audit_metadata(),
            ))
            .await?;

        let _ = audit_prepared.mark_audited();
        Ok(())
    }
}

fn should_moderate(body: &chat::MessageBody) -> bool {
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
    async fn record_message_post_audit(&self, message: &chat::Message) -> Result<()> {
        self.audit
            .record(self.audit_entry(
                message.room_id,
                message.user_id,
                AuditAction::MessagePost,
                vec![
                    (
                        AuditKey::MessageId,
                        AuditValue::new(message.id.as_uuid().to_string()),
                    ),
                    (
                        AuditKey::Status,
                        AuditValue::new(message.status.to_string()),
                    ),
                ],
            ))
            .await
    }

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

        metadata.push((AuditKey::TimestampMs, AuditValue::new(timestamp)));

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
        #[builder(setters(name = with_moderation_queue))] moderation: Arc<
            dyn ModerationQueue,
        >,
        #[builder(setters(name = with_rate_limiter))] rate_limiter: Arc<dyn RateLimiter>,
        #[builder(setters(name = with_audit_log))] audit: Arc<dyn AuditLog>,
        #[builder(setters(name = with_clock))] clock: Arc<dyn Clock>,
        #[builder(setters(name = with_id_generator))] ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self::new(repo, moderation, rate_limiter, audit, clock, ids)
    }
}
