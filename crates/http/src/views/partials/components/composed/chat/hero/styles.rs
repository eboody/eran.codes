crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-card);
  align-items: start;
  margin-top: 0;
  view-transition-name: chat-hero;
}

me [data-chat-hero-copy] {
  min-width: 0;
}

me [data-chat-hero-copy] > [data-section-header] {
  margin-bottom: 0;
}

me [data-chat-hero-copy] > [data-section-header] h1 {
  font-size: var(--text-size-hero-md);
  line-height: var(--text-line-heading);
}

me [data-chat-hero-copy] > [data-section-header] .u-muted {
  max-width: 54ch;
}

me [data-chat-hero-card] {
  --inset-card-border: var(--ui-border-soft);
  --inset-card-padding: var(--space-4);
  display: grid;
  gap: var(--space-2);
  inline-size: min(100%, 21rem);
}

me [data-chat-hero-card-title] {
  margin: 0;
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wider);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-chat-hero-card] p {
  margin: 0;
}

me [data-chat-hero-card] .u-muted {
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

@media (min-width: 900px) {
  me {
    grid-template-columns: minmax(0, 1.25fr) minmax(18rem, 0.75fr);
  }

  me [data-chat-hero-card] {
    justify-self: end;
  }
}

@media (max-width: 768px) {
  me {
    gap: var(--space-3);
  }

  me [data-chat-hero-card] .button {
    width: 100%;
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
