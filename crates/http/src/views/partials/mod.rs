pub mod components;
mod demo;
mod error;

// ci: descriptive-module-import crate::views::partials::chat
pub use demo::chat;
pub use demo::{
    AuthStatus, BoundaryCheck, CtaRow, DbCheck, DemoResultPlaceholder, EngineeringQuality,
    HomeHero, KeyValueList, LiveLog, ModerationAction, NetworkLog, Ping, RequestBurstDemo,
    RequestMeta, SectionHeader, SessionStatus, StatusCard, TabSetShowcase, TraceLog,
};
pub use error::Error;
