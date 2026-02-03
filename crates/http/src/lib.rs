mod auth;
pub mod chat_demo;
mod error;
mod handlers;
pub mod request;
pub mod sse;
mod trace;
pub mod trace_log;
mod views;
mod router;
mod state;
pub mod paths;

pub use error::{Error, Result};
pub use router::router;
pub use sse::Registry as SseRegistry;
pub use state::{DemoState, State};
pub use paths::Route;
