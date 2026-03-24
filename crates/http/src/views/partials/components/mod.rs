mod composed;
pub(crate) mod logs;
pub(crate) mod primitives;

pub(crate) fn head_styles() -> maud::Markup {
    maud::html! {
        (composed::head_styles())
        (primitives::head_styles())
    }
}

pub use composed::button;
pub use composed::chat;
pub(crate) use composed::portfolio;
pub(crate) use composed::ResultCard;
pub(crate) use composed::tab_set;
pub use composed::button::Button;
pub use composed::{
    AuthShell, AuthShellVariant, DemoResultPlaceholder, NavAuth, NavBar, NavBrand, NavLink,
    NavLinkList, NavLinkListRole, NavSignedIn, SectionHeader, SectionHeaderDensity,
    SectionHeaderLevel,
    SectionHeaderMetaText, StatusCard, StatusCardItem,
};
pub use logs::primitives::EmptyState;
pub use primitives::{
    BadgeKind, CodeBlock, KeyValueItem, KeyValueList, KeyValueListLayout, KeyValueValueAttr, Pill,
};
pub(crate) use primitives::{LocalTabPanel, LocalTabRoot, LocalTabRootSurface, Tab, TabInteraction};
