# mds-ui-decomposer

## Purpose
Define structural UI composition for Maud output and Datastar binding points.

## Inputs It Expects
- `component_spec.meta`
- `component_spec.scope`
- Existing `component_spec.ui` (if regenerating)

## Outputs It Must Produce
- `component_spec.ui.nodes`
- `component_spec.ui.slots`
- `component_spec.ui.bindings`

## Non-Goals / Forbidden Behaviors
- Must not define state transition logic.
- Must not define backend endpoint details.
- Must not emit Rust code.
- Must not hardcode final marketing copy in UI node attrs/content design; model content slots for typed `*Content` input.

## Checklist Of Required Invariants
- Every UI node has unique `id`.
- `parent_id` references are valid or null for root.
- Every binding references an existing node id.
- Slot names are unique per component.
- UI structure can be populated from CMS-shaped `*Content` rather than fixed inline strings.

## Minimal Valid Output Snippet
```json
{
  "ui": {
    "nodes": [
      {
        "id": "root",
        "parent_id": null,
        "tag": "section",
        "attrs": {"class": "profile-card"}
      }
    ],
    "slots": [
      {"name": "header", "required": true}
    ],
    "bindings": [
      {"node_id": "root", "kind": "datastar-signal", "path": "profile"}
    ]
  }
}
```
