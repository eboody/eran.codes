# Style System

This directory is the repo-local source of truth for styling decisions in the Maud UI layer.

## Defaults

- Preserve and standardize the current visual language unless a task explicitly asks for redesign.
- Keep styles scoped to the component by default.
- Promote styles into `crates/http/static/app.css` only when they become shared `ui-*` packages.
- Treat Open Props as the primitive layer and workspace semantic aliases as the authored API.
- Treat typography the same way: repeated font sizes, line heights, and tracking values belong in semantic tokens, not as repeated component literals.
- Treat raw design values as token-source material, not declaration-site defaults. Shared scales belong in `app.css`; component-specific numbers belong behind a small number of local aliases only when they earn their keep.
- Avoid alias farms and one-off selectors. Add a local alias only when it clarifies repeated meaning, powers a real override, or defines a component-owned metric, and prefer existing `u-*`/`ui-*` packages or local `data-*` hooks over inventing throwaway classes.
- Prefer modern native CSS features where they improve clarity and are reasonable for the task's browser targets.
- Use small local controllers for DOM-local presentation behavior and Datastar only when the interaction belongs in the component contract.

## Workflow

1. Inspect the component markup, surrounding composed components, `app.css`, and the relevant styling spec or fixture.
2. Classify the task as local polish, shared package work, theme/token work, design audit, or migration cleanup.
3. Apply the scope rules in `promotion-rules.md`.
4. Apply the token rules in `token-layers.md`.
5. Run a literal sweep for touched styling declarations; move repeated or direct rhythm/type numbers into shared tokens or small local aliases, and delete aliases or selectors that only rename a single use without adding structure.
6. Choose modern native CSS features with the rules in `modern-css.md`.
7. Validate with `component-checklist.md`.
8. Update specs and CI-facing documentation when shared styling behavior changes.

## Outputs

For non-trivial styling work, include:
- scope decision
- token decision
- shared package changes
- scoped CSS changes
- interaction styling notes
- validation results

Use `repo-map.md` for the files to inspect first.
