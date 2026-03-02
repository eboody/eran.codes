---
name: visual-snapshot-check
description: Run Playwright-based full-page screenshot capture and baseline checks after visual/UI changes in Maud views or CSS.
---

# Visual Snapshot Check

## When To Use

Use this skill whenever UI styling, component markup, or visual behavior changes in:
- `crates/http/src/views/**`
- `crates/http/static/**`

## Local Sources First

Start from `docs/reference-map.md` and review:
- `docs/visual-signoff/latest/ux-signoff.md`
- `docs/visual-signoff/latest/ui-signoff.md`
- `docs/visual-signoff/latest/signoff.env`

## Workflow

1. Load local docs from `docs/reference-map.md` and capture signoff expectations.
2. Ensure the app is running and reachable at `VISUAL_URL` (default `http://127.0.0.1:3000/`).
3. If the visual change is intentional, baseline refresh is required (do not skip):
   - `VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh`
4. Run the visual check:
   - `scripts/ci/visual-snapshot.sh`
5. If check fails, inspect:
   - current screenshot: `artifacts/visual/current/home.png`
   - baseline screenshot: `artifacts/visual/baseline/home.png`
6. Finalize only after explicitly reporting baseline status:
   - `updated` when visuals intentionally changed and baseline was refreshed
   - `unchanged` when no visual change was intended
7. Record UX/UI signoff artifacts for visual commits:
   - `artifacts/visual/audits/latest/ux-signoff.md`
   - `artifacts/visual/audits/latest/ui-signoff.md`
   - `artifacts/visual/audits/latest/signoff.env`
8. Run the visual signoff gate:
   - `scripts/ci/visual-expert-signoff.sh`

## Notes

- Screenshot capture is implemented by the Rust Playwright binary:
  - `cargo run -p utils --bin visual_snapshot -- ...`
- The checker exits non-zero on mismatch so it is safe for CI/local guardrails.
- The signoff gate must fail if either UX/UI artifact is missing, not `pass`, or marks agent unavailable.
- Final response should include `sources_used` with exact local files consulted.
