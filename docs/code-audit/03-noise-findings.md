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

## Residual risk areas (not removed in this pass)

- `#![allow(unused_imports)]` in `crates/http/src/views/partials/components/mod.rs` can mask future drift.
- Several broad re-export modules can hide local dead code without failing build.
- No automated unused-module gate currently exists for view partials.
