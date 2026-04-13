crate::views::scoped::inline_css!(
    r#"
me {
  --_nav-shell-padding: 0.75rem 0.85rem;
  --_nav-link-font-size: var(--text-size-meta-md);
  --_nav-brand-wrap-size: 2.6rem;
  --_nav-brand-mark-size: 2.1rem;
  --_nav-list-gap: 0.35rem;
  --_nav-meta-gap: 0.1rem;
  --_nav-auth-gap: 0.2rem;
  --_nav-auth-text-gap: 0.28rem;
  --_nav-primary-gap-compact: 0.15rem;
  --_nav-primary-row-gap-mobile: 0.2rem;
  --_nav-auth-gap-mobile: 0.25rem;
  position: sticky;
  top: var(--nav-sticky-offset);
  z-index: 20;
  margin-top: var(--nav-sticky-offset);
  margin-bottom: clamp(0.45rem, 0.3rem + 0.55vw, 0.8rem);
  view-transition-name: app-nav;
}

me > [data-nav] {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  align-items: center;
  position: relative;
  isolation: isolate;
  gap: var(--space-2) var(--space-4);
  padding: var(--_nav-shell-padding);
  border-radius: var(--ui-radius-md);
  border: 1px solid var(--border-default);
  background: var(--surface-fill-panel);
  box-shadow: var(--shadow-panel);
  overflow: visible;
}

me > [data-nav][data-nav-layout='split'] [data-nav-trailing] {
  gap: 0;
  padding-inline-start: 0;
  border-inline-start: none;
}

me [data-nav-list] {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  align-items: center;
  gap: var(--_nav-list-gap);
}

me [data-nav-link] {
  margin-bottom: 0;
  padding-block: calc(var(--control-padding-block) - 0.25rem);
  padding-inline: calc(var(--control-padding-inline) - 0.45rem);
  border-radius: calc(var(--control-radius) - 2px);
  border: 1px solid transparent;
  font-size: var(--_nav-link-font-size);
  white-space: nowrap;
  position: relative;
  z-index: 0;
  color: var(--text-muted);
  text-decoration: none;
  transition:
    color var(--motion-fast),
    background-color var(--motion-fast),
    transform var(--motion-fast);
}

me [data-nav-trailing] {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  justify-self: end;
  padding-inline-start: var(--space-3);
  border-inline-start: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
}

me [data-nav-link-label='compact'] {
  display: none;
}

me [data-nav-link][aria-current="page"] {
  color: var(--text-strong);
  border-color: color-mix(in srgb, var(--accent-signal) 30%, var(--border-default));
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 42%, transparent),
      transparent 78%
    ),
    color-mix(in srgb, var(--surface-panel) 94%, var(--accent-signal-soft));
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    0 0 0 1px color-mix(in srgb, var(--accent-signal) 10%, transparent);
}

me [data-nav-link]:focus-visible {
  outline: none;
  color: var(--text-strong);
  background: var(--accent-signal-soft);
  z-index: 1;
}

me [data-nav-brand] {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  min-width: 0;
}

me [data-nav-brand-link] {
  display: inline-flex;
  align-items: center;
  gap: var(--control-gap);
  position: relative;
  z-index: 0;
  color: var(--text-strong);
  text-decoration: none;
  transition:
    opacity var(--motion-fast),
    transform var(--motion-fast);
}

me [data-nav-brand-link]:focus-visible {
  outline: 2px solid color-mix(in srgb, var(--accent-signal) 64%, white);
  outline-offset: var(--interactive-bleed);
  border-radius: calc(var(--control-radius) - 2px);
  z-index: 1;
}

me [data-nav-brand-picture] {
  display: flex;
  position: relative;
  z-index: 1;
}

me [data-nav-brand-mark-wrap] {
  --_logo-glow-red: rgb(218 89 85 / 0.88);
  --_logo-glow-blue: rgb(38 125 255 / 0.88);

  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  inline-size: var(--_nav-brand-wrap-size);
  block-size: var(--_nav-brand-wrap-size);
  flex: none;
  isolation: isolate;
}

me [data-nav-brand-mark-wrap]::before {
  content: "";
  position: absolute;
  inset: auto;
  inline-size: 100%;
  block-size: 100%;
  top: 50%;
  left: 50%;
  z-index: 0;
  border-radius: 0.85rem;
  background-image: linear-gradient(
    -45deg,
    var(--_logo-glow-red) 50%,
    var(--_logo-glow-blue) 50%
  );
  filter: blur(0.7rem);
  opacity: 0.1;
  transform: translate(-50%, -50%);
}

me [data-nav-brand-mark] {
  display: block;
  inline-size: var(--_nav-brand-mark-size);
  block-size: var(--_nav-brand-mark-size);
  flex: none;
  filter: drop-shadow(0.18rem 0.24rem 0.8rem color-mix(in srgb, black 18%, transparent));
}

me [data-nav-brand-text] {
  font-family: var(--ui-font-display);
  font-size: var(--text-size-body-xl);
  font-weight: 600;
  letter-spacing: var(--text-track-tight);
  line-height: var(--text-line-flat);
}

me [data-nav-list='primary'] {
  flex: 1;
  min-width: 0;
  justify-content: center;
  flex-wrap: wrap;
}

me [data-nav-list='meta'] {
  gap: var(--_nav-meta-gap);
}

me [data-nav-list='meta'] [data-nav-link] {
  color: color-mix(in srgb, var(--text-muted) 80%, var(--text-body) 20%);
  font-size: var(--text-size-meta-sm);
}

me [data-nav-list='auth'] {
  min-width: 0;
  justify-content: flex-end;
  flex-wrap: wrap;
  gap: var(--_nav-auth-gap);
}

me [data-nav-list='auth'] li {
  min-width: 0;
}

me [data-nav-auth-text] {
  display: inline-flex;
  align-items: center;
  gap: var(--_nav-auth-text-gap);
  font-size: var(--_nav-link-font-size);
  color: var(--text-muted);
  min-width: 0;
  max-inline-size: 8.75rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

me [data-nav-auth-prefix] {
  flex: none;
  white-space: nowrap;
}

me [data-nav-auth-name] {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

me [data-nav-list='auth'] form {
  margin: 0;
}

me [data-nav-list='auth'] :where(button, [data-nav-link]) {
  margin-bottom: 0;
}

me [data-nav-list='auth'] [data-nav-link] {
  padding-inline: calc(var(--control-padding-inline) - 0.55rem);
}

me [data-nav-auth-action] {
  --_button-padding-block: var(--control-padding-block-compact);
  --_button-padding-inline: var(--control-padding-inline-compact);
  --_button-font-size: var(--control-font-size-compact);
}

me [data-nav-guest-auth] {
  min-width: 0;
  display: grid;
  justify-items: end;
}

me [data-nav-guest-auth] .ui-button-row {
  --button-row-gap: 0.35rem;
  --button-row-item-min-inline-size: 7.4rem;
}

me [data-nav-list='auth'] [data-nav-link-cta='true'] {
  color: var(--ui-text-on-accent);
  border-color: color-mix(in srgb, var(--ui-accent-primary) 56%, var(--border-default));
  background: var(--ui-accent-primary);
  box-shadow:
    inset 0 1px 0 var(--control-edge-accent),
    0 10px 24px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
}

@media (hover: hover) {
  me [data-nav-link]:not([aria-current="page"]):hover {
    color: var(--text-strong);
    background: var(--accent-signal-soft);
    z-index: 1;
  }

  me [data-nav-list='meta'] [data-nav-link]:hover {
    color: var(--text-strong);
  }

  me [data-nav-brand-link]:hover {
    opacity: 0.9;
  }

  me [data-nav-list='auth'] [data-nav-link-cta='true']:hover {
    color: var(--ui-text-on-accent);
    transform: translateY(-1px);
    box-shadow:
      inset 0 1px 0 var(--control-edge-accent-hover),
      0 14px 28px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
  }
}

@media (max-width: 96rem) and (min-width: 48.001rem) {
  me {
    --_nav-shell-padding: 0.68rem 0.78rem;
    --_nav-link-font-size: var(--text-size-meta-sm);
  }

  me > [data-nav] {
    gap: var(--space-2) var(--space-3);
  }

  me [data-nav-brand-text] {
    font-size: var(--text-size-body-lg);
  }

  me [data-nav-list='primary'] {
    flex-wrap: nowrap;
    gap: var(--_nav-primary-gap-compact);
  }

  me [data-nav-list='primary'] [data-nav-link-label='full'] {
    display: none;
  }

  me [data-nav-list='primary'] [data-nav-link-label='compact'] {
    display: inline;
  }

  me [data-nav-link] {
    padding-inline: calc(var(--control-padding-inline) - 0.62rem);
  }

  me [data-nav-trailing] {
    gap: var(--space-2);
    padding-inline-start: var(--space-2);
  }

  me [data-nav-list='meta'] {
    gap: 0;
  }

  me [data-nav-list='meta'] [data-nav-link] {
    font-size: var(--text-size-label-md);
    padding-inline: calc(var(--control-padding-inline) - 0.7rem);
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-gap: 0.3rem;
    --button-row-item-min-inline-size: 6.8rem;
  }
}

@media (max-width: 48rem) {
  me {
    --_nav-shell-padding: 0.5rem 0.65rem;
    --_nav-brand-wrap-size: 2.25rem;
    --_nav-brand-mark-size: 1.85rem;
    position: static;
    top: auto;
    --_nav-link-font-size: var(--text-size-meta-xs);
    margin-top: var(--space-1);
    margin-bottom: var(--space-1);
  }

  me > [data-nav] {
    grid-template-columns: 1fr;
    padding: var(--_nav-shell-padding);
    border-radius: var(--ui-radius-md-inset);
    gap: var(--space-1);
  }

  me [data-nav-brand] {
    justify-content: flex-start;
  }

  me [data-nav-list='primary'] {
    display: flex;
    flex-wrap: wrap;
    grid-column: auto;
    justify-content: flex-start;
    row-gap: var(--_nav-primary-row-gap-mobile);
  }

  me [data-nav-trailing] {
    display: contents;
  }

  me [data-nav-list='primary'] [data-nav-link-label='full'] {
    display: none;
  }

  me [data-nav-list='primary'] [data-nav-link-label='compact'] {
    display: inline;
  }

  me [data-nav-auth-text] {
    max-inline-size: min(42vw, 12rem);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  me [data-nav-brand-text] {
    display: none;
  }

  me [data-nav-link][aria-current="page"] {
    box-shadow: inset 0 1px 0 var(--surface-edge-default);
  }

  me [data-nav-list='auth'] {
    grid-column: auto;
    gap: var(--_nav-auth-gap-mobile);
  }

  me [data-nav-guest-auth] {
    width: 100%;
  }

  me [data-nav-guest-auth] .ui-button-row {
    width: 100%;
    --button-row-grid-template: repeat(2, minmax(0, 1fr));
  }

  me [data-nav-list='meta'] {
    display: none;
  }
}

@media (max-width: 38rem) {
  me [data-nav-list='primary'] li[data-nav-link-item-kind='external'] {
    display: none;
  }

  me [data-nav-list='auth'] {
    align-items: center;
    gap: var(--space-1);
    padding-top: 0;
    justify-self: start;
    justify-content: flex-start;
    border-top: none;
  }

  me [data-nav-auth-text] {
    flex-basis: auto;
    max-inline-size: min(44vw, 10rem);
    justify-content: flex-start;
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-gap: var(--space-1);
  }
}

@media (max-width: 26rem) {
  me > [data-nav][data-nav-layout='split'] {
    grid-template-columns: 1fr;
  }

  me > [data-nav][data-nav-layout='split'] [data-nav-trailing] {
    display: flex;
    padding-top: var(--space-1);
    border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
    justify-content: stretch;
  }

  me > [data-nav][data-nav-layout='split'] [data-nav-list='auth'] {
    grid-template-columns: 1fr;
    justify-items: stretch;
  }

  me > [data-nav][data-nav-layout='split'] [data-nav-auth-switch] {
    width: 100%;
    text-align: center;
  }

  me [data-nav-list='primary'] {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    width: 100%;
    column-gap: var(--space-1);
    row-gap: var(--_nav-primary-row-gap-mobile);
  }

  me [data-nav-list='primary'] [data-nav-link-label='full'] {
    display: inline;
  }

  me [data-nav-list='primary'] [data-nav-link-label='compact'] {
    display: none;
  }

  me [data-nav-list='primary'] li {
    min-width: 0;
  }

  me [data-nav-list='primary'] [data-nav-link] {
    display: flex;
    justify-content: center;
    width: 100%;
    text-align: center;
  }

  me [data-nav-list='auth'] {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    width: 100%;
    gap: var(--space-1);
    justify-self: stretch;
    align-items: center;
  }

  me [data-nav-list='auth'] li,
  me [data-nav-list='auth'] form,
  me [data-nav-list='auth'] [data-nav-link],
  me [data-nav-list='auth'] [data-button] {
    min-width: 0;
    width: 100%;
  }

  me [data-nav-auth-text] {
    grid-column: auto;
    justify-content: flex-start;
    max-inline-size: none;
  }

  me [data-nav-guest-auth] .ui-button-row {
    width: auto;
    --button-row-grid-template: 1fr;
  }

  me [data-nav-list='auth'] [data-nav-link] {
    padding-inline: calc(var(--control-padding-inline-compact) - 0.1rem);
  }

  me [data-nav-auth-action] {
    --_button-padding-inline: calc(var(--control-padding-inline-compact) - 0.1rem);
  }

  me [data-nav-auth-prefix],
  me [data-nav-create-account-action],
  me [data-nav-account-item] {
    display: none;
  }

  me [data-nav-guest-auth] {
    width: auto;
  }
}

@media (max-width: 23rem) {
  me [data-nav-auth-prefix] {
    display: none;
  }

  me [data-nav-guest-auth] .ui-button-row {
    --button-row-grid-template: 1fr;
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
