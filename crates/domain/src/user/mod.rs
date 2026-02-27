moddef::moddef!(mod { error, repo, user });

pub use error::{Error, Result};
use nutype::nutype;
pub use repo::Repository;
pub use user::User;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Username(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 254, predicate = is_valid_email),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Email(String);

fn is_valid_email(value: &str) -> bool {
    let mut parts = value.split('@');
    let Some(local) = parts.next() else {
        return false;
    };
    let Some(domain) = parts.next() else {
        return false;
    };

    if parts.next().is_some() || local.is_empty() || domain.is_empty() {
        return false;
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    domain.contains('.')
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
