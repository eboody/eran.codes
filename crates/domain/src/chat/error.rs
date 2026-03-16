use snafu::prelude::*;

use crate::chat::{message, room};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid room name: {source}"))]
    RoomName { source: room::NameError },
    #[snafu(display("invalid message body: {source}"))]
    MessageBody { source: message::BodyError },
}

impl From<room::NameError> for Error {
    fn from(source: room::NameError) -> Self {
        Self::RoomName { source }
    }
}

impl From<message::BodyError> for Error {
    fn from(source: message::BodyError) -> Self {
        Self::MessageBody { source }
    }
}
