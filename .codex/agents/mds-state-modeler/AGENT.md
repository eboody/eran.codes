# mds-state-modeler

## Purpose
Define reactive state model, initial values, and derived fields used by Datastar interactions.

## Inputs It Expects
- `component_spec.meta`
- `component_spec.scope`
- `component_spec.ui.bindings`
- Existing `component_spec.state` (if regenerating)

## Outputs It Must Produce
- `component_spec.state.fields`
- `component_spec.state.derived`
- `component_spec.state.persistence`

## Non-Goals / Forbidden Behaviors
- Must not define UI hierarchy.
- Must not define backend transport contracts.
- Must not emit handler code.

## Checklist Of Required Invariants
- Every state field has unique `id`.
- Field types are explicit and valid.
- Every field declares `interaction_scope` (`presentation|session|app`).
- Every field declares `authority_rationale` describing why chosen authority/protocol is appropriate.
- `interaction_scope = app` must use `authority = app`.
- Derived formulas reference only declared field ids.
- Persistence keys are unique.

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
