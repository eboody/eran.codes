use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PanelStyles;

inline_css! {
    me .showcase-panels {
      margin-top: var(--size-3);
      padding: var(--size-2);
      border: var(--border-size-1) solid var(--surface-panel-border);
      border-radius: var(--radius-4);
      background: var(--surface-panel);
    }

    me .showcase-panel {
      display: grid;
      gap: var(--size-3);
      min-width: 0;
      min-height: var(--size-14);
    }

    @media (min-width: 64rem) {
      me .showcase-panel {
        grid-template-columns: 1.1fr 0.9fr;
      }

      me .showcase-panel--full {
        grid-template-columns: 1fr;
      }
    }

    me .showcase-panel > * {
      min-width: 0;
    }

    me .showcase-mockup {
      border: var(--border-size-1) solid var(--surface-row-border);
      border-radius: var(--radius-4);
      padding: var(--size-3);
      background: var(--surface-row);
      box-shadow: var(--shadow-1);
      display: grid;
      align-content: start;
      min-height: var(--size-14);
    }

    me .showcase-mockup > header > h3 {
      margin: 0 0 var(--size-1);
      color: var(--ui-text);
    }

    me .showcase-mockup > header > .is-muted {
      margin: 0;
      color: var(--ui-text-muted);
    }

    me .showcase-rows {
      margin: var(--size-4) 0 0;
      padding: 0;
      display: grid;
      gap: var(--size-2);
    }

    me .showcase-row {
      border: var(--border-size-1) solid var(--surface-row-border);
      border-radius: var(--radius-2);
      background: color-mix(in srgb, var(--surface-row) 90%, transparent);
      padding: var(--size-2) var(--size-3);
      display: flex;
      justify-content: space-between;
      gap: var(--size-3);
      margin: 0;
    }

    me .showcase-row-label {
      margin: 0;
      color: var(--ui-text-muted);
      font-size: var(--font-size-0);
    }

    me .showcase-row-value {
      margin: 0;
      color: var(--ui-text);
      font-size: var(--font-size-0);
      font-weight: var(--font-weight-6);
      text-align: right;
    }

    me .showcase-copy {
      border: var(--border-size-1) solid var(--showcase-tone-border);
      border-top: var(--border-size-2) solid var(--showcase-tone-accent);
      border-radius: var(--radius-4);
      padding: var(--size-5);
      background: linear-gradient(
        180deg,
        var(--showcase-tone-surface-start),
        var(--showcase-tone-surface-end)
      );
      color: var(--ui-text);
      box-shadow: var(--shadow-2);
      overflow: hidden;
      min-height: var(--size-14);
    }

    me .showcase-copy-content {
      width: 100%;
      max-width: var(--size-content-4);
      display: grid;
      gap: var(--size-4);
    }

    me .showcase-copy-content > h3 {
      margin: 0;
      color: inherit;
      font-size: var(--font-size-3);
    }

    me .showcase-copy-content > .is-muted {
      margin: 0;
      color: color-mix(
        in srgb,
        var(--ui-text-muted) 84%,
        var(--showcase-tone-accent) 16%
      );
      line-height: var(--font-lineheight-3);
    }

    me .showcase-bullets {
      margin: 0;
      padding-left: var(--size-5);
      display: grid;
      gap: var(--size-2);
    }

    me .showcase-bullets > li::marker {
      color: var(--showcase-tone-accent);
    }

    me .showcase-copy-content > .button {
      margin: 0;
      border-radius: var(--radius-round);
      border: var(--border-size-1) solid color-mix(in srgb, var(--ui-text) 24%, transparent);
      background: var(--showcase-tone-accent);
      color: var(--ui-text-on-accent);
      box-shadow: var(--shadow-2);
      justify-self: start;
    }

    me .showcase-integrations {
      margin: 0;
      display: grid;
      gap: var(--size-1);
      color: var(--ui-text-muted);
      font-size: var(--font-size-0);
    }

    me .showcase-integrations-label {
      margin: 0;
    }

    me .showcase-chip-list {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      gap: var(--size-1);
    }

    me .showcase-chip {
      border: var(--border-size-1) solid var(--showcase-tone-chip-border);
      border-radius: var(--radius-round);
      padding: var(--size-1) var(--size-2);
      background: var(--showcase-tone-chip-bg);
      color: var(--ui-text);
      font-weight: var(--font-weight-6);
      font-size: var(--font-size-0);
    }

    me .showcase-code-path {
      margin: 0;
      color: var(--ui-text-muted);
      font-size: var(--font-size-0);
    }

    me .showcase-code-path > code {
      font-family: var(--ui-font-mono);
      color: var(--ui-text);
    }
}

impl Render for PanelStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
