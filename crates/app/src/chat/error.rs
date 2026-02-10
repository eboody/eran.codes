pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Domain(domain::chat::Error),
    Repo(RepoErrorText),
    InvalidId(InvalidIdText),
    RateLimited,
    RoomNotFound,
    MessageNotFound,
    NotMember,
}

impl From<domain::chat::Error> for Error {
    fn from(error: domain::chat::Error) -> Self {
        Self::Domain(error)
    }
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::error::Error for Error {}

use nutype::nutype;

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct RepoErrorText(String);

impl From<String> for RepoErrorText {
    fn from(value: String) -> Self {
        RepoErrorText::new(value)
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Display)
)]
pub struct InvalidIdText(String);

impl From<String> for InvalidIdText {
    fn from(value: String) -> Self {
        InvalidIdText::new(value)
    }
}
