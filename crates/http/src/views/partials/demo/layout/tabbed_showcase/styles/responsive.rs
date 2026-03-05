use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ResponsiveStyles;

inline_css! {
    @media (max-width: 48rem) {
      me.showcase {
        margin-top: var(--size-5);
        padding: var(--size-3);
      }

      me .showcase-shell {
        padding: var(--size-2);
      }

      me .showcase-panel,
      me .showcase-mockup,
      me .showcase-copy {
        min-height: auto;
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
