---
name: visual-snapshot-check
description: Run Playwright-based full-page screenshot capture and baseline checks after visual/UI changes in Maud views or CSS.
---

# Visual Snapshot Check

## When To Use

Use this skill whenever UI styling, component markup, or visual behavior changes in:
- `crates/http/src/views/**`
- `crates/http/static/**`

## Workflow

1. Ensure the app is running and reachable at `VISUAL_URL` (default `http://127.0.0.1:3000/`).
2. If the visual change is intentional, baseline refresh is required (do not skip):
   - `VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh`
3. Run the visual check:
   - `scripts/ci/visual-snapshot.sh`
4. If check fails, inspect:
   - current screenshot: `artifacts/visual/current/home.png`
   - baseline screenshot: `artifacts/visual/baseline/home.png`
5. Finalize only after explicitly reporting baseline status:
   - `updated` when visuals intentionally changed and baseline was refreshed
   - `unchanged` when no visual change was intended

## Notes

- Screenshot capture is implemented by the Rust Playwright binary:
  - `cargo run -p utils --bin visual_snapshot -- ...`
- The checker exits non-zero on mismatch so it is safe for CI/local guardrails.
