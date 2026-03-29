reviewer: ui-expert
agent_status: available
result: blocked
reviewed_at: 2026-03-28
scope: route-matrix
sitewide_status: blocked
snapshot: artifacts/visual/current/presentation-matrix
component: route-matrix

notes:
- Fresh screenshots now exist for the stable guest matrix and the main signed-in proof routes.
- `/`, `/work`, `/work/sensitive-sync`, `/lab`, `/login`, and `/register` were rechecked from captured renders on March 28, 2026.
- Full-site visual signoff is still blocked by the open-source route, which still mismatches the committed desktop-light baselines in both guest and signed-in states.
- The signed-in `/` route renders the portfolio home flow; earlier capture failures were assertion drift, not a missing page.
- `signoff.env` now mirrors the blocked route-matrix status in this docs directory; the older component-only pass is historical, not current full-site truth.
