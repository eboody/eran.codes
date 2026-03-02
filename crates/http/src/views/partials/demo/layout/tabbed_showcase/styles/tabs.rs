use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct TabStyles;

inline_css! {
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs] {
      position: relative;
      display: flex;
      align-items: stretch;
      gap: var(--showcase-tab-gap);
      overflow-x: auto;
      padding-inline: var(--showcase-tabs-inline-pad);
      padding-bottom: calc(
        var(--showcase-tab-list-padding-bottom) + var(--showcase-indicator-height)
      );
      scroll-padding-inline: var(--showcase-tabs-edge-fade-size);
      border-bottom: var(--showcase-border-size) solid var(--showcase-tab-divider);
      scrollbar-width: thin;
      --showcase-active-tone: hsl(25 91% 58%);
      --showcase-active-tone-shadow: 0 2px 8px hsl(25 91% 58% / 0.3);
    }
    me[data-showcase-theme="netbird-detail"]
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs] {
      --showcase-active-tone: var(--blue-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(206 80% 58% / 0.28);
    }
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs]::before,
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs]::after {
      content: "";
      position: absolute;
      top: 0;
      bottom: calc(
        var(--showcase-tab-list-padding-bottom) + var(--showcase-indicator-height)
      );
      width: var(--showcase-tabs-edge-fade-size);
      pointer-events: none;
      opacity: var(--showcase-tabs-edge-fade-opacity);
      transition: opacity var(--showcase-tab-transition-duration) var(--ease-out-3);
      z-index: 1;
    }
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs]::before {
      left: 0;
      background: linear-gradient(90deg, hsl(220 36% 7% / 0.88), transparent);
    }
    me > [data-showcase-root] > [data-showcase-shell] > [data-showcase-tabs]::after {
      right: 0;
      background: linear-gradient(270deg, hsl(220 36% 7% / 0.88), transparent);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-scroll-left="true"]::before {
      opacity: var(--showcase-tabs-edge-fade-opacity-visible);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-scroll-right="true"]::after {
      opacity: var(--showcase-tabs-edge-fade-opacity-visible);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"] {
      cursor: pointer;
      border-radius: var(--radius-2);
      border: var(--showcase-border-size) solid var(--showcase-tab-border);
      background: var(--showcase-tab-bg);
      color: var(--showcase-tab-text);
      padding: var(--showcase-tab-padding-y) var(--showcase-tab-padding-x);
      min-width: max-content;
      font-size: var(--showcase-tab-font-size);
      font-weight: var(--showcase-tab-font-weight);
      letter-spacing: var(--showcase-tab-letter-spacing);
      display: inline-flex;
      align-items: center;
      gap: var(--showcase-space-2);
      transition:
        border-color var(--showcase-tab-transition-duration) var(--ease-3),
        background-color var(--showcase-tab-transition-duration) var(--ease-3),
        color var(--showcase-tab-transition-duration) var(--ease-3),
        transform var(--showcase-tab-transition-duration) var(--ease-3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:hover {
      border-color: transparent;
      background: var(--showcase-tab-hover-bg);
      color: var(--showcase-tab-text-active);
      transform: translateY(var(--showcase-tab-hover-offset));
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"][aria-selected="true"] {
      border-color: var(--showcase-tab-active-border);
      background: var(--showcase-tab-active-bg);
      color: var(--showcase-tab-text-selected);
      font-weight: var(--font-weight-7);
      box-shadow: var(--showcase-tab-active-shadow);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:focus-visible {
      outline: none;
      outline-offset: var(--showcase-focus-outline-offset);
      border-color: var(--showcase-focus-border-color);
      background: var(--showcase-focus-background);
      box-shadow: var(--showcase-focus-inset-ring);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]:focus:not(:focus-visible) {
      outline: none;
      box-shadow: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content] {
      display: inline-flex;
      align-items: center;
      gap: var(--showcase-space-2);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-label] {
      white-space: nowrap;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-icon] {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-width: var(--showcase-tab-icon-min-width);
      color: inherit;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"]
      > [data-showcase-tab-content]
      > [data-showcase-tab-icon]
      > svg {
      width: var(--showcase-tab-icon-size);
      height: var(--showcase-tab-icon-size);
      display: block;
      stroke: currentColor;
      fill: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tab-indicator] {
      position: absolute;
      left: 0;
      bottom: 0;
      width: var(--showcase-indicator-width);
      height: var(--showcase-indicator-height);
      transform: translateX(var(--showcase-indicator-x));
      border-radius: var(--radius-round);
      background: var(--showcase-active-tone);
      box-shadow: var(--showcase-active-tone-shadow);
      transition:
        transform var(--showcase-tab-indicator-transition-duration)
          var(--ease-spring-2),
        width var(--showcase-tab-indicator-transition-duration) var(--ease-3),
        background-color var(--showcase-tab-transition-duration) var(--ease-out-3),
        box-shadow var(--showcase-tab-transition-duration) var(--ease-out-3);
      pointer-events: none;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="indigo"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--indigo-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(229 68% 58% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="sky"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--blue-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(206 80% 58% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="teal"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--teal-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(183 61% 52% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="mint"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--green-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(151 48% 48% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="violet"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--violet-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(274 64% 61% / 0.3);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs][data-active-tone="amber"]
      > [data-showcase-tab-indicator] {
      --showcase-active-tone: var(--orange-6);
      --showcase-active-tone-shadow: 0 2px 8px hsl(25 91% 58% / 0.3);
    }
}

impl Render for TabStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
