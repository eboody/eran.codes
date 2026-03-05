use maud::Render;
use maud_extensions::inline_css;

use crate::types::Text;
use crate::views::proper_theme::Palette;

use super::primitives::{Icon, IconGlyph};

#[derive(Clone, Debug)]
pub(crate) struct Tab {
    pub id: Text,
    pub controls: Text,
    pub palette: &'static Palette,
    pub is_selected: bool,
    pub icon_glyph: Option<IconGlyph>,
    pub text: Text,
}

impl Render for Tab {
    fn render(&self) -> maud::Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };
        let foreground_color = if self.is_selected {
            self.palette.lighter.clone()
        } else {
            self.palette.main.clone()
        };

        let icon = self.icon_glyph.as_ref().map(|glyph| Icon {
            glyph: glyph.clone(),
            color: foreground_color.clone(),
        });

        maud::html! {
            button.showcase-tab.is-selected[self.is_selected]
                type="button"
                role="tab"
                id=(&self.id)
                aria-controls=(&self.controls)
                aria-selected=(self.is_selected)
                tabindex=(tab_index)
                style={
                    "--tab-accent: " (&self.palette.main) "; "
                    "--tab-fg: " (&foreground_color) ";"
                }
            {
                (css())
                @if let Some(icon) = &icon {
                    span class="showcase-tab-icon" { (icon) }
                }
                span class="showcase-tab-label" { (&self.text) }
            }
        }
    }
}

inline_css! {
    me {
      display: var(--control-inline-display);
      align-items: var(--control-inline-align-items);
      gap: var(--control-gap);
      margin: var(--control-margin);
      border: var(--control-border-width) solid var(--control-border-color-default);
      border-radius: var(--control-radius);
      padding: var(--control-padding-block) var(--control-padding-inline);
      background: transparent;
      color: var(--tab-fg, var(--tab-accent));
      cursor: pointer;
      font: var(--control-font);
      white-space: var(--control-white-space);
    }

    me.is-selected {
      background: color-mix(in srgb, var(--tab-accent) 14%, transparent);
      border-color: color-mix(in srgb, var(--tab-accent) 55%, transparent);
      box-shadow: inset 0 0 0 var(--border-size-2)
        color-mix(in srgb, var(--tab-accent) 35%, transparent);
    }

    me > .showcase-tab-icon {
      --control-icon-size: var(--size-4);

      display: var(--control-inline-display);
      align-items: var(--control-inline-align-items);
    }
}
