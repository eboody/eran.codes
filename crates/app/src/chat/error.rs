pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Domain(domain::chat::Error),
    Repo(String),
    InvalidId(String),
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
