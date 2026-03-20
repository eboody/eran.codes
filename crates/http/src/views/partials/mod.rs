pub mod components;
mod demo;
mod error;

// ci: descriptive-module-import crate::views::partials::chat
pub use components::button;
pub use components::{
    AuthShell, AuthShellVariant, DemoResultPlaceholder, SectionHeader, SectionHeaderMetaText,
    StatusCard, StatusCardItem,
};
#[allow(unused_imports)]
pub use components::Button;
pub use demo::chat;
pub use demo::{
    AuthStatus, BoundaryCheck, DbCheck, EngineeringQuality, HomeHero,
    OperationalRequestFilter, Ping, RequestBurstDemo, RequestMeta,
    RequestTraceLog, SessionStatus, TabSetShowcase, TransportLogSet,
};
pub use error::Error;
