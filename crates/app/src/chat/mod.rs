mod error;

use std::sync::Arc;

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
    async fn list_messages(
        &self,
        room_id: &chat::RoomId,
        limit: usize,
    ) -> Result<Vec<chat::Message>>;
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
