mod auth_shell;
pub mod button;
pub mod chat;
mod demo_result_placeholder;
mod nav_bar;
pub mod portfolio;
mod result_card;
mod section_header;
mod status_card;
pub(crate) mod tab_set;

pub(crate) fn head_styles() -> maud::Markup {
    button::head_styles()
}

pub use auth_shell::{AuthShell, Variant as AuthShellVariant};
pub use demo_result_placeholder::DemoResultPlaceholder;
pub use nav_bar::{
    NavAuth, NavBar, NavBrand, NavGuestAuth, NavGuestSwitch, NavLink, NavLinkList,
    NavLinkListRole, NavSignedIn,
};
pub(crate) use result_card::ResultCard;
pub use section_header::{
    Density as SectionHeaderDensity, MetaText as SectionHeaderMetaText, SectionHeader,
    SectionHeaderLevel,
};
pub use status_card::{Item as StatusCardItem, StatusCard};
