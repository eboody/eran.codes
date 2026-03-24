mod code_block;
mod icon;
mod key_value_list;
mod local_tab_panel;
mod local_tab_root;
mod pill;
mod tab;

pub use code_block::CodeBlock;
pub(crate) use icon::Icon;
pub use key_value_list::{KeyValueItem, KeyValueList, KeyValueListLayout, KeyValueValueAttr};
pub(crate) use local_tab_panel::LocalTabPanel;
pub(crate) use local_tab_root::{LocalTabRoot, LocalTabRootSurface};
pub use pill::{BadgeKind, Pill};
pub(crate) use tab::{Tab, TabInteraction};

pub(crate) fn head_styles() -> maud::Markup {
    maud::html! {
        (pill::head_styles())
        (icon::head_styles())
    }
}
