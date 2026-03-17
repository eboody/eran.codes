// ci: descriptive-module-import crate::views::partials::components::composed::button
mod attr;
mod component;
mod role;
mod row;
mod variant;

const STYLES: &str = r#"
.button[data-button] {
  --_button-padding-block: var(--control-padding-block);
  --_button-padding-inline: var(--control-padding-inline);
  --_button-font-size: var(--control-font-size);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--control-gap);
  margin: var(--control-margin);
  padding-block: var(--_button-padding-block);
  padding-inline: var(--_button-padding-inline);
  border: var(--control-border-width) solid var(--control-border-color-default);
  border-radius: var(--control-radius);
  font: var(--control-font);
  font-family: var(--font-ui);
  font-size: var(--_button-font-size);
  font-weight: var(--control-font-weight);
  line-height: var(--control-line-height);
  letter-spacing: var(--control-letter-spacing);
  text-decoration: none;
  position: relative;
  z-index: 0;
  white-space: var(--control-white-space);
  color: var(--ui-text-on-accent);
  background: var(--ui-accent-primary);
  box-shadow:
    inset 0 1px 0 var(--control-edge-accent),
    0 10px 24px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
  transition:
    background-color var(--motion-fast),
    color var(--motion-fast),
    border-color var(--motion-fast),
    box-shadow var(--motion-fast),
    transform var(--motion-fast);
}

button.button[data-button] {
  appearance: none;
}

.button.secondary[data-button] {
  color: var(--text-strong);
  background: var(--control-fill-secondary);
  border-color: var(--border-default);
  box-shadow: inset 0 1px 0 var(--surface-edge-default);
}

@media (prefers-color-scheme: dark) {
  .button.secondary[data-button] {
    color: var(--text-body);
    background: var(--control-fill-secondary);
    border-color: color-mix(in srgb, var(--border-default) 96%, transparent);
  }
}

.button[data-button]:focus-visible {
  outline: none;
  border-color: color-mix(in srgb, var(--ui-accent-primary) 54%, var(--border-default));
  box-shadow:
    0 0 0 0.22rem color-mix(in srgb, var(--ui-accent-primary) 18%, transparent),
    inset 0 1px 0 var(--surface-edge-default);
}

@media (hover: hover) {
  .button[data-button]:hover {
    transform: translateY(-1px);
    box-shadow:
      inset 0 1px 0 var(--control-edge-accent-hover),
      0 14px 28px color-mix(in srgb, var(--ui-accent-primary) 18%, transparent);
  }

  .button.secondary[data-button]:hover {
    box-shadow:
      inset 0 1px 0 var(--surface-edge-strong),
      0 10px 18px color-mix(in srgb, #142033 8%, transparent);
  }

  .button[data-button]:hover {
    z-index: 1;
  }
}
"#;

pub(crate) fn head_styles() -> maud::Markup {
    crate::views::scoped::style(STYLES)
}

pub use attr::DataAttr;
pub use component::Button;
pub use role::Role;
pub use row::Row;
pub use variant::Variant;
