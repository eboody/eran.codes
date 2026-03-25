mod code_block;
mod icon;
mod key_value_list;
mod local_tab_panel;
mod local_tab_root;
mod pill;
mod tab;

pub use code_block::{CodeBlock, CodeLineMode};
pub(crate) use icon::Icon;
pub use key_value_list::{KeyValueItem, KeyValueList, KeyValueValueAttr, Layout as KeyValueListLayout};
pub(crate) use local_tab_panel::LocalTabPanel;
pub(crate) use local_tab_root::{LocalTabRoot, Surface as LocalTabRootSurface};
pub use pill::{BadgeKind, Pill};
pub(crate) use tab::{Interaction as TabInteraction, Tab};

pub(crate) fn head_styles() -> maud::Markup {
    maud::html! {
        (pill::head_styles())
        (icon::head_styles())
    }
}
