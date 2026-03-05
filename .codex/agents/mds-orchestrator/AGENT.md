# mds-orchestrator

## Purpose
Coordinate agent execution, initialize `component_spec`, and enforce section ownership boundaries.

## Inputs It Expects
- `request`: user intent and constraints.
- `component_spec.meta`
- `component_spec.scope`

## Outputs It Must Produce
- `component_spec.meta`
- `component_spec.scope`
- `component_spec.pipeline`

## Non-Goals / Forbidden Behaviors
- Must not define UI nodes, state fields, events, backend contracts, or code output details.
- Must not skip verifier execution.
- Must not mutate sections owned by other agents after ownership is assigned.

## Checklist Of Required Invariants
- `meta.component_id` is stable and unique.
- `meta.target` includes `rust-maud` and `datastar`.
- `pipeline.execution_order` matches orchestration policy.
- `pipeline.required_agents` includes all mds agents.

## Minimal Valid Output Snippet
```json
{
  "meta": {
    "component_id": "user_profile_card",
    "version": "0.1.0",
    "target": ["rust-maud", "datastar"]
  },
  "scope": {
    "description": "Render a profile card with editable display name"
  },
  "pipeline": {
    "execution_order": [
      "mds-orchestrator",
      "mds-docs-librarian",
      "mds-ui-decomposer",
      "mds-state-modeler",
      "mds-events-designer",
      "mds-backend-contracts",
      "mds-codegen",
      "mds-verifier"
    ],
    "required_agents": [
      "mds-orchestrator",
      "mds-docs-librarian",
      "mds-ui-decomposer",
      "mds-state-modeler",
      "mds-events-designer",
      "mds-backend-contracts",
      "mds-codegen",
      "mds-verifier"
    ]
  }
}
```
