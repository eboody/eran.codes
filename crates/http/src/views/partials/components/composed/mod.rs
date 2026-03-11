mod auth_shell;
pub mod button;
mod demo_result_placeholder;
mod nav_bar;
pub mod portfolio;
mod section_header;
mod status_card;
pub(crate) mod tab_set;

pub(crate) fn head_styles() -> maud::Markup {
    button::head_styles()
}

pub use auth_shell::{AuthShell, AuthShellVariant};
pub use demo_result_placeholder::DemoResultPlaceholder;
pub use nav_bar::{
    NavAuth, NavBar, NavBrand, NavLink, NavLinkList, NavLinkListRole, NavSignedIn,
};
pub use section_header::{SectionHeader, SectionHeaderMetaText};
pub use status_card::{StatusCard, StatusCardItem};
