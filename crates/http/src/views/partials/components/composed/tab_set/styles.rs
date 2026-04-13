crate::views::scoped::inline_css!(
    r#"
me.tab-set-showcase {
  --_tab-set-panel-gap: clamp(1rem, 0.85rem + 0.8vw, 1.75rem);
  --_tab-set-tab-padding: var(--control-padding-block) var(--control-padding-inline);
  --_tab-set-shell-padding: clamp(0.95rem, 0.82rem + 0.55vw, 1.3rem);
  --_tab-set-code-stack-gap: clamp(0.8rem, 0.72rem + 0.35vw, 1.05rem);
  --_tab-set-badge-padding: 0.35rem 0.65rem;
  --_tab-set-feature-item-padding: 0.7rem 0.85rem;
  --_tab-set-mobile-subtitle-line-height: 1.55;

  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 38%, transparent),
      transparent 44%
    ),
    var(--surface-panel);
  position: relative;
  overflow: clip;
}

me .tab-set__tabs {
  display: flex;
  gap: var(--space-2);
  position: relative;
  isolation: isolate;
  padding-block: var(--interactive-bleed) calc(var(--space-3) + var(--interactive-bleed));
  padding-inline: var(--interactive-bleed);
  margin: calc(var(--interactive-bleed) * -1);
  overflow-x: auto;
  border-bottom: var(--_tab-set-tab-border-width) solid var(--border-default);
  scrollbar-width: thin;
  scroll-snap-type: x proximity;
  overflow-y: visible;
}

me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] {
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
  padding: 0;
  margin: 0;
  overflow: visible;
  border-bottom: none;
  scroll-snap-type: none;
}

me .tab-set__tab {
  appearance: none;
  display: inline-flex;
  align-items: center;
  gap: var(--control-gap);
  margin: 0;
  padding: var(--_tab-set-tab-padding);
  border-radius: var(--ui-radius-md-inset);
  border: var(--control-border-width) solid var(--border-default);
  background: color-mix(in srgb, var(--surface-field) 78%, var(--surface-panel));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  color: var(--text-muted);
  cursor: pointer;
  font: inherit;
  font-size: var(--control-font-size-compact);
  font-weight: var(--control-font-weight);
  line-height: var(--control-line-height);
  letter-spacing: var(--control-letter-spacing);
  white-space: var(--control-white-space);
  position: relative;
  z-index: 0;
  scroll-snap-align: start;
  transition:
    border-color var(--motion-fast),
    background-color var(--motion-fast),
    color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

me .tab-set__tab.is-selected {
  color: var(--text-body);
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 42%,
    var(--border-default)
  );
  background: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 10%,
    var(--surface-panel)
  );
  box-shadow:
    inset 0 1px 0 var(--surface-edge-soft),
    0 0 0 1px color-mix(in srgb, var(--tab-accent, var(--accent-signal)) 22%, transparent);
}

me .tab-set__tab:focus-visible {
  outline: none;
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 54%,
    var(--border-default)
  );
  box-shadow:
    0 0 0 0.22rem color-mix(
      in srgb,
      var(--tab-accent, var(--accent-signal)) 18%,
      transparent
    ),
    inset 0 1px 0 var(--surface-edge-default);
}

me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab {
  width: fit-content;
  justify-content: center;
  padding:
    calc(var(--control-padding-block-compact) + var(--space-1) * 0.25)
    calc(var(--control-padding-inline-compact) + var(--space-1) * 0.5);
  border-radius: var(--radius-pill);
  background: color-mix(in srgb, var(--surface-field) 84%, var(--surface-panel));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  color: var(--text-muted);
  font-size: var(--text-size-body-xs);
}

me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab.is-selected {
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 34%,
    var(--border-default)
  );
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--tab-accent, var(--accent-signal)) 18%, transparent),
      transparent 70%
    ),
    color-mix(
      in srgb,
      var(--surface-panel) 92%,
      color-mix(in srgb, var(--tab-accent, var(--accent-signal)) 14%, transparent)
    );
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    0 0 0 1px color-mix(
      in srgb,
      var(--tab-accent, var(--accent-signal)) 10%,
      transparent
    );
}

me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab:focus-visible {
  border-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 54%,
    var(--border-default)
  );
  box-shadow:
    0 0 0 0.22rem color-mix(
      in srgb,
      var(--tab-accent, var(--accent-signal)) 18%,
      transparent
    ),
    inset 0 1px 0 var(--surface-edge-default);
}

me .tab-set__tab-icon {
  --control-icon-size: var(--space-4);

  display: var(--control-inline-display);
  align-items: var(--control-inline-align-items);
}

me .tab-set__tab-label {
  display: grid;
  gap: calc(var(--space-1) * 0.2);
  text-align: left;
}

me .tab-set__tab-line {
  font-weight: 600;
}

me .tab-set__tab-secondary {
  font-size: var(--text-size-label-sm);
  font-weight: 500;
  line-height: var(--text-line-flat);
  letter-spacing: var(--text-track-label);
  color: color-mix(in srgb, currentColor 74%, var(--text-muted));
}

me .tab-set__preview {
  min-width: 0;
}

me .tab-set__panel {
  display: grid;
  gap: var(--_tab-set-panel-gap);
  align-items: start;
  grid-template-columns: minmax(0, 1.08fr) minmax(0, 0.92fr);
  min-width: 0;
  padding-block-start: var(--space-2);
}

me .tab-set__panel > * {
  min-width: 0;
}

me .tab-set__preview-frame {
  overflow: visible;
  border: var(--control-border-width) solid
    color-mix(in srgb, var(--accent-signal-soft) 28%, var(--border-default));
  border-radius: var(--ui-radius-md-inset);
  min-height: 260px;
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 34%, transparent),
      transparent 42%
    ),
    color-mix(in srgb, var(--surface-field) 82%, var(--surface-panel));
  padding: var(--_tab-set-shell-padding);
  display: grid;
  gap: var(--space-2);
  align-content: start;
  box-shadow:
    inset 0 1px 0 var(--surface-edge-default),
    inset 0 0 0 1px color-mix(in srgb, black 2%, transparent);
}

me .tab-set__preview-frame[data-preview-kind="code"] {
  min-height: 0;
  gap: var(--space-3);
  border-color: color-mix(
    in srgb,
    var(--accent-signal-soft) 18%,
    var(--border-default)
  );
  background: color-mix(in srgb, var(--surface-panel) 92%, transparent);
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me.tab-set-showcase--flat-gallery {
  background: transparent;
}

me.tab-set-showcase--flat-gallery .tab-set__tabs {
  gap: var(--space-2);
  padding-block-end: calc(var(--space-2) + var(--interactive-bleed));
  padding-inline: 0;
  margin-inline: 0;
  border-bottom-color: color-mix(in srgb, var(--border-default) 48%, transparent);
}

me.tab-set-showcase--flat-gallery .tab-set__panel {
  gap: clamp(1.35rem, 1.12rem + 0.85vw, 2rem);
  grid-template-columns: minmax(0, 1.28fr) minmax(22rem, 0.72fr);
}

me.tab-set-showcase--flat-gallery .tab-set__copy {
  gap: var(--space-4);
  align-content: start;
  max-inline-size: 31rem;
}

me.tab-set-showcase--flat-gallery .tab-set__copy h2 {
  font-size: var(--text-size-title-md);
  line-height: var(--text-line-heading);
}

me.tab-set-showcase--flat-gallery .tab-set__preview-frame[data-preview-kind="code"] {
  min-height: 0;
  padding: clamp(1.2rem, 1.02rem + 0.7vw, 1.6rem);
  border: var(--control-border-width) solid
    color-mix(in srgb, var(--accent-signal-soft) 18%, var(--border-default));
  border-radius: var(--ui-radius-md-inset);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 12%, transparent),
      transparent 36%
    ),
    color-mix(in srgb, var(--surface-panel) 88%, var(--surface-field));
  box-shadow: inset 0 1px 0 var(--surface-edge-soft);
}

me.tab-set-showcase--flat-gallery .tab-set__preview-meta {
  gap: var(--space-2);
}

me.tab-set-showcase--flat-gallery .tab-set__preview-code-stack {
  gap: var(--space-4);
}

me.tab-set-showcase--flat-gallery .ui-code-block {
  --_code-block-font-size: var(--text-size-body-md);
  --_code-block-padding: clamp(1rem, 0.9rem + 0.4vw, 1.35rem);
}

me.tab-set-showcase--flat-gallery .tab-set__tab {
  padding-block: var(--space-2);
  padding-inline: var(--space-2);
  border-width: 0 0 var(--control-border-width);
  border-style: solid;
  border-color: transparent transparent color-mix(in srgb, var(--border-default) 40%, transparent);
  border-radius: 0;
  background: transparent;
  box-shadow: none;
  font-size: var(--text-size-body-xs);
}

me.tab-set-showcase--flat-gallery .tab-set__tab.is-selected {
  border-bottom-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 52%,
    var(--border-default)
  );
  background: transparent;
  box-shadow: none;
}

me.tab-set-showcase--flat-gallery .tab-set__tab:focus-visible {
  border-bottom-color: color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 58%,
    var(--border-default)
  );
  box-shadow: 0 0 0 0.18rem color-mix(
    in srgb,
    var(--tab-accent, var(--accent-signal)) 16%,
    transparent
  );
}

me.tab-set-showcase--flat-gallery .tab-set__features {
  margin-block: var(--space-1) 0;
  padding: 0;
  list-style: none;
  display: grid;
  gap: var(--space-2);
  border-top: none;
}

me.tab-set-showcase--flat-gallery .tab-set__features li {
  padding: var(--_tab-set-feature-item-padding);
  border: var(--control-border-width) solid
    color-mix(in srgb, var(--accent-signal-soft) 18%, var(--border-default));
  border-radius: var(--ui-radius-md-inset);
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-signal-soft) 16%, transparent),
      transparent 100%
    ),
    color-mix(in srgb, var(--surface-field) 82%, var(--surface-panel));
  color: var(--text-muted);
  font-size: var(--text-size-body-sm);
  line-height: var(--text-line-summary);
}

me .tab-set__preview-meta {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-2);
}

me .tab-set__preview-code-stack {
  display: grid;
  gap: var(--_tab-set-code-stack-gap);
}

me .tab-set__preview-code {
  margin: 0;
}

me .tab-set__features {
  margin-block: var(--space-4) 0;
  padding-block-start: var(--space-3);
  padding-inline-start: var(--space-4);
  display: grid;
  gap: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--border-default) 80%, transparent);
}

me .tab-set__preview-label {
  margin: 0;
  font-size: var(--text-size-label-2xs);
  letter-spacing: var(--text-track-caps-wide);
  text-transform: uppercase;
  color: var(--text-muted);
}

me .tab-set__preview-asset {
  margin: 0;
  font-size: var(--control-font-size);
  font-weight: 600;
}

me .tab-set__badge {
  margin: 0;
  width: fit-content;
  border-radius: var(--radius-pill);
  padding: var(--_tab-set-badge-padding);
  border: var(--control-border-width) solid var(--border-subtle);
  font-size: var(--text-size-meta-xs);
  color: var(--text-muted);
  background: var(--ui-surface-soft);
}

me .tab-set__copy {
  display: grid;
  gap: var(--space-2);
}

me .tab-set__copy h2 {
  margin: 0;
  font-size: var(--text-size-title-md);
  line-height: var(--text-line-control);
  letter-spacing: var(--text-track-tight);
  text-wrap: balance;
}

me .tab-set__subtitle {
  margin: 0;
  color: var(--text-muted);
  max-width: 52ch;
}

@media (prefers-color-scheme: dark) {
  me.tab-set-showcase {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 26%),
      color-mix(in srgb, var(--surface-panel) 95%, black 5%);
  }

  me .tab-set__preview-frame {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 22%),
      color-mix(in srgb, var(--surface-field) 95%, black 5%);
    box-shadow: inset 0 1px 0 var(--surface-edge-soft);
  }

  me .tab-set__preview-frame[data-preview-kind="code"] {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 20%),
      color-mix(in srgb, var(--surface-panel) 96%, black 4%);
  }

  me .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 90%, black 10%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me .tab-set__tab.is-selected {
    background: color-mix(in srgb, var(--accent-signal) 8%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent-signal) 24%, var(--border-default));
  }

  me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab {
    background: color-mix(in srgb, var(--surface-field) 92%, black 8%);
    border-color: color-mix(in srgb, var(--border-default) 90%, transparent);
  }

  me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab.is-selected {
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 44%),
      color-mix(
        in srgb,
        var(--tab-accent, var(--accent-signal)) 14%,
        var(--surface-raised)
      );
    border-color: color-mix(
      in srgb,
      var(--tab-accent, var(--accent-signal)) 40%,
      var(--border-default)
    );
  }

  me .tab-set__badge {
    background: color-mix(in srgb, var(--surface-field) 94%, black 6%);
  }
}

@media (hover: hover) {
  me .tab-set__tab:hover {
    transform: translateY(-1px);
    z-index: 1;
  }

  me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab:hover {
    transform: translateY(-1px);
  }

  me.tab-set-showcase--flat-gallery .tab-set__tab:hover {
    transform: none;
    color: var(--text-body);
  }
}

@media (max-width: 980px) {
  me .tab-set__panel {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 48rem) {
  me.tab-set-showcase {
    --_tab-set-panel-gap: var(--space-2);
    --_tab-set-tab-padding:
      calc(var(--control-padding-block) - 0.08rem)
      calc(var(--control-padding-inline) - 0.16rem);
    --_tab-set-shell-padding: var(--control-padding-inline-compact);
    --_tab-set-code-stack-gap: var(--space-2);
    --_tab-set-badge-padding: 0.28rem 0.55rem;
  }

  me .tab-set__tabs {
    gap: var(--space-1);
    padding-block: var(--interactive-bleed) calc(var(--space-1) + var(--interactive-bleed));
    flex-wrap: wrap;
    overflow-x: visible;
    scroll-snap-type: none;
  }

  me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] {
    gap: var(--space-1);
  }

  me .tab-set__tab {
    font-size: var(--text-size-meta-xs);
    min-width: 0;
    flex: 1 1 10rem;
  }

  me .tab-set__tabs[data-tab-set-tabs-style='pill-cluster'] .tab-set__tab {
    padding: var(--control-padding-block-compact) var(--control-padding-inline-compact);
    flex: 0 1 auto;
  }

  me .tab-set__panel {
    gap: var(--space-3);
    padding-top: 0;
  }

  me.tab-set-showcase--flat-gallery .tab-set__tabs {
    gap: var(--space-1);
    padding-block-end: calc(var(--space-1) + var(--interactive-bleed));
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 7rem), 1fr));
    align-items: stretch;
  }

  me.tab-set-showcase--flat-gallery .tab-set__tab {
    padding-block: var(--space-1);
    padding-inline: calc(var(--space-1) * 0.75);
    justify-content: center;
    text-align: center;
  }

  me.tab-set-showcase--flat-gallery .ui-code-block {
    --_code-block-font-size: var(--text-size-meta-xs);
  }

  me.tab-set-showcase--flat-gallery .tab-set__panel {
    gap: var(--space-2);
    grid-template-columns: 1fr;
  }

  me.tab-set-showcase--flat-gallery .tab-set__copy {
    order: -1;
    max-inline-size: none;
  }

  me.tab-set-showcase--flat-gallery .tab-set__copy h2 {
    font-size: var(--text-size-title-sm);
  }

  me .tab-set__preview-frame {
    min-height: 10.5rem;
    gap: var(--space-1);
  }

  me .tab-set__preview-frame[data-preview-kind="code"] {
    gap: var(--space-2);
  }

  me .tab-set__copy {
    gap: var(--space-1);
  }

  me .tab-set__copy h2 {
    font-size: var(--text-size-title-sm);
  }

  me .tab-set__subtitle {
    font-size: var(--text-size-body-md);
    line-height: var(--_tab-set-mobile-subtitle-line-height);
  }

  me .tab-set__preview-meta {
    gap: var(--space-1);
  }

  me .tab-set__preview-code-stack > :not(:first-child) {
    display: none;
  }

  me .tab-set__features {
    margin-block: var(--space-3) 0;
    padding-block-start: var(--space-2);
    padding-inline-start: var(--space-3);
    gap: var(--space-1);
    display: none;
  }

  me .tab-set__badge {
    font-size: var(--text-size-label-sm);
  }
}

@media (max-width: 23rem) {
  me.tab-set-showcase--flat-gallery .tab-set__tabs {
    grid-template-columns: 1fr;
  }

  me .tab-set__tab {
    flex-basis: 100%;
  }
}
"#
);

pub(super) fn render() -> maud::Markup {
    css()
}
