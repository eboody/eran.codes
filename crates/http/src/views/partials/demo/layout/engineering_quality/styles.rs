crate::views::scoped::inline_css!(
    r#"
me [data-engineering-quality-grid] {
  display: grid;
  gap: var(--space-4);
}

me [data-engineering-quality-card] {
  --inset-card-padding: var(--space-card);

  display: grid;
  gap: var(--space-2);
  overflow: visible;
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me [data-engineering-quality-card-title] {
  margin: 0 0 var(--space-2);
}

me [data-engineering-quality-card-summary] {
  margin: 0;
  font-size: var(--text-size-body-md);
  line-height: var(--text-line-body);
  color: var(--text-muted);
}

me [data-engineering-quality-card-points] {
  margin: var(--space-3) 0 0;
  padding-left: var(--space-4);
  display: grid;
  gap: var(--space-2);
  font-size: var(--text-size-body-xs);
  line-height: var(--text-line-body);
  color: var(--text-muted);
}

@media (min-width: 980px) {
  me [data-engineering-quality-grid] {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (max-width: 48rem) {
  me [data-engineering-quality-grid] {
    gap: var(--space-3);
  }

  me [data-engineering-quality-card] {
    --inset-card-padding: var(--space-3);
    gap: var(--space-1);
  }

  me [data-engineering-quality-card-title] {
    margin-bottom: var(--space-1);
  }

  me [data-engineering-quality-card-points] {
    margin-top: var(--space-2);
    gap: var(--space-1);
    padding-left: var(--space-3);
  }
}

@media (hover: hover) {
  me [data-engineering-quality-card]:hover {
    transform: var(--motion-lift-subtle);
    border-color: color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
    box-shadow: var(--shadow-panel-hover);
  }
}

@media (prefers-reduced-motion: reduce) {
  me [data-engineering-quality-card] {
    transition: none;
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
