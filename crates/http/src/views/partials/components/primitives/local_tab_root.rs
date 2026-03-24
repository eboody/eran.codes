use maud::{Markup, Render};

use crate::types::Text;

pub(crate) enum LocalTabRootSurface<'a> {
    Standard { id: &'a str, class: &'a str },
    PortfolioCrateSwitcher,
}

impl<'a> LocalTabRootSurface<'a> {
    pub(crate) fn standard(id: &'a str, class: &'a str) -> Self {
        Self::Standard { id, class }
    }

    pub(crate) fn portfolio_crate_switcher() -> Self {
        Self::PortfolioCrateSwitcher
    }
}

pub(crate) struct LocalTabRoot<'a> {
    pub surface: LocalTabRootSurface<'a>,
    pub active_tab_id: Text,
    pub content: Markup,
}

impl Render for LocalTabRoot<'_> {
    fn render(&self) -> Markup {
        match &self.surface {
            LocalTabRootSurface::Standard { id, class } => maud::html! {
                section
                    id=(id)
                    class=(class)
                    data-local-tabs-root
                    data-local-tabs-active=(&self.active_tab_id) {
                    (&self.content)
                }
            },
            LocalTabRootSurface::PortfolioCrateSwitcher => maud::html! {
                section
                    data-portfolio-crate-switcher
                    data-local-tabs-root
                    data-local-tabs-active=(&self.active_tab_id) {
                    (&self.content)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_standard_local_tab_root_contract() {
        let markup = LocalTabRoot {
            surface: LocalTabRootSurface::standard("root-id", "root-class"),
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

    #[test]
    fn renders_portfolio_switcher_marker_when_requested() {
        let markup = LocalTabRoot {
            surface: LocalTabRootSurface::portfolio_crate_switcher(),
            active_tab_id: Text::from("crate_0"),
            content: maud::html! {
                p { "Example" }
            },
        }
        .render()
        .into_string();

        assert!(markup.contains("data-portfolio-crate-switcher"));
        assert!(markup.contains("data-local-tabs-root"));
    }
}
