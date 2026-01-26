use std::fmt;

use derive_more::From;

use crate::chat::{MessageBodyError, RoomNameError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    RoomName(RoomNameError),
    MessageBody(MessageBodyError),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::RoomName(error) => {
                write!(f, "invalid room name: {}", error)
            }
            Error::MessageBody(error) => {
                write!(f, "invalid message body: {}", error)
            }
        }
    }
}

impl std::error::Error for Error {}
