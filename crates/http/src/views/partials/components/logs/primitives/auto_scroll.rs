use bon::Builder;
use maud::Render;

use crate::types::Text;

const AUTO_SCROLL_ASSET_URL: &str =
    "/static/log-auto-scroll.js?v=20260328-runtime-ownership";

#[derive(Clone, Copy, Debug, Default)]
pub enum Scope {
    Single,
    #[default]
    All,
}

#[derive(Clone, Debug, Builder)]
pub struct AutoScroll {
    pub root_id: Text,
    pub selector: Text,
    #[builder(default)]
    pub scope: Scope,
}

impl Render for AutoScroll {
    fn render(&self) -> maud::Markup {
        let scope = match self.scope {
            Scope::Single => "single",
            Scope::All => "all",
        };
        maud::html! {
            script
                src=(AUTO_SCROLL_ASSET_URL)
                data-auto-scroll-root-id=(&self.root_id)
                data-auto-scroll-selector=(&self.selector)
                data-auto-scroll-scope=(scope) {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_repo_owned_asset_with_scope_and_selectors() {
        let markup = AutoScroll::builder()
            .root_id(Text::from("network-log-target"))
            .selector(Text::from("[data-log-scroll]"))
            .scope(Scope::Single)
            .build()
            .render()
            .into_string();

        assert!(markup.contains("/static/log-auto-scroll.js"));
        assert!(markup.contains("data-auto-scroll-root-id=\"network-log-target\""));
        assert!(markup.contains("data-auto-scroll-selector=\"[data-log-scroll]\""));
        assert!(markup.contains("data-auto-scroll-scope=\"single\""));
        assert!(!markup.contains("MutationObserver"));
    }

    #[test]
    fn escapes_selector_for_html_attribute_output() {
        let markup = AutoScroll::builder()
            .root_id(Text::from("live-log-target"))
            .selector(Text::from("button[data-bind=\"chatDraftBody\"]"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains(
            "data-auto-scroll-selector=\"button[data-bind=&quot;chatDraftBody&quot;]\""
        ));
        assert!(markup.contains("data-auto-scroll-scope=\"all\""));
    }
}
