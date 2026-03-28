# Presentation Verification Plan

Status: historical.

Superseded as the active presentation branch by
[Presentation Remediation Plan](./presentation-remediation-plan.md) on March 28,
2026.

This document still matters as the signed-in smoke and visual-regression
hardening plan that followed the larger IA and presentation cleanup pass. It is
no longer the full active presentation roadmap.

Last updated: March 27, 2026.

The presentation-cleanup phase is effectively shipped.

The site now has the intended reviewer path:

- `/` states the thesis
- `/lab` proves it live
- `/work/sensitive-sync` carries the flagship case
- `/work` is supporting archive
- `/open-source` is code-level proof

The next project is not another broad cleanup pass.

The next project is to protect that presentation from drift, especially across
guest and signed-in states, mobile and desktop breakpoints, and repo-level
smoke coverage.

## Goal

Make visual quality observable and enforceable instead of review-only.

That means:

- the guest experience stays clean across the stable proof routes
- the signed-in experience stays equally disciplined instead of becoming the new noisy path
- the repo ships screenshot automation and baseline policy that match the current site, not stale selectors or one-off manual inspection

## What Is Already Done

- The reviewer path and IA were tightened.
- The public nav was reduced to the proof path.
- Home, lab, work, work current-proof, open-source, login, and register were all simplified.
- Guest mobile nav now uses a balanced proof-path layout instead of a cramped overflow pattern.
- Signed-in mobile nav no longer leaves a hidden-account gap that forces `Sign out` onto a second row.
- The portfolio browser smoke baselines for the stable guest routes were refreshed to the new presentation.

This plan starts where that work stops.

## Current Status

The first branch is now implemented in the repo:

- the visual snapshot tool can create a real signed-in browser session
- signed-in smoke coverage exists for the stable proof routes
- signed-in baseline refreshes are normalized and documented
- the Docker runtime and publish path can exercise that signed-in mode on demand

The remaining decision is narrower:

- keep signed-in smoke opt-in while it hardens further, or
- promote it into the default required gate once the route set and failure rate stay stable

The implementation branch also exposed one separate repo reality:

- the signed-in browser smoke now passes through the Docker runtime gate
- the full publish gate still inherits unrelated existing `repo-checks` policy failures outside this branch

## Main Problem

The current repo protection is still incomplete in one important way:

- smoke coverage is guest-first
- signed-in visual states still rely too much on manual spot checks
- the current ad hoc CDP scripts are useful for investigation, but they are not yet a repo-owned verification surface

So the risk has shifted.

It is no longer “the pages are too noisy.”

It is now “the pages are clean, but signed-in and future-breakpoint drift can reintroduce noise without CI catching it early enough.”

## Execution Order

1. Freeze the stable signed-in review matrix.
   Pick the routes and states that are stable enough for automation. The likely baseline set is `/lab`, `/work`, `/work/sensitive-sync`, and `/open-source`, each at desktop and mobile widths, signed in through the real browser flow.

2. Define the signed-in setup contract.
   Decide one truthful way to reach the signed-in state for visual checks. Prefer a browser-native register or login flow over synthetic DOM mutation so the screenshots reflect the real auth path.

3. Move signed-in capture out of ad hoc scripts.
   Replace temporary one-off probes with a repo-owned script or script extension that can create a session, normalize dynamic text, and capture signed-in screenshots deterministically.

4. Define dynamic-text normalization for signed-in surfaces.
   Normalize usernames, timestamps, request IDs, and any other volatile text that would otherwise make the signed-in snapshots flaky.

5. Add a signed-in smoke mode before making it required.
   Start with a separate signed-in smoke target or opt-in matrix so the path can stabilize without weakening the existing guest gate.

6. Decide which signed-in surfaces deserve baselines.
   Keep stable routes pixel-locked. Keep volatile routes assertion-only if they still contain runtime-heavy surfaces that should not be baseline compared.

7. Extend the browser smoke policy docs.
   Update the docs hub and release-gate notes so the repo explains which visual states are protected, how to refresh them intentionally, and which routes remain volatility exceptions.

8. Tighten failure messages and operator ergonomics.
   Make sure visual failures tell the maintainer what drifted: guest route, signed-in route, breakpoint, selector contract, or baseline mismatch.

9. Re-run the publish gate with the new coverage.
   The signed-in visual checks should be exercised through the same downstream confidence path as the existing guest smoke once they are stable enough.

10. Stop when visual regression ownership is explicit.
    The plan is complete when future presentation drift is caught by the repo without requiring another manual screenshot-polish loop.

## Acceptance Criteria

- Stable guest routes remain covered by committed baselines.
- Stable signed-in routes gain deterministic screenshot coverage.
- Signed-in mobile nav and account-state chrome are protected by automation, not memory.
- Visual smoke assertions reference the current page contracts instead of historical selectors.
- Baseline refresh instructions are documented and intentional.
- The publish gate can fail on real signed-in presentation regressions without depending on temporary local scripts.

## Non-Goals

- Do not reopen the thesis, IA, or route hierarchy without a new product decision.
- Do not redesign the pages again just because signed-in coverage is being added.
- Do not baseline volatile runtime surfaces only to create noisy CI.
- Do not turn temporary inspection scripts into a second unowned toolchain; either promote them cleanly into the repo or retire them.

## Suggested Sequence

- Phase 1: codify the signed-in capture contract and route matrix
- Phase 2: add signed-in screenshot tooling with normalization
- Phase 3: wire the new checks into smoke or matrix mode
- Phase 4: update docs and release-gate instructions
- Phase 5: decide whether signed-in visual coverage is stable enough to make required by default

## First Branch

Start with three things only:

- add a repo-owned signed-in screenshot path for the stable proof routes
- normalize signed-in dynamic text so captures are deterministic
- expose that path through the existing visual smoke workflow without breaking the guest baseline path

That branch is the real continuation of the presentation work.

It protects the cleaned-up site instead of reopening it.
