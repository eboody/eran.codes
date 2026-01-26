pub use crate::repo::chat::{
    AuditLog, ModerationQueue, RateLimiter, Repository,
};

use app::chat::{Clock, IdGenerator};
use domain::chat;

#[derive(Default)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Clock for SystemClock {
    fn now(&self) -> std::time::SystemTime {
        std::time::SystemTime::now()
    }
}

#[derive(Default)]
pub struct UuidGenerator;

impl UuidGenerator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdGenerator for UuidGenerator {
    fn new_room_id(&self) -> chat::RoomId {
        chat::RoomId::new_v4()
    }

    fn new_message_id(&self) -> chat::MessageId {
        chat::MessageId::new_v4()
    }
}
