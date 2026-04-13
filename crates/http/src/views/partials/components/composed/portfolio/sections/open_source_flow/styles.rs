crate::views::scoped::inline_css!(
    r#"
me {
  --open-source-max-inline-size: min(104rem, calc(100vw - (var(--shell-gutter) * 2)));

  display: grid;
  gap: clamp(1.35rem, 1.05rem + 0.9vw, 2.1rem);
  inline-size: min(100%, var(--open-source-max-inline-size));
  max-inline-size: var(--open-source-max-inline-size);
  margin-top: var(--space-5);
  margin-inline: auto;
}

me .ui-open-source-hero-aside {
  gap: var(--space-2);
}

me .ui-open-source-hero-intro {
  display: grid;
  gap: var(--space-1);
}

me .ui-open-source-hero-intro p {
  margin: 0;
}

me .ui-open-source-hero-footnote strong {
  font-size: var(--text-size-label-sm);
  letter-spacing: var(--text-track-ui);
}

me .ui-open-source-hero-footnote {
  margin: 0;
  color: var(--ui-text-muted);
}

me .ui-open-source-hero-footnote {
  padding-top: var(--space-1);
  border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 78%, transparent);
}

me .ui-open-source-supporting-libraries {
  display: grid;
  gap: var(--space-2);
}

me .ui-open-source-supporting-label {
  margin: 0;
  font-size: var(--text-size-meta-xs);
  letter-spacing: var(--text-track-caps-sm);
  text-transform: uppercase;
  color: var(--ui-text-muted);
}

me .ui-open-source-supporting-grid {
  gap: var(--space-2);
}

me .ui-open-source-supporting-card {
  gap: var(--space-2);
}

me .ui-open-source-mobile-intro {
  display: none;
}

@media (max-width: 64rem) {
  me {
    inline-size: 100%;
    margin-inline: 0;
  }
}

@media (max-width: 48rem) {
  me {
    gap: var(--space-4);
    max-inline-size: none;
    margin-top: var(--space-4);
  }

  me .ui-open-source-hero-aside {
    display: none;
  }

  me .ui-open-source-mobile-intro {
    display: grid;
    gap: var(--space-1);
    margin-top: var(--space-1);
    padding-top: var(--space-2);
    border-top: 1px solid color-mix(in srgb, var(--ui-border-soft) 72%, transparent);
  }

  me .ui-open-source-mobile-intro-eyebrow {
    margin: 0;
    font-size: var(--text-size-meta-xs);
    letter-spacing: var(--text-track-caps-sm);
    text-transform: uppercase;
    color: var(--ui-text-muted);
  }

  me .ui-open-source-mobile-intro .ui-portfolio-summary {
    font-size: var(--text-size-body-md);
    line-height: var(--portfolio-feature-summary-line-height);
  }
}

@media (max-width: 26rem) {
  me .ui-open-source-mobile-intro {
    gap: calc(var(--space-1) * 0.75);
    margin-top: calc(var(--space-1) * 0.75);
    padding-top: var(--space-1);
  }

  me .ui-portfolio-badges {
    gap: calc(var(--space-1) * 0.85);
  }

  me .ui-portfolio-badges li {
    padding: var(--portfolio-badge-padding-compact);
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
