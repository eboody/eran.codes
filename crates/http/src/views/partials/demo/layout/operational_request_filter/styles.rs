crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-3);
  margin-top: var(--space-2);
  padding: var(--space-card);
  overflow: visible;
}

me [data-op-filter-label] {
  margin: 0;
  font-size: var(--text-size-label-xs);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-op-filter-row] {
  display: grid;
  gap: var(--space-2);
  grid-template-columns: minmax(0, 1fr) auto;
}

me [data-op-filter-row] > input[type='text'] {
  margin: 0;
  min-width: 0;
}

me [data-op-filter-row] > [data-button] {
  margin: 0;
}

me [data-op-filter-meta] {
  margin: 0;
  font-size: var(--text-size-meta-md);
  color: var(--text-muted);
}

@media (max-width: 48rem) {
  me [data-op-filter-row] {
    grid-template-columns: 1fr;
  }

  me [data-op-filter-row] > [data-button] {
    width: 100%;
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
