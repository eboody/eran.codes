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
pub(crate) use composed::portfolio;
pub(crate) use composed::tab_set;
pub use composed::{
    AuthShell, AuthShellVariant, DemoResultPlaceholder, NavAuth, NavBar, NavBrand, NavLink,
    NavLinkList, NavLinkListRole, NavSignedIn, SectionHeader, SectionHeaderMetaText,
    StatusCard, StatusCardItem,
};
pub use logs::primitives::EmptyState;
pub use primitives::{BadgeKind, KeyValueList, Pill};
pub(crate) use primitives::{Tab, TabInteraction};
