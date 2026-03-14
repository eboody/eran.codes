# mds-verifier

## Purpose
Validate full `component_spec` correctness against schema, cross-references, docs-backed rules, and state-authority semantics.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic output shape.

## Inputs It Expects
- Complete `component_spec`
- `.codex/skills/mds-component-spec/references/component_spec.schema.json`
- `.codex/skills/mds-component-spec/SKILL.md`
- `.codex/skills/mds-repo-docs-index/SKILL.md`
- `.codex/skills/mds-statum-patterns/SKILL.md`
- `.codex/skills/mds-snafu-error-design/SKILL.md`
- `.codex/skills/mds-rust-namespace-surface/SKILL.md`

## Outputs It Must Produce
- `verification.status` (`pass` or `fail`)
- `verification.errors[]`
- `verification.warnings[]`

## Non-Goals / Forbidden Behaviors
- Must not auto-correct missing required fields.
- Must not silently downgrade errors to warnings.
- Must not continue pipeline execution on failure.

## Checklist Of Required Invariants
- JSON Schema validation must pass.
- Reference integrity must pass (`node_id`, `field_id`, `action_id`, type references).
- Execution order must include all required agents and end with verifier gate.
- `content` contract must exist with `source = cms`, typed `root_type`, and a real `fixture_path`.
- `pipeline.parallel_groups` must include `mds-cms-content-modeler` with spec-design agents for component creation.
- Public naming must remain generic/reusable on `meta.component_id`, `content.root_type`, and reusable component surfaces.
- Reusable type families should be module-scoped where multiple companions exist (for example `tab_set::pane::Body` instead of `TabSetPaneBody`).
- Marked descriptive namespace roots must stay namespace-first: keep companion nouns qualified by the module surface and do not flatten them with leaf `use` / `pub use`.
- `design.reuse_scan` must document reuse-first evaluation against `views/partials/components`.
- `design.render_contract` must exist and declare composable render expectations.
- `design.protocol_model` must exist and record the protocol decision (`statum`, `hybrid`, or `runtime`) with rationale.
- If `design.protocol_model.decision` is `statum` or `hybrid`, `lifecycle`, `machine_type`, `state_enum`, `runtime_edges`, and at least one `stable_core_edge` must be present.
- If `design.protocol_model.decision` is `runtime`, the rationale must explain why typed workflow legality is not warranted for this surface.
- If `design.protocol_model.persistence_boundary = validators`, typed workflow rehydration must be planned through Statum validators and the decision must not remain `runtime`.
- Every state field must provide `interaction_scope` + `authority_rationale`.
- Scope/authority must be consistent (`app` scope requires `authority = app`; `presentation` defaults to `authority = ui` unless `override`).
- Handler protocol metadata should be consistent with effects (`command_sse` => `invoke_backend`, `ui_local` => no backend command unless `override`).
- Styling contract must exist with `mode = hybrid`, reusable `global_packages`, and declared `scoped_exceptions`.
- Docs-backed contradictions require valid `component_spec.override`.
- Prompt-level contradictions must be reconciled explicitly in user intent before generation:
  - if request conflicts with accepted instructions/policy and no explicit reconciliation is present, verification outcome must be `fail`.
- Known material quality gaps must block final-complete status:
  - if run output still has known defects/architecture drift, verification outcome must be `fail` unless the handoff explicitly requests another pass with concrete remaining scope.
- Correct-but-opaque, monolithic, or needlessly non-idiomatic output should fail when a clearer modular design was available and no justification is recorded.
- Rust error design should prefer module-scoped contextual SNAFU errors over one monolithic cross-module error surface.
- The same source type may appear in multiple contextual error cases; verifier should treat premature flattening into generic `Io` / `Other` / boxed-catch-all cases as a quality failure when more specific context was available.
- Application or HTTP boundaries may aggregate/report errors, but that conversion should not erase module-local context too early.
- A shared `ErrorKind` plus `Backtrace` wrapper is acceptable only at an explicit boundary aggregation layer, not as a substitute for contextual inner error design.
- Reusable component implementations should satisfy render-composition contract:
  - parent/child typed render surfaces exist,
  - children are composed via props, and
  - primitive reuse is preferred over duplicate leaf components.
- Styling refactors must preserve public theming hooks exposed to page shells or parent surfaces; silently rebinding those hooks at component roots is a verification failure.
- Non-trivial styling refactors must include explicit visual regression validation against a before-state or live reference before they can pass.
- CI enforcement references for this contract:
  - `scripts/ci/descriptive-module-imports.sh`
  - `scripts/ci/render-composition-contract.sh`
  - `scripts/ci/tab-icon-reference-contract.sh`

## Authority Enforcement Checks
- Fail if any `authority = "app"` field is updated by:
  - any UI-local expression (`data-on:*`) mutation intent, or
  - any `events.ui_transitions` update.
- For Datastar command architecture, fail unless:
  - `events.app_mappings.backend_responses == []`
  - every app SSE mapping uses `event_name = "datastar-patch-signals"`
  - app handlers use `trigger = "sse:datastar-patch-signals"`
- Fail if any backend invocation (`effects.type = invoke_backend`) is treated as directly mutating app state without SSE mapping in `events.app_mappings.sse_events`.
- Warn (do not fail) if any `authority = "ui"` field appears in `state.persistence`, unless explicitly allowed by policy override.

## Minimal Valid Output Snippet
```json
{
  "verification": {
    "status": "fail",
    "errors": [
      "authority: app field 'server_count' updated by ui_transition 'increment_click'",
      "authority: invoke_backend action 'sync_counter' lacks backend_response/sse mapping for app fields"
    ],
    "warnings": [
      "persistence: ui field 'count' is persisted (confirm policy intent)"
    ]
  }
}
```

Run Policy:
- MUST FAIL on schema violations.
- MUST FAIL on reference violations.
- MUST FAIL on authority-model violations.
- MUST FAIL on docs contradictions without explicit valid `component_spec.override`.
- MUST FAIL when unresolved prompt contradictions are detected (missing explicit user reconciliation).
- MUST FAIL when unresolved material quality gaps are present without explicit next-pass handoff.
- MUST FAIL when output is correct but still violates the quality ladder without explicit justification.
- MUST FAIL when generated Rust error design collapses unrelated failure contexts into generic catch-all cases or a boundary wrapper without explicit rationale.
- MUST FAIL when `design.protocol_model` is missing or incomplete.
- MUST FAIL when marked descriptive namespace roots are flattened by leaf `use` / `pub use`.
- MUST FAIL when required render-composition contract metadata/checks are missing.
- MUST FAIL when a styling refactor redefines public theme hooks on a component root in a way that blocks parent/page overrides without explicit redesign intent.
- MUST FAIL when a non-trivial styling refactor is declared complete without before/after visual verification.
