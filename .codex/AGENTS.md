# MDS Multi-Agent System

This repository uses a project-local multi-agent system for generating Rust Maud + Datastar UI components from a shared `component_spec`.

## Execution Order

1. `mds-orchestrator`
2. `mds-docs-librarian`
3. Parallel spec-design phase:
   - `mds-ui-decomposer`
   - `mds-cms-content-modeler`
   - `mds-state-modeler`
   - `mds-events-designer`
   - `mds-backend-contracts`
4. `mds-codegen`
5. `mds-verifier` (stop-the-line gate)

## Routing Rules

- Run `mds-orchestrator` first for every new component request.
- Run `mds-docs-librarian` immediately after orchestration to collect docs-backed rules for decision areas in scope.
- Run `mds-cms-content-modeler` for component creation requests; it defines `component_spec.content` and fixture contract.
- Run `mds-ui-decomposer` when UI structure is missing or changed.
- Run `mds-state-modeler` when state, derived values, or data flow is missing or changed.
- Run `mds-events-designer` when event triggers, payloads, or transitions are missing or changed.
- Run `mds-backend-contracts` when server endpoints, actions, or validation constraints are missing or changed.
- Run `mds-codegen` only after `content`, `ui`, `state`, `events`, and `backend_contracts` are valid and internally linked.
- Run `mds-verifier` last on every run. Verification is mandatory before accepting output.

## Docs-First Decision Rules

- Agents must consult docs skills before deciding:
  - Datastar bindings
  - JS/browser event selection and wiring
  - Backend contracts
  - SSE transport behavior
  - Axum routing decisions
- Required skills for docs lookup:
  - `.codex/skills/mds-repo-docs-index/SKILL.md`
  - `.codex/skills/mds-maud-patterns/SKILL.md`
  - `.codex/skills/mds-datastar-architecture/SKILL.md`
  - `.codex/skills/mds-datastar-patterns/SKILL.md`
  - `.codex/skills/mds-axum-integration/SKILL.md`
  - `.codex/skills/mds-css-conventions/SKILL.md`
- Docs under `/docs` are evergreen source of truth and override agent priors.

## Stop-The-Line Verification Gate

- `mds-verifier` validates `component_spec` against:
  - `.codex/skills/mds-component-spec/references/component_spec.schema.json`
  - Cross-reference invariants in `.codex/skills/mds-component-spec/SKILL.md`
- The run must fail if:
  - Any required field is missing.
  - Any reference points to a non-existent id.
  - Any event references undefined state or backend action ids.
  - Any `component_spec` decision contradicts docs-backed rules and `component_spec.override` is missing or invalid.
  - Generated output omits required file markers.
- If verification fails, no downstream publish/accept step is allowed.

## Regeneration Safety Rules

- Generated code blocks must include markers:
  - `BEGIN MDS GENERATED:<section>`
  - `END MDS GENERATED:<section>`
- Agents may regenerate only inside managed marker blocks.
- Agents must never overwrite user-edited content outside managed markers.
- If markers are absent in an expected generated file, agents must fail and request explicit user confirmation/workflow for first-time generation.
- Agents must preserve manual imports, helper functions, and comments outside generated blocks.

## Default Component Policy (Strict)

This policy applies by default when a user asks to create/build/add a component (or equivalent intent).

- Default workflow:
  - Phase 1: produce decomposition + component/spec contracts only.
  - Phase 2: implement after spec approval.
  - Phase 3: harden with tests, accessibility checks, and CI proof commands.
- Default Datastar architecture:
  - Command endpoints mutate server state.
  - Datastar commands return `204 NO CONTENT` (or `202 ACCEPTED` when queued).
  - Datastar commands must not return JSON state payloads.
  - App-authority state converges via the global `/events` SSE stream.
  - SSE event name for app-state convergence is `datastar-patch-signals`.
  - Datastar component specs use `backend_responses: []`.
  - Choose protocol by interaction scope:
    - `presentation`/`session` interactions default to `ui_local`.
    - `app` interactions use command + SSE.
  - Tabs/selectors are presentation-state by default unless explicit app-level semantics are requested.
- Default component decomposition:
  - Split new UI into orchestrator/container, child feature components, and shared primitives.
  - Do not collapse into one monolithic component unless explicitly requested.
  - Reuse existing primitives/composites from `crates/http/src/views/partials/components` first.
  - If no fit exists, add a new reusable component there before introducing feature-specific view fragments.
  - Use generic, library-style public names for reusable surfaces (for example `tabs_panel`, `tab_item`, `card_header`), not screenshot-specific names.
  - Include `design.reuse_scan` in `component_spec` to record checked/reused/created reusable components.
- Default CMS-shaped content model:
  - Assume component copy/images/features/CTAs/tabs come from CMS content, not inline literals.
  - `component_spec` must include a top-level `content` contract (`source: "cms"`, typed `root_type`, and `fixture_path`).
  - Define a typed `*Content` schema first (required/optional, enums for variants, asset refs for media).
  - Add a realistic fixture entry and treat it as the source of truth during development.
  - Keep component APIs content-shaped (`content: *Content`, optional small `ui: *UiState`), not many loose string props.
  - Separate content mapping from rendering:
    - Infra fetches CMS payload.
    - App validates/maps payload into `*Content` / view model.
    - HTTP/View renders only from the view model.
  - Rule: no literal marketing copy in templates except placeholder/debug text.
  - Keep public naming generic on:
    - `meta.component_id`
    - `content.root_type`
    - reusable component files/types under `views/partials/components`
    - top-level slot names and root-level UI node ids
- Opt-out is explicit:
  - Only bypass this default when the user clearly asks for `single component`, `skip spec`, or `end-to-end in one pass`.
