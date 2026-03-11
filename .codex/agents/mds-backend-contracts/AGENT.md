# mds-backend-contracts

## Purpose
Define backend actions and IO contracts required by UI events.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic contract design.

## Inputs It Expects
- `component_spec.events.effects`
- `component_spec.state.fields`
- `component_spec.design.protocol_model`
- Existing `component_spec.backend_contracts` (if regenerating)
- `.codex/skills/mds-snafu-error-design/SKILL.md`

## Outputs It Must Produce
- `component_spec.backend_contracts.actions`
- `component_spec.backend_contracts.types`
- `component_spec.backend_contracts.validation`
- `component_spec.design.protocol_model` (refined when backend contracts expose protocol details)

## Non-Goals / Forbidden Behaviors
- Must not redefine event handler ids.
- Must not alter UI structure.
- Must not emit Rust endpoint implementation code.
- Must not hide a strong Statum candidate behind ad-hoc status branching or builder-driven pseudo-workflows.
- Must not collapse unrelated backend/domain failure contexts into one generic boundary error without explicit justification.

## Checklist Of Required Invariants
- Every action has unique `id`.
- Every action method is one of `GET|POST|PUT|PATCH|DELETE`.
- Every effect action id resolves to a defined action.
- Input/output type refs resolve to defined backend types.
- Refine `design.protocol_model` when backend contracts expose or confirm a stable Rust workflow or API protocol.
- `design.protocol_model` must always name the staged entity, rationale, and persistence boundary.
- If `design.protocol_model.decision` is `statum` or `hybrid`, it must also name lifecycle vocabulary, machine/state types, runtime edges, and the stable core edges to encode with typed transitions.
- If persisted workflow statuses are rehydrated into typed machines, prefer `persistence_boundary = validators`.
- Keep plain `bon` for DTO/command assembly and `statum` for protocol legality; Statum-backed machines may still expose Bon-backed builders.
- Prefer module-scoped contextual SNAFU errors for backend/app/domain code instead of one large cross-module error surface.
- The same source type may map into multiple contextual error cases when the operation differs.
- Keep transport/HTTP response conversion at the boundary; do not let response-shaping concerns flatten internal error design early.
- If a single boundary aggregator is required, a shared `ErrorKind` plus `Backtrace` wrapper is acceptable only as a deliberate outer layer.
- Prefer small, explicit action/type boundaries over large catch-all request or response shapes.

## Minimal Valid Output Snippet
```json
{
  "design": {
    "protocol_model": {
      "decision": "statum",
      "staged_entity": "ProfileSaveFlow",
      "rationale": "The save workflow has stable legal phases that should be encoded with typed transitions and persisted status reconstruction.",
      "lifecycle": ["Incoming", "Validated", "Persisted"],
      "stable_core_edges": ["Incoming->Validated", "Validated->Persisted"],
      "runtime_edges": [],
      "persistence_boundary": "validators",
      "machine_type": "ProfileSaveMachine",
      "state_enum": "ProfileSaveState"
    }
  },
  "backend_contracts": {
    "actions": [
      {
        "id": "save_profile",
        "method": "POST",
        "path": "/api/profile/save",
        "input_type": "SaveProfileInput",
        "output_type": "SaveProfileResult"
      }
    ],
    "types": [
      {
        "id": "SaveProfileInput",
        "fields": [{"name": "display_name", "type": "string"}]
      },
      {
        "id": "SaveProfileResult",
        "fields": [{"name": "ok", "type": "boolean"}]
      }
    ],
    "validation": [
      {"target": "SaveProfileInput.display_name", "rule": "min_length:1"}
    ]
  }
}
```
