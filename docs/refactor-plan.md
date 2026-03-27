# Resume Alignment Packaging Status (Historical)

As of March 23, 2026, the resume-first content unification, the IA-hardening pass, the supporting-proof route-policy pass, and the archive-collapse pass are all shipped.

This file is no longer the active forward plan for the site.

Use [Presentation Verification Plan](./presentation-tightening-plan.md) for the current execution plan.

This file remains as the historical status note for the packaging phases that followed the secure-data proof slice.

## What Changed

- The public site now reads from one checked-in content directory at `crates/http/.../site_content/`.
- `/` is now the primary resume narrative instead of a portfolio-first landing page.
- `/resume.txt` now derives from the same authored source as the public site.
- Repeated public CTAs now resolve through shared action references and shared action bundles instead of being hand-authored in many places.
- Portfolio nav now exposes both `Live Proof` and `Current Proof` as first-class paths.
- `/work` now leads with the current secure-data case, keeps the older case studies in an explicit supporting-proof archive section, and renders their detailed archive blocks in place.
- Legacy `/work/*` pages now permanently redirect to `/work#...` archive anchors instead of acting like parallel primary narratives.
- Content validation now rejects unresolved action refs and invalid internal-route targets.

## What This Phase Improved

- Reviewer flow is more explicit: `/`, `/lab`, and `/work/sensitive-sync` are now the intended evaluation path.
- The shared content root is less drift-prone because common links and repeated CTA patterns now resolve from named references.
- Supporting-proof routes are now intentionally secondary instead of competing with the current flagship proof, and the old leaf pages no longer keep reviewers off the canonical path.
- The authored content surface is easier to maintain because the old `site_content.json` monolith was replaced with split fragment files and fragment-level validation.
- The repo docs no longer need to describe the shared-content migration as future work.

## What Still Remains Optional

- Additional cleanup of the shared content model if future edits reveal awkward patterns that the current action-ref layer does not cover.
- A stronger runtime-proof branch later if resume alignment needs deeper authorization or integration evidence instead of packaging changes.

## Recommended Next Decision

Do not continue broad packaging work by inertia.

Pick one focused branch:

- incremental content-root ergonomics cleanup if real editing pain shows up
- a new runtime-proof branch such as key rotation or a narrower real-integration pilot
