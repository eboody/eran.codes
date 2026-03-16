use bon::Builder;
use nutype::nutype;

use super::room;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 1000),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Body(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(value: uuid::Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString,
)]
pub enum Status {
    #[strum(serialize = "visible")]
    Visible,
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "removed")]
    Removed,
}

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct Message {
    pub id: Id,
    pub room_id: room::Id,
    pub user_id: room::UserId,
    pub body: Body,
    pub status: Status,
    pub client_id: Option<ClientId>,
    pub created_at: std::time::SystemTime,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 128),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct ClientId(String);
