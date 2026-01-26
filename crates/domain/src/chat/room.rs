use bon::Builder;
use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Display)
)]
pub struct RoomName(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(uuid::Uuid);

impl RoomId {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(uuid::Uuid);

impl UserId {
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

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct Room {
    pub id: RoomId,
    pub name: RoomName,
    pub created_by: UserId,
}
