# mds-ui-decomposer

## Purpose
Define structural UI composition for Maud output and Datastar binding points.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic structure.

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
- Prefer existing reusable view components from `crates/http/src/views/partials/components` before creating new structures.
- Public-facing ids/slot names should be generic/reusable (`tab_item`, `tab_set`, `card_header`) rather than request-specific names.
- Composition structure should model parent/child render components, not a single monolithic renderer.
- Child component slots should be designed to accept typed component props (`Vec<TChild>`/slices) where repetition exists.
- Prefer module-scoped type families in generated surfaces (`tab_set::pane::Body`, `application::Service`) instead of long prefixed standalone names.
- Avoid dense node trees or slot shapes that are technically valid but harder to extend or reason about.

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
