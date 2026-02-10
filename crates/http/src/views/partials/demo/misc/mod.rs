moddef::moddef!(mod { moderation_action, pill, ping });

pub use moderation_action::ModerationAction;
pub use pill::{BadgeKind, LevelKind, MethodKind, Pill, PillColor, PillVariant, StatusKind};
pub use ping::Ping;
