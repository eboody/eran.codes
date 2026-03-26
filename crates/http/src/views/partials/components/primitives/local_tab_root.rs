use maud::{Markup, Render};

use crate::types::Text;

pub(crate) struct Surface<'a> {
    id: &'a str,
    class: &'a str,
}

impl<'a> Surface<'a> {
    pub(crate) fn standard(id: &'a str, class: &'a str) -> Self {
        Self { id, class }
    }
}

pub(crate) struct LocalTabRoot<'a> {
    pub surface: Surface<'a>,
    pub active_tab_id: Text,
    pub content: Markup,
}

impl Render for LocalTabRoot<'_> {
    fn render(&self) -> Markup {
        maud::html! {
            section
                id=(self.surface.id)
                class=(self.surface.class)
                data-local-tabs-root
                data-local-tabs-active=(&self.active_tab_id) {
                (&self.content)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_standard_local_tab_root_contract() {
        let markup = LocalTabRoot {
            surface: Surface::standard("root-id", "root-class"),
            active_tab_id: Text::from("alpha"),
            content: maud::html! {
                p { "Example" }
            },
        }
        .render()
        .into_string();

        assert!(markup.contains("id=\"root-id\""));
        assert!(markup.contains("class=\"root-class\""));
        assert!(markup.contains("data-local-tabs-root"));
        assert!(markup.contains("data-local-tabs-active=\"alpha\""));
    }
}
