# Noise Findings

## Confirmed redundant or unused code

### A) Unused layout modules

These modules were present under `crates/http/src/views/partials/demo/layout/` but were not referenced by current page composition:

- `flow_map.rs`
- `support_card.rs`
- `highlights_section.rs`

Corresponding style selectors were also unused:

- `.flow-map`, `.flow-map .step`, `.flow-map .arrow`
- `.highlights article`, `.highlights ul`

### B) Stale theme variable path

The tabbed showcase previously computed and passed `--tab-opposite-h` without active CSS consumers after selection-style changes. This was dead state and visual-noise risk.

## Areas reviewed and intentionally retained

- `professionalism-breakdown.md` doc file (outside runtime path) retained for historical/reference value.
- Existing component exports in `partials/mod.rs` retained where still consumed by pages/handlers.
- Theme token set in `app.css` retained because it now normalizes light/dark behavior for custom sections.

## Additional confirmed noise (follow-up pass)

### C) Unused domain type and stale suppressions

- `crates/domain/src/user/new_user.rs` was exported but not used anywhere in workspace flows.
- `#[allow(unused)]` markers in:
  - `crates/app/src/user/mod.rs`
  - `crates/infra/src/repo/user.rs`
- `#![allow(unused_imports)]` in `crates/http/src/views/partials/components/mod.rs`

### D) Over-broad re-exports and dead styling hook

- `crates/http/src/views/partials/demo/misc/mod.rs` and `crates/http/src/views/partials/components/mod.rs` were exporting symbols not consumed by current call sites.
- `PillColor` and inline `--pill-accent` styling path in `crates/http/src/views/partials/demo/misc/pill.rs` had one active value path and extra unused variants.
