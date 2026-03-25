pub mod client;
#[path = "error.rs"]
mod failure;
pub mod message;
pub mod room;

pub use client::Id as Client;
pub use failure::{Error, Result};
pub use message::Message;
pub use room::{Room, UserId};
