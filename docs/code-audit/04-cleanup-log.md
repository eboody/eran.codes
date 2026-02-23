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

## Follow-up cleanup pass

### Removed files

- `crates/domain/src/user/new_user.rs`

### Updated files

- `crates/domain/src/user/mod.rs`
  - Removed `new_user` module declaration and `NewUser` re-export.
- `crates/infra/src/repo/user.rs`
  - Removed unused `count` helper and stale `#[allow(unused)]`.
- `crates/app/src/user/mod.rs`
  - Removed stale `#[allow(unused)]` on `Service`.
- `crates/http/src/views/partials/components/mod.rs`
  - Removed broad `allow(unused_imports)` and narrowed exports to consumed symbols.
- `crates/http/src/views/partials/demo/misc/mod.rs`
  - Narrowed exports to consumed symbols.
- `crates/http/src/views/partials/demo/misc/pill.rs`
  - Removed unused `PillColor` enum and inline style path; kept class-based variant rendering.
- `crates/http/static/app.css`
  - Added `.log-fields` class style to replace inline pill accent usage.

### Verification

- `RUSTC_WRAPPER= cargo check --all-targets`
- `RUSTC_WRAPPER= cargo test -p http@0.1.0 --test home_demos`

All passed after follow-up cleanup.
