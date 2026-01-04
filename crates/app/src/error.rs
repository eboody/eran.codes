pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    User,
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
