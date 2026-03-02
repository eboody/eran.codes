use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct BaseStyles;

inline_css! {
    me {
      --showcase-space-1: var(--size-2);
      --showcase-space-2: var(--size-3);
      --showcase-space-3: var(--size-4);
      --showcase-space-4: var(--size-5);
      --showcase-space-5: var(--size-6);
      --showcase-space-6: var(--size-7);
      --showcase-radius-shell: var(--radius-5);
      --showcase-radius-surface: var(--radius-4);
      --showcase-border-size: var(--border-size-1);
      --showcase-border-size-strong: var(--border-size-2);
      --showcase-section-margin-top: var(--size-7);
      --showcase-heading-gap: var(--size-4);
      --showcase-tab-gap: var(--size-3);
      --showcase-tab-list-padding-bottom: var(--size-2);
      --showcase-tabs-inline-pad: var(--size-1);
      --showcase-tabs-edge-fade-size: var(--size-8);
      --showcase-tabs-edge-fade-opacity: 0;
      --showcase-tabs-edge-fade-opacity-visible: 1;
      --showcase-tab-font-size: var(--font-size-0);
      --showcase-tab-font-weight: var(--font-weight-6);
      --showcase-tab-letter-spacing: var(--font-letterspacing-2);
      --showcase-tab-padding-y: var(--size-2);
      --showcase-tab-padding-x: var(--size-2);
      --showcase-tab-hover-offset: calc(var(--border-size-1) * -1);
      --showcase-focus-outline-size: var(--border-size-2);
      --showcase-focus-outline-offset: 0;
      --showcase-focus-border-color: hsl(220 62% 66% / 0.92);
      --showcase-focus-background: hsl(220 34% 16% / 0.72);
      --showcase-focus-inset-ring: inset 0 0 0 var(--showcase-focus-outline-size)
        var(--tone-accent);
      --showcase-shell-blur: var(--size-px-2);
      --showcase-tab-transition-duration: 240ms;
      --showcase-tab-indicator-transition-duration: 320ms;
      --showcase-tab-icon-size: var(--size-4);
      --showcase-tab-icon-min-width: calc(
        var(--showcase-tab-icon-size) + var(--size-1)
      );
      --showcase-shell-bg: var(--gray-12);
      --showcase-shell-bg-alt: hsl(222 40% 8%);
      --showcase-shell-border: hsl(220 16% 25% / 0.78);
      --showcase-shell-highlight-a: hsl(25 91% 58% / 0.08);
      --showcase-shell-highlight-b: hsl(205 85% 62% / 0.06);
      --showcase-shell-shadow: var(--shadow-6);
      --showcase-panel-bg: hsl(220 16% 10% / 0.96);
      --showcase-panel-border: hsl(220 12% 28% / 0.48);
      --showcase-row-bg: hsl(220 16% 14% / 0.96);
      --showcase-row-border: hsl(220 12% 29% / 0.42);
      --showcase-tab-bg: transparent;
      --showcase-tab-text: hsl(214 18% 79% / 0.92);
      --showcase-tab-text-active: var(--gray-0);
      --showcase-tab-text-selected: var(--tone-accent, var(--showcase-tab-text-active));
      --showcase-tab-hover-bg: var(--tone-tab-soft, hsl(220 14% 18% / 0.82));
      --showcase-tab-active-bg: color-mix(in srgb, var(--tone-accent) 14%, transparent);
      --showcase-tab-active-border: color-mix(in srgb, var(--tone-accent) 55%, transparent);
      --showcase-tab-active-shadow: inset 0 0 0 var(--border-size-1)
        color-mix(in srgb, var(--tone-accent) 35%, transparent);
      --showcase-tab-border: transparent;
      --showcase-tab-divider: hsl(220 14% 24% / 0.85);
      --showcase-heading-title-color: var(--gray-0);
      --showcase-heading-subtitle-color: hsl(214 18% 78% / 0.9);
      --showcase-copy-text-default: var(--gray-0);
      --showcase-copy-muted-default: hsl(215 14% 70% / 0.92);
      --showcase-copy-title-size: var(--font-size-3);
      --showcase-copy-line-height: var(--font-lineheight-3);
      --showcase-copy-padding: var(--size-5);
      --showcase-copy-max-width: var(--size-content-4);
      --showcase-copy-inner-gap: var(--size-4);
      --showcase-copy-min-padding-bottom: var(--size-5);
      --showcase-copy-min-height: var(--size-14);
      --showcase-copy-bullets-padding-left: var(--size-5);
      --showcase-copy-bullets-gap: var(--size-2);
      --showcase-integrations-gap: var(--size-1);
      --showcase-chip-padding-block: var(--size-1);
      --showcase-chip-padding-inline: var(--size-2);
      --showcase-chip-font-size: var(--font-size-0);
      --showcase-row-gap: var(--size-2);
      --showcase-row-padding-block: var(--size-2);
      --showcase-row-padding-inline: var(--size-3);
      --showcase-row-label-size: var(--font-size-0);
      --showcase-row-value-size: var(--font-size-0);
      --showcase-row-label-color: hsl(216 14% 68% / 0.88);
      --showcase-row-value-color: hsl(216 18% 82% / 0.9);
      --showcase-mockup-title-color: hsl(215 22% 82% / 0.9);
      --showcase-mockup-muted-color: hsl(216 14% 66% / 0.88);
      --showcase-mockup-min-height: var(--size-14);
      --showcase-mockup-title-margin-bottom: var(--size-1);
      --showcase-indicator-height: var(--border-size-3);
      --showcase-indicator-width: 0;
      --showcase-indicator-x: 0;
      --showcase-panel-grid-main: 1.1fr;
      --showcase-panel-grid-side: 0.9fr;
      --showcase-panel-min-height: var(--size-14);
      --showcase-mobile-margin-top: var(--size-5);
      --showcase-mobile-shell-padding: var(--size-3);
      --showcase-mobile-tab-font-size: var(--font-size-0);
      --showcase-row-value-font-weight: var(--font-weight-6);
      --showcase-chip-font-weight: var(--font-weight-6);
      --showcase-button-border-color: hsl(0 0% 100% / 0.36);
      --showcase-selection-text: var(--gray-0);
      --showcase-button-shadow: var(--shadow-3);
      --showcase-panels-surface-bg: hsl(220 24% 7% / 0.88);
      --showcase-panels-surface-border: hsl(220 12% 24% / 0.82);
      --showcase-panels-surface-shadow: inset 0 var(--border-size-1) 0
        hsl(220 14% 28% / 0.36);
      margin-top: var(--showcase-section-margin-top);
    }
    me[data-showcase-theme="netbird"] {
      border: var(--showcase-border-size) solid var(--showcase-shell-border);
      border-radius: var(--showcase-radius-shell);
      padding: var(--showcase-space-4);
      background:
        radial-gradient(
          circle at 0% 0%,
          var(--showcase-shell-highlight-a),
          transparent 48%
        ),
        radial-gradient(
          circle at 100% 0%,
          var(--showcase-shell-highlight-b),
          transparent 55%
        ),
        linear-gradient(
          180deg,
          var(--showcase-shell-bg),
          var(--showcase-shell-bg-alt) 74%
        );
      box-shadow: var(--showcase-shell-shadow);
    }
    me[data-showcase-theme="netbird-detail"] {
      --showcase-section-margin-top: var(--size-8);
      border: var(--showcase-border-size) solid var(--showcase-shell-border);
      border-radius: var(--showcase-radius-shell);
      padding: var(--showcase-space-4);
      background:
        radial-gradient(circle at 0% 0%, hsl(229 68% 58% / 0.12), transparent 52%),
        radial-gradient(circle at 100% 0%, hsl(206 80% 58% / 0.1), transparent 56%),
        linear-gradient(180deg, hsl(220 36% 7%), hsl(223 46% 8%) 72%);
      box-shadow: var(--showcase-shell-shadow);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone] {
      --tone-base: var(--indigo-5);
      --tone-base-strong: var(--indigo-6);
      --tone-base-end: var(--indigo-8);
      --tone-accent: var(--indigo-5);
      --tone-tab-soft: hsl(229 69% 25% / 0.28);
      --tone-surface-start: hsl(229 40% 14%);
      --tone-surface-end: hsl(229 48% 10%);
      --tone-border: hsl(229 58% 32% / 0.56);
      --tone-copy-text: hsl(0 0% 100%);
      --tone-copy-muted: hsl(214 18% 76% / 0.88);
      --tone-chip-bg: hsl(220 22% 14% / 0.92);
      --tone-chip-border: hsl(220 10% 32% / 0.74);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="indigo"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="indigo"] {
      --tone-base: var(--indigo-5);
      --tone-base-strong: var(--indigo-6);
      --tone-base-end: var(--indigo-8);
      --tone-accent: var(--indigo-5);
      --tone-tab-soft: hsl(229 69% 25% / 0.28);
      --tone-surface-start: hsl(229 40% 14%);
      --tone-surface-end: hsl(229 48% 10%);
      --tone-border: hsl(229 58% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="sky"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="sky"] {
      --tone-base: var(--blue-5);
      --tone-base-strong: var(--blue-6);
      --tone-base-end: var(--blue-8);
      --tone-accent: var(--blue-5);
      --tone-tab-soft: hsl(205 75% 24% / 0.28);
      --tone-surface-start: hsl(208 46% 14%);
      --tone-surface-end: hsl(212 52% 10%);
      --tone-border: hsl(208 64% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="teal"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="teal"] {
      --tone-base: var(--teal-5);
      --tone-base-strong: var(--teal-6);
      --tone-base-end: var(--cyan-8);
      --tone-accent: var(--teal-5);
      --tone-tab-soft: hsl(182 58% 24% / 0.28);
      --tone-surface-start: hsl(186 44% 13%);
      --tone-surface-end: hsl(190 52% 9%);
      --tone-border: hsl(184 55% 31% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="mint"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="mint"] {
      --tone-base: var(--green-5);
      --tone-base-strong: var(--green-6);
      --tone-base-end: var(--lime-8);
      --tone-accent: var(--green-5);
      --tone-tab-soft: hsl(152 46% 23% / 0.28);
      --tone-surface-start: hsl(154 40% 13%);
      --tone-surface-end: hsl(158 48% 9%);
      --tone-border: hsl(153 48% 30% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="violet"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="violet"] {
      --tone-base: var(--violet-5);
      --tone-base-strong: var(--violet-6);
      --tone-base-end: var(--purple-8);
      --tone-accent: var(--violet-5);
      --tone-tab-soft: hsl(274 53% 24% / 0.28);
      --tone-surface-start: hsl(275 42% 14%);
      --tone-surface-end: hsl(278 50% 10%);
      --tone-border: hsl(274 50% 32% / 0.56);
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > [data-showcase-tone="amber"],
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-tone="amber"] {
      --tone-base: hsl(25 91% 58%);
      --tone-base-strong: hsl(25 91% 58%);
      --tone-base-end: hsl(14 89% 57%);
      --tone-accent: hsl(25 91% 58%);
      --tone-tab-soft: hsl(25 91% 58% / 0.24);
      --tone-surface-start: hsl(22 56% 14%);
      --tone-surface-end: hsl(18 66% 10%);
      --tone-border: hsl(24 72% 35% / 0.6);
    }
    me > [data-showcase-heading] {
      margin-bottom: var(--showcase-heading-gap);
    }
    me > [data-showcase-heading] > [data-showcase-title] {
      display: grid;
      gap: var(--showcase-space-1);
    }
    me
      > [data-showcase-heading]
      > [data-showcase-title]
      > [data-showcase-title-text] {
      margin: 0;
      color: var(--showcase-heading-title-color);
      font-size: var(--font-size-fluid-2);
      line-height: var(--font-lineheight-1);
    }
    me
      > [data-showcase-heading]
      > [data-showcase-title]
      > [data-showcase-title-subtitle] {
      margin: 0;
      color: var(--showcase-heading-subtitle-color);
      max-width: var(--size-content-4);
    }
    me > [data-showcase-root] > [data-showcase-shell] {
      border: var(--showcase-border-size) solid var(--showcase-shell-border);
      border-radius: var(--showcase-radius-surface);
      padding: var(--showcase-space-2);
      background: var(--showcase-shell-bg);
      backdrop-filter: blur(var(--showcase-shell-blur));
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-tabs]
      > button[role="tab"] {
      margin: 0;
    }
    me
      > [data-showcase-root]
      > [data-showcase-shell]
      > [data-showcase-panels]
      > [data-showcase-panel] {
      margin: 0;
      padding: 0;
      border: 0;
      background: transparent;
      box-shadow: none;
    }
}

impl Render for BaseStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
