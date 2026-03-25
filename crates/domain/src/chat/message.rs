use bon::Builder;
use nutype::nutype;

use super::{client, room};

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
}

impl From<uuid::Uuid> for Id {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for uuid::Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl AsRef<uuid::Uuid> for Id {
    fn as_ref(&self) -> &uuid::Uuid {
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
    pub client_id: Option<client::Id>,
    pub created_at: std::time::SystemTime,
}
