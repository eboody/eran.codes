use async_trait::async_trait;
use bon::Builder;
use nutype::nutype;
use strum_macros::{Display, EnumString};

use super::{Result, TimestampText, chat};

#[derive(Clone, Debug, Builder)]
pub struct Item {
    pub message_id: chat::message::Id,
    pub room_id: chat::room::Id,
    pub room_name: chat::room::Name,
    pub user_id: chat::UserId,
    pub body: chat::message::Body,
    pub queue_status: QueueStatus,
    pub reason: Reason,
    pub created_at: TimestampText,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum Decision {
    #[strum(serialize = "approve")]
    Approve,
    #[strum(serialize = "remove")]
    Remove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum QueueStatus {
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
pub struct Reason(String);

impl Reason {
    pub fn auto() -> Self {
        Self::try_new("auto").expect("valid static moderation reason")
    }
}

#[async_trait]
pub trait Queue: Send + Sync {
    async fn enqueue(&self, message_id: &chat::message::Id, reason: &Reason) -> Result<()>;
    async fn list_pending(&self, limit: usize) -> Result<Vec<Item>>;
    async fn complete_if_pending(
        &self,
        message_id: &chat::message::Id,
        reviewer_id: &chat::UserId,
        decision: Decision,
        reason: Option<Reason>,
    ) -> Result<super::PendingMutationResult>;
}
