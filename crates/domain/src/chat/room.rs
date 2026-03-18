pub mod name;

use bon::Builder;
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

impl Name {
    pub fn try_new(value: impl AsRef<str>) -> Result<Self, name::Error> {
        let raw = name::Text::try_new(value.as_ref())
            .map_err(|source| name::Error::Invalid { source })?;
        raw.to_string()
            .parse()
            .map_err(|_| name::Error::Unknown { value: raw })
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
    use super::{Name, name};

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
            Err(name::Error::Unknown { .. })
        ));
        assert!(matches!(
            Name::try_new("random-room"),
            Err(name::Error::Unknown { .. })
        ));
    }
}
