use snafu::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    User { source: crate::user::Error },
    #[snafu(display("{source}"))]
    Chat { source: crate::chat::Error },
}

impl From<crate::user::Error> for Error {
    fn from(source: crate::user::Error) -> Self {
        Self::User { source }
    }
}

impl From<crate::chat::Error> for Error {
    fn from(source: crate::chat::Error) -> Self {
        Self::Chat { source }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use std::error::Error as _;

    #[test]
    fn user_errors_wrap_with_source() {
        let source = crate::user::Email::try_new("not-an-email")
            .expect_err("email should be rejected");
        let error = Error::from(crate::user::Error::from(source));

        assert!(error.to_string().starts_with("invalid email: "));
        assert!(error.source().is_some());
    }

    #[test]
    fn chat_errors_wrap_with_source() {
        let source = crate::chat::room::Name::try_new("lobby")
            .expect_err("room name should be rejected");
        let error = Error::from(crate::chat::Error::from(source));

        assert_eq!(error.to_string(), "invalid room name: lobby");
        assert!(error.source().is_some());
    }
}
