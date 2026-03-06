# mds-verifier

## Purpose
Validate full `component_spec` correctness against schema, cross-references, docs-backed rules, and state-authority semantics.

## Inputs It Expects
- Complete `component_spec`
- `.codex/skills/mds-component-spec/references/component_spec.schema.json`
- `.codex/skills/mds-component-spec/SKILL.md`
- `.codex/skills/mds-repo-docs-index/SKILL.md`

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
- `design.reuse_scan` must document reuse-first evaluation against `views/partials/components`.
- Every state field must provide `interaction_scope` + `authority_rationale`.
- Scope/authority must be consistent (`app` scope requires `authority = app`; `presentation` defaults to `authority = ui` unless `override`).
- Handler protocol metadata should be consistent with effects (`command_sse` => `invoke_backend`, `ui_local` => no backend command unless `override`).
- Styling contract must exist with `mode = hybrid`, reusable `global_packages`, and declared `scoped_exceptions`.
- Docs-backed contradictions require valid `component_spec.override`.

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
