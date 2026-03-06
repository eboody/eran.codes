use maud::Render;
use maud_extensions::inline_css;

use crate::types::Text;
use crate::views::proper_theme::Palette;

use super::primitives::Icon;

#[derive(Clone, Debug)]
pub(crate) struct Tab {
    pub id: Text,
    pub controls: Text,
    pub palette: &'static Palette,
    pub is_selected: bool,
    pub icon: Option<Icon>,
    pub text: Text,
    pub interaction: TabInteraction,
}

#[derive(Clone, Debug)]
pub(crate) enum TabInteraction {
    PanelJs,
    DatastarLocal { signal: Text, value: Text },
}

impl Render for Tab {
    fn render(&self) -> maud::Markup {
        let tab_index = if self.is_selected { 0 } else { -1 };
        let foreground_color = if self.is_selected {
            self.palette.lighter.clone()
        } else {
            self.palette.main.clone()
        };

        let icon = self
            .icon
            .as_ref()
            .map(|icon| icon.clone().with_color(foreground_color.clone()));

        let style = format!(
            "--tab-accent: {}; --tab-fg: {};",
            self.palette.main.as_ref(),
            foreground_color.as_ref()
        );

        match &self.interaction {
            TabInteraction::PanelJs => {
                maud::html! {
                    button.showcase-tab.is-selected[self.is_selected]
                        type="button"
                        role="tab"
                        id=(&self.id)
                        aria-controls=(&self.controls)
                        aria-selected=(self.is_selected)
                        tabindex=(tab_index)
                        style=(style) {
                        (css())
                        (render_content(&icon, &self.text))
                    }
                }
            }
            TabInteraction::DatastarLocal { signal, value } => {
                let selected_expr = format!("${} == '{}'", signal, value);
                let selected_attr = format!("{} ? 'true' : 'false'", selected_expr);
                let tabindex_attr = format!("{} ? '0' : '-1'", selected_expr);
                let click_expr = format!("${} = '{}'", signal, value);

                maud::html! {
                    button.showcase-tab.tab-set__tab.ui-tab.is-selected[self.is_selected]
                        type="button"
                        role="tab"
                        id=(&self.id)
                        aria-controls=(&self.controls)
                        aria-selected=(self.is_selected)
                        tabindex=(tab_index)
                        data-tab-id=(value)
                        data-class:is-selected=(selected_expr)
                        data-attr:aria-selected=(selected_attr)
                        data-attr:tabindex=(tabindex_attr)
                        data-on:click=(click_expr)
                        style=(style) {
                        (css())
                        (render_content(&icon, &self.text))
                    }
                }
            }
        }
    }
}

fn render_content(icon: &Option<Icon>, text: &Text) -> maud::Markup {
    maud::html! {
        @if let Some(icon) = icon {
            span class="showcase-tab-icon" { (icon) }
        }
        span class="showcase-tab-label tab-set__tab-line" { (text) }
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
