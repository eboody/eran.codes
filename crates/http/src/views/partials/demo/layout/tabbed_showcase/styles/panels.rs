use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PanelStyles;

inline_css! {
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-panels] {
      margin-top: var(--showcase-space-3);
      padding: var(--showcase-space-2);
      border: var(--showcase-border-size) solid var(--showcase-panels-surface-border);
      border-radius: var(--showcase-radius-surface);
      background: var(--showcase-panels-surface-bg);
      box-shadow: var(--showcase-panels-surface-shadow);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel] {
      display: grid;
      gap: var(--showcase-space-3);
      min-width: 0;
      min-height: var(--showcase-panel-min-height);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel][hidden] {
      display: none;
    }
    @media (min-width: 64rem) {
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-panels]
        > [data-showcase-panel] {
        grid-template-columns: var(--showcase-panel-grid-main)
          var(--showcase-panel-grid-side);
        align-items: stretch;
      }
      me
        > [data-showcase-root]
        > [data-showcase-shell]
        > [data-showcase-panels]
        > [data-showcase-panel][data-panel-full] {
        grid-template-columns: 1fr;
      }
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > * {
      min-width: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup] {
      border: var(--showcase-border-size) solid var(--showcase-panel-border);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-space-3);
      background: var(--showcase-panel-bg);
      box-shadow: var(--shadow-1);
      display: grid;
      align-content: start;
      min-height: var(--showcase-mockup-min-height);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > header
      > h3 {
      margin-bottom: var(--showcase-mockup-title-margin-bottom);
      color: var(--showcase-mockup-title-color);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > header
      > [data-muted] {
      margin: 0;
      color: var(--showcase-mockup-muted-color);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows] {
      margin: var(--showcase-space-4) 0 0;
      padding: 0;
      display: grid;
      gap: var(--showcase-row-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > [data-showcase-row] {
      border: var(--showcase-border-size) solid var(--showcase-row-border);
      border-radius: var(--radius-2);
      background: var(--showcase-row-bg);
      padding: var(--showcase-row-padding-block) var(--showcase-row-padding-inline);
      display: flex;
      justify-content: space-between;
      gap: var(--showcase-space-3);
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > [data-showcase-row]
      > [data-showcase-row-label] {
      color: var(--showcase-row-label-color);
      font-size: var(--showcase-row-label-size);
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-mockup]
      > [data-showcase-rows]
      > [data-showcase-row]
      > [data-showcase-row-value] {
      font-size: var(--showcase-row-value-size);
      font-weight: var(--showcase-row-value-font-weight);
      color: var(--showcase-row-value-color);
      text-align: right;
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy] {
      border: var(--showcase-border-size) solid var(--tone-border);
      border-top: var(--showcase-border-size-strong) solid var(--tone-accent);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-copy-padding);
      background: linear-gradient(
        180deg,
        var(--tone-surface-start),
        var(--tone-surface-end)
      );
      color: var(--tone-copy-text, var(--showcase-copy-text-default));
      box-shadow: var(--shadow-2);
      overflow: hidden;
      display: flex;
      justify-content: flex-start;
      padding-bottom: var(--showcase-copy-min-padding-bottom);
      min-height: var(--showcase-copy-min-height);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content] {
      width: 100%;
      max-width: var(--showcase-copy-max-width);
      display: grid;
      gap: var(--showcase-copy-inner-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > h3 {
      color: inherit;
      margin: 0;
      font-size: var(--showcase-copy-title-size);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-muted] {
      margin: 0;
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
      line-height: var(--showcase-copy-line-height);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets] {
      margin: 0;
      padding-left: var(--showcase-copy-bullets-padding-left);
      display: grid;
      gap: var(--showcase-copy-bullets-gap);
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets]
      > li {
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-bullets]
      > li::marker {
      color: var(--tone-accent);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > .button {
      margin: 0;
      border-radius: var(--radius-round);
      background: var(--tone-accent);
      color: var(--tone-copy-text, var(--showcase-copy-text-default));
      border: var(--showcase-border-size) solid var(--showcase-button-border-color);
      box-shadow: var(--showcase-button-shadow);
      justify-self: start;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations] {
      margin: 0;
      display: grid;
      gap: var(--showcase-space-1);
      font-size: var(--showcase-chip-font-size);
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-integrations-label] {
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-chip-list] {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      gap: var(--showcase-integrations-gap);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-showcase-integrations]
      > [data-showcase-chip-list]
      > [data-showcase-chip] {
      border: var(--showcase-border-size) solid var(--tone-chip-border);
      border-radius: var(--radius-round);
      padding: var(--showcase-chip-padding-block)
        var(--showcase-chip-padding-inline);
      font-weight: var(--showcase-chip-font-weight);
      background: var(--tone-chip-bg, hsl(0 0% 100% / 0.14));
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-code-path] {
      margin: 0;
      color: var(--tone-copy-muted, var(--showcase-copy-muted-default));
      font-size: var(--font-size-0);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]
      > [data-showcase-copy-content]
      > [data-code-path]
      > code {
      word-break: break-all;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]::selection {
      background: var(--tone-accent);
      color: var(--showcase-selection-text);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel]
      > [data-showcase-copy]::-moz-selection {
      background: var(--tone-accent);
      color: var(--showcase-selection-text);
    }
}

impl Render for PanelStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
