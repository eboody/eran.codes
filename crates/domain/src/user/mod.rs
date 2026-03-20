mod parse;

moddef::moddef!(mod { entity, error, repository });

pub use entity::User;
pub use error::{Error, Result};
use nutype::nutype;
pub use repository::Repository;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Username(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 254, predicate = parse::is_valid_email_address),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Email(String);

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

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn accepts_valid_email() {
        let email = Email::try_new("Demo.User@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().to_string(), "demo.user@example.com");
    }

    #[test]
    fn rejects_invalid_email() {
        assert!(Email::try_new("not-an-email").is_err());
        assert!(Email::try_new("missing-domain@").is_err());
        assert!(Email::try_new("@missing-local.com").is_err());
    }
}
