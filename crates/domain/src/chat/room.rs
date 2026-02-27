use bon::Builder;
use nutype::nutype;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum RoomName {
    #[strum(serialize = "Lobby")]
    Lobby,
    #[strum(serialize = "Demo")]
    Demo,
    #[strum(serialize = "Support")]
    Support,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct RoomNameText(String);

#[derive(Debug, Clone, PartialEq)]
pub enum RoomNameError {
    Invalid(RoomNameTextError),
    Unknown(RoomNameText),
}

impl std::fmt::Display for RoomNameError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            RoomNameError::Invalid(error) => {
                write!(f, "{error}")
            }
            RoomNameError::Unknown(value) => {
                write!(f, "{value}")
            }
        }
    }
}

impl RoomName {
    pub fn try_new(
        value: impl AsRef<str>,
    ) -> Result<Self, RoomNameError> {
        let raw = RoomNameText::try_new(value.as_ref())
            .map_err(RoomNameError::Invalid)?;
        raw.to_string()
            .parse()
            .map_err(|_| RoomNameError::Unknown(raw))
    }
}

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

#[cfg(test)]
mod tests {
    use super::{RoomName, RoomNameError};

    #[test]
    fn room_name_accepts_known_values() {
        assert_eq!(RoomName::try_new("Lobby"), Ok(RoomName::Lobby));
        assert_eq!(RoomName::try_new("Demo"), Ok(RoomName::Demo));
        assert_eq!(RoomName::try_new("Support"), Ok(RoomName::Support));
    }

    #[test]
    fn room_name_rejects_unknown_values() {
        assert!(matches!(
            RoomName::try_new("lobby"),
            Err(RoomNameError::Unknown(_))
        ));
        assert!(matches!(
            RoomName::try_new("random-room"),
            Err(RoomNameError::Unknown(_))
        ));
    }
}
