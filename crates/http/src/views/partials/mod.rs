pub mod components;
mod demo;
mod error;

// ci: descriptive-module-import crate::views::partials::chat
pub use demo::chat;
pub use demo::{
    AuthStatus, BoundaryCheck, CapabilityShowcase, CtaRow, DbCheck, DemoResultPlaceholder,
    HomeHero, KeyValueList, LiveLog, ModerationAction, NetworkLog, Ping,
    ProfessionalismInPracticeTabs, RequestBurstDemo, RequestMeta, SectionHeader,
    SessionStatus, StatusCard, TabsPanelShowcase, TraceLog,
};
pub use error::Error;
