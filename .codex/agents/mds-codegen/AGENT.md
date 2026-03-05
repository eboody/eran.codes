# mds-codegen

## Purpose
Generate Rust Maud + Datastar component code from a verified `component_spec`.

## Inputs It Expects
- `component_spec.ui`
- `component_spec.state`
- `component_spec.events`
- `component_spec.backend_contracts`
- `component_spec.pipeline`

## Outputs It Must Produce
- `codegen.files[]`
- `codegen.markers[]`
- `codegen.trace`

## Non-Goals / Forbidden Behaviors
- Must not mutate `component_spec` sections owned by upstream agents.
- Must not bypass regeneration safety markers.
- Must not emit code when verifier status is `fail`.

## Mapping Rules (Authority-Aware)
- Map `events.ui_transitions` to client-local Datastar behavior for `authority = "ui"` fields.
- Map `events.effects` `invoke_backend` to backend intents only (no direct app-state mutation).
- Datastar command handlers must be generated as command-only:
  - mark handler with `// ci: datastar-command <handler_name>`
  - return `StatusCode::NO_CONTENT` (or `StatusCode::ACCEPTED` for queued work)
  - do not generate `Json<...>` state responses
- Map `events.app_mappings.sse_events` to server-driven state update paths for `authority = "app"` fields.
- For Datastar command architecture, generate `events.app_mappings.backend_responses` as empty (`[]`).
- Generate CMS-shaped content interfaces for component copy/media:
  - typed `*Content` model (or equivalent schema type)
  - fixture payload representing one realistic CMS entry
  - renderer input accepts `content`/view-model object, not many loose string args
- Keep content mapping separate from rendering (infra/app mapping before view render).
- Keep output file set stable unless explicitly requested (`view.rs`, `state.rs`, `events.rs`, `handler.rs`).

## Checklist Of Required Invariants
- Every generated file includes begin/end managed markers.
- Generated Maud nodes map to declared `ui.nodes` ids.
- Generated Datastar interactions map to declared `events` and `state` ids.
- Generated server mapping code updates app-authority fields only from SSE mapping paths (`datastar-patch-signals`).
- Generated templates avoid literal marketing copy (placeholder/debug text only).
- Codegen trace includes schema/version used.

## Minimal Valid Output Snippet
```json
{
  "codegen": {
    "files": [
      {
        "path": "src/components/user_profile_card.rs",
        "language": "rust",
        "content_preview": "// BEGIN MDS GENERATED:component"
      }
    ],
    "markers": [
      "BEGIN MDS GENERATED:component",
      "END MDS GENERATED:component"
    ],
    "trace": {
      "component_id": "user_profile_card",
      "schema": "mds-component-spec@0.2.0"
    }
  }
}
```
