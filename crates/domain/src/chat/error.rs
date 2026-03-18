use snafu::prelude::*;

use crate::chat::{message, room};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid room name: {source}"))]
    RoomName { source: room::name::Error },
    #[snafu(display("invalid message body: {source}"))]
    MessageBody { source: message::BodyError },
}

impl From<room::name::Error> for Error {
    fn from(source: room::name::Error) -> Self {
        Self::RoomName { source }
    }
}

impl From<message::BodyError> for Error {
    fn from(source: message::BodyError) -> Self {
        Self::MessageBody { source }
    }
}
