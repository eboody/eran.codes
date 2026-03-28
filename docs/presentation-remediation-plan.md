# Presentation Remediation Plan

Last updated: March 28, 2026.

Status: active.

This is the active follow-up to the presentation audit.

It supersedes [Presentation Verification Plan](./presentation-tightening-plan.md)
as the next branch of work. That earlier plan is still useful history for
signed-in smoke coverage and visual-regression ownership, but it is no longer
the whole presentation problem.

## Goal

Fix the remaining presentation debt across four areas at the same time:

- component construction and composition
- style-system cleanliness and token discipline
- runtime ownership across Datastar, local JS, and Surreal
- visual verification and docs freshness

The work is done when the public proof surfaces stay clean without depending on
tribal memory, dead presentation runtime is gone, component boundaries are
stronger, and the docs accurately describe the current repo state.

## Audit Summary

The current presentation layer is already strong in a few important ways:

- the shell/pages/partials split is real
- Datastar usage is mostly aligned with repo doctrine
- shared page composition is typed and reusable
- scoped CSS is the default instead of global spill

The remaining debt is narrower and more concrete:

- an apparently unused Surreal runtime still ships globally
- some parent components restyle child-component internals instead of using a
  cleaner public component surface
- shared primitives still bypass the semantic token layer in places
- several large composed components have grown local alias farms
- one public partial contract already fails repo CI
- the docs still overstate or misroute a few presentation surfaces

## Guardrails

- Do not redesign the site just because cleanup is happening.
- Preserve the current proof path and IA unless a separate product decision says
  otherwise.
- Use Datastar for backend-coupled interaction state.
- Keep local JS only for genuinely DOM-local behavior.
- Do not keep a runtime, route, or signal contract alive “just in case.”
- Promote repeated type, spacing, color, and motion meanings into shared tokens
  instead of cloning local aliases across components.
- Do not claim full-site visual signoff without fresh route coverage.

## Workstreams

## 1. Runtime Ownership

### Objective

Remove or justify every presentation runtime and route that ships to the user.

### Primary targets

- `crates/http/src/views/page/assets.rs`
- `crates/http/src/state.rs`
- `crates/http/src/handlers/sse/mod.rs`
- `crates/http/src/paths.rs`
- `crates/http/src/router/routes.rs`
- `crates/http/static/surreal.js`

### Tasks

- Audit every Surreal-specific route, signal, asset, and state holder.
- Confirm whether any live view still depends on:
  - `surreal.js`
  - `surrealMessage`
  - `surrealStatus`
  - `/partials/surreal-message-guarded`
  - `/partials/surreal-message-cancel`
- If no live consumer exists, remove the full Surreal presentation path.
- If a consumer does exist, document the owning component and why Datastar or a
  smaller local controller is insufficient.
- Stop loading optional runtime assets globally when only one surface needs them.

### Acceptance criteria

- No dead presentation runtime ships by default.
- Every remaining runtime asset has a live owner and documented reason to exist.
- No Surreal-specific signal or route remains without a real consumer.

## 2. Component Construction And Composition

### Objective

Make component boundaries clean enough that parent surfaces do not need to style
child internals to get their intended result.

### Primary targets

- `crates/http/src/views/partials/components/composed/tab_set/mod.rs`
- `crates/http/src/views/partials/demo/layout/supporting_proof_tabs.rs`
- `crates/http/src/views/partials/components/composed/portfolio/sections/open_source_flow.rs`
- `crates/http/src/views/partials/components/primitives/key_value_list.rs`
- `scripts/ci/partials-render.sh`

### Tasks

- Define what the public tab-set surface is allowed to customize:
  - palette
  - spacing
  - variant class
  - extension hooks
- Move `SupportingProofTabs` off direct `.tab-set__*` restyling where possible.
- Replace parent reach-through selectors with explicit variant hooks or a shared
  tab-set variant API.
- Do the same for portfolio crate-gallery composition where parent CSS is
  styling tab-set internals.
- Fix the `KeyValueItem` contract:
  - either make it non-public if it is only a payload type
  - or make it independently renderable if it is meant to be a public partial
- Re-run and keep `scripts/ci/partials-render.sh` green.

### Acceptance criteria

- Parent components do not need to know child internal selectors unless that is
  the documented extension contract.
- The tab-set API is reusable without copy-pasted per-surface internals.
- Public partial types match the repo’s own render contract checks.

## 3. Style-System Cleanup

### Objective

Bring the live presentation back under the documented token and package rules.

### Primary targets

- `crates/http/static/app.css`
- `crates/http/src/views/partials/components/primitives/pill.rs`
- `crates/http/src/views/partials/components/composed/chat/mod.rs`
- `crates/http/src/views/partials/components/composed/portfolio/mod.rs`
- `crates/http/src/views/partials/demo/layout/home_hero.rs`
- `crates/http/src/views/partials/components/composed/portfolio/sections/open_source_flow.rs`

### Tasks

- Replace raw shared primitive colors in `Pill` with semantic token usage.
- Add missing semantic aliases in `app.css` when repeated meanings do not have a
  proper shared home yet.
- Collapse pass-through local alias farms in chat and portfolio surfaces.
- Promote repeated line-height, spacing, badge-padding, and rhythm values into
  shared tokens when the same meaning appears in multiple components.
- Keep component-local aliases only when they describe a real component metric
  or breakpoint override.
- Review shared `ui-*` packages for gaps that are forcing parent surfaces to
  invent their own local design language.

### Acceptance criteria

- Shared primitives consume semantic tokens instead of raw color literals.
- Repeated typography and spacing meanings stop reappearing as new local aliases.
- The largest composed components use fewer local aliases and clearer ownership.

## 4. Datastar, Local JS, And Interaction Boundaries

### Objective

Keep the interaction layer honest: Datastar for contract state, local JS for
DOM-local behavior, and no leftover runtime ambiguity.

### Primary targets

- `crates/http/src/views/page.rs`
- `crates/http/src/views/partials/components/composed/chat/composer.rs`
- `crates/http/src/views/partials/demo/layout/operational_request_filter.rs`
- `crates/http/src/views/partials/components/primitives/local_tab_root.rs`
- `crates/http/src/views/partials/components/primitives/local_tab_panel.rs`
- `crates/http/static/local-tabs.js`
- `crates/http/static/request-burst.js`
- `crates/http/static/chat-demo.js`
- `crates/http/src/views/partials/components/logs/primitives/auto_scroll.rs`

### Tasks

- Keep the page-level Datastar transport shell as the source of truth for:
  - SSE connection state
  - transport errors
  - request-coupled signal flow
- Inventory each remaining local script and document its ownership:
  - local tabs
  - request burst harness
  - chat autoscroll behavior
  - log autoscroll behavior
  - operational timeline scroll helper
- Remove inline scripts when a repo-owned static asset or a simpler component
  contract is cleaner.
- Do not convert local tabs or autoscroll into Datastar unless the behavior
  actually belongs in the backend-owned contract.
- Add one repo doc section that explains which behaviors are intentionally local
  controllers and why.

### Acceptance criteria

- Datastar is used for backend-coupled state and request flow, not as a generic
  local state bag.
- Local JS exists only for clearly DOM-local behavior.
- No runtime surface is both undocumented and globally loaded.

## 5. Responsiveness, Typography, And Visual QA

### Objective

Turn the current static presentation audit into route-level visual confidence.

### Route matrix

At minimum, review these at mobile and desktop widths:

- `/`
- `/lab`
- `/work`
- `/work/sensitive-sync`
- `/open-source`
- `/login`
- `/register`

Review both guest and signed-in states where the route supports both.

### Tasks

- Capture fresh screenshots for the stable proof routes.
- Confirm typography and spacing still read cleanly after token cleanup.
- Validate focus-visible, active, empty, read-only, and error states on major
  composed surfaces.
- Keep volatile runtime-heavy surfaces assertion-based where pixel baselines are
  still too noisy.
- Refresh visual signoff docs only after real screenshots and route checks, not
  from static inspection alone.

### Acceptance criteria

- The route matrix above has current visual coverage.
- Full-site presentation claims are backed by current screenshots.
- The latest visual signoff docs clearly state their scope.

## 6. Docs And Repo Freshness

### Objective

Make the docs tell the truth about what is current, what is historical, and what
still needs work.

### Tasks

- Add this remediation plan to the docs hub and reference router.
- Mark `presentation-tightening-plan.md` as historical/superseded rather than
  silently leaving it as the apparent active plan.
- Fix stale visual-signoff routing in `docs/reference-map.md`.
- Mark the latest committed visual signoff docs as component-scoped, not
  full-site presentation signoff.
- Update the Maud ecosystem note so it reflects current repo reality:
  `maud-extensions` is not currently in use here.
- Remove docs that are only historical noise if they no longer serve as useful
  history. Otherwise mark them clearly as historical.

### Acceptance criteria

- The docs hub points to the active remediation plan.
- The reference map points to the correct live docs paths.
- Historical docs are still available but no longer masquerade as active truth.

## Execution Order

1. Runtime ownership cleanup
2. Component API and composition cleanup
3. Style-system token cleanup
4. Interaction-boundary cleanup
5. Route-level visual verification
6. Final docs refresh and closeout

This order matters.

Runtime cleanup first removes dead branches before refactoring live ones.
Component API cleanup comes before styling cleanup so styling can target stable
boundaries. Visual signoff comes after both.

## Master Checklist

- [x] Confirm whether the Surreal presentation path has any live consumer.
- [x] Remove unused Surreal assets, routes, state, and signal handling if no
  consumer exists.
- [x] Document any remaining local runtime that still needs to exist.
- [x] Define an explicit public extension contract for the shared tab-set
  component.
- [x] Refactor `SupportingProofTabs` to consume that contract instead of
  restyling child internals.
- [x] Refactor open-source crate gallery composition to stop styling tab-set
  internals directly.
- [x] Fix the `KeyValueItem` public/render contract mismatch.
- [x] Keep `scripts/ci/partials-render.sh` green.
- [x] Replace raw shared primitive colors in `Pill` with semantic token usage.
- [ ] Add any missing shared semantic aliases in `app.css`.
- [ ] Collapse repeated local alias farms in chat and portfolio surfaces.
- [ ] Promote repeated typography/rhythm values into shared tokens where they
  represent the same meaning across components.
- [ ] Inventory all remaining local scripts and state their owner.
- [ ] Move inline JS into clearer repo-owned surfaces when that improves
  maintainability.
- [ ] Preserve Datastar for backend-coupled state and local JS for DOM-local
  behavior.
- [ ] Re-run route-level visual verification at guest and signed-in states.
- [ ] Refresh visual signoff docs with truthful scope notes.
- [ ] Update the docs hub and reference map to the new active plan.
- [ ] Mark historical presentation docs as superseded.
- [ ] Remove or rewrite stale docs that still imply broader signoff than the
  repo actually has.

## Done Criteria

This remediation pass is done when all of the following are true:

- no dead presentation runtime ships by default
- shared components expose cleaner public composition surfaces
- style-system token use is more centralized and less duplicated
- public docs describe current repo reality without stale “latest” claims
- route-level visual confidence exists for the stable proof path
- the repo’s presentation checks pass without relying on undocumented exceptions
