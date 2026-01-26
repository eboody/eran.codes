moddef::moddef!(mod { error, message, room });

pub use error::{Error, Result};
pub use message::{
    Message, MessageBody, MessageBodyError, MessageId, MessageStatus,
};
pub use room::{Room, RoomId, RoomName, RoomNameError, UserId};
