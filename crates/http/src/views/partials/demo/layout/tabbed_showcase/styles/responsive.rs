use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ResponsiveStyles;

inline_css! {
    @media (max-width: 48rem) {
      me[data-showcase-theme="netbird"],
      me[data-showcase-theme="netbird-detail"] {
        margin-top: var(--showcase-mobile-margin-top);
        padding: var(--showcase-space-3);
        --showcase-panel-min-height: auto;
        --showcase-mockup-min-height: auto;
        --showcase-copy-min-height: auto;
      }
      me > [data-showcase-root] > [data-showcase-shell] {
        padding: var(--showcase-mobile-shell-padding);
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > button[role="tab"] {
        min-width: max-content;
        font-size: var(--showcase-mobile-tab-font-size);
      }
    }
    @media (prefers-reduced-motion: reduce) {
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > button[role="tab"] {
        transition: none;
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-tabs]
        > [data-showcase-tab-indicator] {
        transition: none;
      }
    }
}

impl Render for ResponsiveStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
