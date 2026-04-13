crate::views::scoped::inline_css!(
    r#"
me [data-burst-controls] {
  display: grid;
  gap: var(--space-4);
}

me [data-burst-slider] {
  display: grid;
  gap: var(--space-2);
  font-size: var(--control-font-size);
  font-weight: 600;
}

me [data-burst-slider] > span {
  font-size: var(--text-size-label-xs);
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-burst-slider] input[type='range'] {
  width: 100%;
  margin: 0;
  accent-color: var(--ui-accent-primary);
}

me [data-burst-selected] {
  margin: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: var(--interactive-bleed);
  font-family: var(--font-display);
  font-size: var(--text-size-title-sm);
  color: var(--text-body);
}

me [data-burst-selected] strong {
  color: var(--text-strong);
}

me [data-burst-actions] {
  align-items: center;
  gap: var(--space-2) var(--space-3);
  min-width: 0;
}

me [data-burst-actions-note] {
  margin: 0;
  font-size: var(--text-size-meta-lg);
}

me .ui-request-burst-result {
  margin-top: 0;
  border: 1px solid color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
  --inset-card-padding: var(--space-3) var(--space-4);
  background: color-mix(in srgb, var(--accent-signal-soft) 38%, var(--surface-field));
  font-family: var(--ui-font-mono);
  font-size: var(--text-size-meta-sm);
  line-height: var(--text-line-body-loose);
  color: var(--text-body);
  overflow: visible;
}

me .ui-request-burst-status {
  margin: 0 0 var(--space-3);
}

me [data-burst-endpoint],
me [data-burst-previous],
me [data-burst-delta] {
  white-space: normal;
  overflow-wrap: anywhere;
}

@media (prefers-color-scheme: dark) {
  me .ui-request-burst-result {
    background:
      linear-gradient(
        180deg,
        color-mix(in srgb, var(--accent-signal) 10%, var(--surface-wash-top-soft)),
        transparent 30%
      ),
      color-mix(in srgb, var(--accent-signal) 14%, var(--surface-field));
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }
}

@media (min-width: 48rem) {
  me [data-burst-controls] {
    grid-template-columns: minmax(0, 1.35fr) minmax(14rem, 0.95fr);
    align-items: end;
    column-gap: var(--space-card);
  }

  me [data-burst-slider],
  me [data-burst-result] {
    grid-column: 1 / -1;
  }

  me [data-burst-actions] {
    justify-content: flex-end;
    align-self: end;
  }
}

@media (max-width: 48rem) {
  me [data-burst-controls] {
    gap: var(--space-3);
  }

  me [data-burst-selected] {
    font-size: var(--text-size-title-xs);
  }

  me [data-burst-actions] {
    align-items: stretch;
  }

  me [data-burst-actions] [data-button] {
    width: 100%;
  }

  me .ui-request-burst-result {
    --inset-card-padding: var(--space-2);
    font-size: var(--text-size-meta-xs);
  }

  me .ui-request-burst-status {
    margin-bottom: var(--space-1);
  }
}

@media (max-width: 26rem) {
  me [data-burst-controls] {
    gap: var(--space-2);
  }

  me [data-burst-slider] {
    gap: var(--space-1);
  }

  me [data-burst-actions] {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-1);
  }

  me [data-burst-actions-note] {
    font-size: var(--text-size-meta-sm);
    max-inline-size: 100%;
    overflow-wrap: anywhere;
  }

  me .ui-request-burst-result {
    --inset-card-padding: var(--space-2) var(--space-1);
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
