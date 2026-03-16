use bon::Builder;
use nutype::nutype;
use snafu::prelude::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum Name {
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
pub struct NameText(String);

#[derive(Debug, Clone, PartialEq, Snafu)]
pub enum NameError {
    #[snafu(display("{source}"))]
    Invalid { source: NameTextError },
    #[snafu(display("{value}"))]
    Unknown { value: NameText },
}

impl Name {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self, NameError> {
        let raw = NameText::try_new(value.as_ref())
            .map_err(|source| NameError::Invalid { source })?;
        raw.to_string()
            .parse()
            .map_err(|_| NameError::Unknown { value: raw })
    }
}

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
    pub id: Id,
    pub name: Name,
    pub created_by: UserId,
}

#[cfg(test)]
mod tests {
    use super::{Name, NameError};

    #[test]
    fn room_name_accepts_known_values() {
        assert_eq!(Name::try_new("Lobby"), Ok(Name::Lobby));
        assert_eq!(Name::try_new("Demo"), Ok(Name::Demo));
        assert_eq!(Name::try_new("Support"), Ok(Name::Support));
    }

    #[test]
    fn room_name_rejects_unknown_values() {
        assert!(matches!(
            Name::try_new("lobby"),
            Err(NameError::Unknown { .. })
        ));
        assert!(matches!(
            Name::try_new("random-room"),
            Err(NameError::Unknown { .. })
        ));
    }
}
