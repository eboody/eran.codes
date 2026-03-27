# Demo Surface Backlog

Use [Portfolio Demo Concepts](./portfolio-demos.md) for the current shipped demo catalog.

This file tracks follow-on demo ideas.

Use [Presentation Verification Plan](./presentation-tightening-plan.md) for the current execution plan.

The current site plan lives elsewhere. Keep this note for post-shipped demo work after the secure-data slice.

## Current Shipped Anchors

- Sensitive sync proof on `/lab#sensitive-proof` and `/work/sensitive-sync`
- Identity and session durability through `/register`, `/login`, and `/protected`
- Trust-boundary and error-shaping proof in the layered runtime
- Inspectable runtime behavior through trace, network, and SSE surfaces
- Supporting proof from live chat and the request-burst harness

## Useful Follow-On Demo Work

### 1. Sensitive-read audit visibility

- Show when the persisted-grant authorized-read path was used and when denied reads were recorded.
- Keep the proof surface explicit about guest-safe views versus decrypted views.

### 2. Sync failure and recovery visibility

- Add a reviewer-visible way to inspect failed refresh or sync runs.
- Make recovery behavior inspectable without relying only on log output.

### 3. Stronger access-control proof

- If the resume later needs it, add a deeper authorization slice beyond authenticated-versus-guest access.
- Keep this separate from the content refactor so the scope stays understandable.

### 4. Tighter cross-linking between actions and traces

- Make it easier to jump from a sensitive-proof action to its related request or trace entries.
- Improve proof readability without inventing a new runtime claim.

## Non-Goals

- Do not add real vendor integrations just to make the demo feel larger.
- Do not add PHI-flavored fixtures.
- Do not restate the current shipped demo catalog here; keep that in [Portfolio Demo Concepts](./portfolio-demos.md).
