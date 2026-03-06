pub mod components;
mod demo;
mod error;

// ci: descriptive-module-import crate::views::partials::chat
pub use demo::chat;
pub use demo::{
    AuthStatus, BoundaryCheck, CtaRow, DbCheck, DemoResultPlaceholder, EngineeringQuality,
    HomeHero, KeyValueList, EventStreamLog, ModerationAction, TransportLogSet, Ping, RequestBurstDemo,
    RequestMeta, SectionHeader, SessionStatus, StatusCard, TabSetShowcase, RequestTraceLog,
};
pub use error::Error;
