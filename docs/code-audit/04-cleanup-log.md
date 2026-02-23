# Cleanup Log

This log tracks code removed or simplified as part of this audit pass.

## Removed files

- `crates/http/src/views/partials/demo/layout/flow_map.rs`
- `crates/http/src/views/partials/demo/layout/support_card.rs`
- `crates/http/src/views/partials/demo/layout/highlights_section.rs`

## Updated files

- `crates/http/src/views/partials/demo/layout/mod.rs`
  - Removed module declarations for deleted layout partials.
- `crates/http/src/views/partials/demo/layout/tabbed_showcase.rs`
  - Removed unused opposite-hue payload and related struct field.
- `crates/http/static/app.css`
  - Removed unused selectors related to deleted layout modules.

## Verification

- `cargo check --all-targets`
- `cargo check -p http@0.1.0`
- `cargo test -p http@0.1.0 --test home_demos`

All passed after cleanup.
