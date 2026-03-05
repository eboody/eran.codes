# mds-events-designer

## Purpose
Design event triggers, payloads, and state transitions for Datastar-driven UI behavior.

## Inputs It Expects
- `component_spec.ui.nodes`
- `component_spec.state.fields`
- Existing `component_spec.events` (if regenerating)

## Outputs It Must Produce
- `component_spec.events.handlers`
- `component_spec.events.ui_transitions`
- `component_spec.events.effects`

## Non-Goals / Forbidden Behaviors
- Must not create backend endpoints.
- Must not change UI node ids.
- Must not generate final Rust/Maud code.

## Checklist Of Required Invariants
- Every event handler has unique `id`.
- `source_node_id` references an existing UI node.
- `ui_transitions.updates[].field_id` references existing state field ids.
- Handlers should declare `protocol_mode` (`ui_local|command_sse`) and `protocol_rationale`.
- `protocol_mode = command_sse` requires matching `invoke_backend` effect.
- `protocol_mode = ui_local` should not invoke backend unless explicitly justified.
- Effects reference declared handler ids.

## Minimal Valid Output Snippet
```json
{
  "events": {
    "handlers": [
      {
        "id": "save_click",
        "class": "ui",
        "source_node_id": "root",
        "trigger": "click",
        "payload": {"display_name": "$state.display_name"},
        "protocol_mode": "command_sse",
        "protocol_rationale": "save mutates canonical app state"
      }
    ],
    "ui_transitions": [
      {
        "handler_id": "save_click",
        "updates": [{"field_id": "display_name", "value": "$payload.display_name"}]
      }
    ],
    "effects": [
      {"handler_id": "save_click", "type": "invoke_backend", "action_id": "save_profile"}
    ]
  }
}
```
