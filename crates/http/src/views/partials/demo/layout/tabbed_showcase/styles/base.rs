use maud::Render;
use maud_extensions::inline_css;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct BaseStyles;

inline_css! {
    me {
      --surface-shell: hsl(220 24% 97%);
      --surface-shell-border: hsl(220 16% 84%);
      --surface-panel: hsl(220 24% 95%);
      --surface-panel-border: hsl(220 16% 84%);
      --surface-row: hsl(220 24% 93%);
      --surface-row-border: hsl(220 16% 84%);
      --showcase-shell-glow-a: hsl(220 44% 80% / 0.12);
      --showcase-shell-glow-b: hsl(350 58% 80% / 0.1);

      --showcase-tone-accent: var(--ui-accent-primary);
      --showcase-tone-border: color-mix(
        in srgb,
        var(--showcase-tone-accent) 46%,
        transparent
      );
      --showcase-tone-surface-start: color-mix(
        in srgb,
        var(--showcase-tone-accent) 24%,
        var(--surface-panel)
      );
      --showcase-tone-surface-end: color-mix(
        in srgb,
        var(--showcase-tone-accent) 12%,
        var(--surface-panel)
      );
      --showcase-tone-chip-bg: color-mix(
        in srgb,
        var(--showcase-tone-accent) 16%,
        var(--surface-row)
      );
      --showcase-tone-chip-border: color-mix(
        in srgb,
        var(--showcase-tone-accent) 38%,
        transparent
      );

      margin-top: var(--size-7);
      border: var(--border-size-1) solid var(--surface-shell-border);
      border-radius: var(--radius-5);
      padding: var(--size-5);
      background:
        radial-gradient(
          circle at 0% 0%,
          var(--showcase-shell-glow-a),
          transparent 52%
        ),
        radial-gradient(
          circle at 100% 0%,
          var(--showcase-shell-glow-b),
          transparent 56%
        ),
        var(--surface-shell);
      box-shadow: var(--portfolio-shadow);
    }

    @media (prefers-color-scheme: dark) {
      me {
        --surface-shell: hsl(222 47% 11% / 0.75);
        --surface-shell-border: hsl(215 28% 50% / 0.38);
        --surface-panel: hsl(222 47% 11% / 0.72);
        --surface-panel-border: hsl(215 28% 50% / 0.3);
        --surface-row: hsl(222 47% 11% / 0.66);
        --surface-row-border: hsl(215 28% 50% / 0.3);
        --showcase-shell-glow-a: hsl(25 95% 54% / 0.1);
        --showcase-shell-glow-b: hsl(18 83% 55% / 0.08);
      }
    }

    me.showcase--netbird-detail {
      margin-top: var(--size-8);
    }

    me > .showcase-heading {
      margin-bottom: var(--size-4);
    }

    me > .showcase-heading > .showcase-title {
      display: grid;
      gap: var(--size-1);
    }

    me .showcase-title-text {
      margin: 0;
      color: var(--ui-text);
      font-size: var(--font-size-fluid-2);
      line-height: var(--font-lineheight-1);
    }

    me .showcase-title-subtitle {
      margin: 0;
      max-width: var(--size-content-4);
      color: var(--ui-text-muted);
    }

    me .showcase-shell {
      border: var(--border-size-1) solid var(--surface-shell-border);
      border-radius: var(--radius-4);
      padding: var(--size-2);
      background: var(--surface-panel);
    }

    me ::selection {
      background: color-mix(
        in srgb,
        var(--showcase-tone-accent, var(--ui-accent-primary)) 28%,
        transparent
      );
      color: var(--ui-text-on-accent);
    }
}

impl Render for BaseStyles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (css())
        }
    }
}
