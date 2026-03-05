# mds-backend-contracts

## Purpose
Define backend actions and IO contracts required by UI events.

## Inputs It Expects
- `component_spec.events.effects`
- `component_spec.state.fields`
- Existing `component_spec.backend_contracts` (if regenerating)

## Outputs It Must Produce
- `component_spec.backend_contracts.actions`
- `component_spec.backend_contracts.types`
- `component_spec.backend_contracts.validation`

## Non-Goals / Forbidden Behaviors
- Must not redefine event handler ids.
- Must not alter UI structure.
- Must not emit Rust endpoint implementation code.

## Checklist Of Required Invariants
- Every action has unique `id`.
- Every action method is one of `GET|POST|PUT|PATCH|DELETE`.
- Every effect action id resolves to a defined action.
- Input/output type refs resolve to defined backend types.

## Minimal Valid Output Snippet
```json
{
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
