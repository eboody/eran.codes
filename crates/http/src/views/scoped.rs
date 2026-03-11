use maud::{Markup, PreEscaped, html};

pub fn style(css: &'static str) -> Markup {
    html! {
        style { (PreEscaped(css)) }
    }
}

macro_rules! inline_css {
    ($css:expr $(,)?) => {
        fn css() -> maud::Markup {
            crate::views::scoped::style($css)
        }
    };
}

pub(crate) use inline_css;
