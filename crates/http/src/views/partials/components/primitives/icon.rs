use maud::{PreEscaped, Render};
use maud_extensions::inline_css;

use crate::views::proper_theme::ThemeColor;

pub(crate) type IconGlyph = PreEscaped<&'static str>;

#[derive(Clone, Debug)]
pub(crate) struct Icon {
    pub glyph: IconGlyph,
    pub color: ThemeColor,
}

impl Render for Icon {
    fn render(&self) -> maud::Markup {
        maud::html! {
            span aria-hidden="true" style={ "--icon-color: " (&self.color) ";" } {
                (css())
                (self.glyph)
            }
        }
    }
}

inline_css! {
    me {
      display: var(--control-inline-display);
      align-items: var(--control-inline-align-items);
      font-size: var(--control-icon-size);
      color: var(--icon-color, currentColor);
    }
}
