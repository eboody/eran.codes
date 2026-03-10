# mds-state-modeler

## Purpose
Define reactive state model, initial values, and derived fields used by Datastar interactions.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic state shape.

## Inputs It Expects
- `component_spec.meta`
- `component_spec.scope`
- `component_spec.ui.bindings`
- `component_spec.design.protocol_model`
- Existing `component_spec.state` (if regenerating)

## Outputs It Must Produce
- `component_spec.state.fields`
- `component_spec.state.derived`
- `component_spec.state.persistence`

## Non-Goals / Forbidden Behaviors
- Must not define UI hierarchy.
- Must not define backend transport contracts.
- Must not emit handler code.
- Must not encode stable Rust workflow or API protocol legality as plain UI/app state when a Statum or hybrid protocol model should own it.

## Checklist Of Required Invariants
- Every state field has unique `id`.
- Field types are explicit and valid.
- Every field declares `interaction_scope` (`presentation|session|app`).
- Every field declares `authority_rationale` describing why chosen authority/protocol is appropriate.
- `interaction_scope = app` must use `authority = app`.
- Derived formulas reference only declared field ids.
- Persistence keys are unique.
- When `design.protocol_model.decision` is `statum` or `hybrid`, UI/app state should expose only projections or inputs needed by the interface; stable workflow phases belong in the typed protocol model.
- When `design.protocol_model.decision` is `runtime`, the rationale should explain why typed workflow legality is unnecessary for this surface.
- Keep state and derived-field design explicit enough that future changes stay local instead of forcing one large coupled state surface.

## Minimal Valid Output Snippet
```json
{
  "state": {
    "fields": [
      {
        "id": "display_name",
        "type": "string",
        "initial": "",
        "authority": "ui",
        "interaction_scope": "presentation",
        "authority_rationale": "local input state"
      }
    ],
    "derived": [
      {"id": "display_name_len", "type": "number", "formula": "len(display_name)"}
    ],
    "persistence": [
      {"field_id": "display_name", "storage": "session"}
    ]
  }
}
```
