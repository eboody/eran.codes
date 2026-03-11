use snafu::prelude::*;

use crate::chat::{MessageBodyError, RoomNameError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid room name: {source}"))]
    RoomName {
        source: RoomNameError,
    },
    #[snafu(display("invalid message body: {source}"))]
    MessageBody {
        source: MessageBodyError,
    },
}

impl From<RoomNameError> for Error {
    fn from(source: RoomNameError) -> Self {
        Self::RoomName {
            source,
        }
    }
}

impl From<MessageBodyError> for Error {
    fn from(source: MessageBodyError) -> Self {
        Self::MessageBody {
            source,
        }
    }
}
