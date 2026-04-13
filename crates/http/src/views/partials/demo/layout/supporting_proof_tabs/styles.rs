crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-4);
}

me [data-supporting-proof-intro] {
  display: grid;
  gap: var(--space-2);
  max-width: 60ch;
}

me [data-supporting-proof-kicker] {
  margin: 0;
  font-size: var(--text-size-label-xs);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wider);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-supporting-proof-intro] h2 {
  margin: 0;
}

me [data-supporting-proof-summary] {
  margin: 0;
  color: color-mix(in srgb, var(--text-body) 88%, var(--text-muted) 12%);
}

me [data-supporting-proof-panel] {
  display: grid;
}

me [data-supporting-proof-panel][data-local-tab-entering='1'] {
  animation: supporting-proof-panel-enter var(--motion-slow) var(--ease-out-3);
}

@keyframes supporting-proof-panel-enter {
  from {
    opacity: 0;
    transform: translateY(0.35rem);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-3);
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
