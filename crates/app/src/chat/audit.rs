use async_trait::async_trait;
use bon::Builder;
use nutype::nutype;
use strum_macros::{Display, EnumString};

use super::{Result, chat};

#[derive(Clone, Debug, Builder)]
pub struct Entry {
    pub room_id: chat::room::Id,
    pub actor_id: chat::UserId,
    pub action: Action,
    pub metadata: Vec<(Key, Value)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
pub enum Action {
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
pub enum Key {
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
pub struct Value(String);

#[async_trait]
pub trait Log: Send + Sync {
    async fn record(&self, entry: Entry) -> Result<()>;
}
