---
name: mds-component-spec
version: 0.2.0
description: JSON schema and contracts for component_spec, including ui/app authority, transitions, mappings, and codegen markers.
scope: project
---

# mds-component-spec

## Purpose
Define and govern the shared `component_spec` contract used by all MDS agents.

## Canonical Schema
- Path: `.codex/skills/mds-component-spec/references/component_spec.schema.json`
- Schema dialect: JSON Schema Draft 2020-12

## Required Top-Level Sections
- `meta`
- `scope`
- `pipeline`
- `content`
- `ui`
- `state`
- `events`
- `backend_contracts`

## CMS Content Contract (Required)
Every generated component spec must declare a CMS-shaped content contract:
- `content.source` must be `cms`.
- `content.root_type` must be a typed root ending in `Content` (for example, `HeroCarouselContent`).
- `content.fixture_path` must point at a checked-in fixture used during generation/development.

Semantics:
- Copy/images/features/CTAs/tabs come from `content.root_type`, not inline literals in templates.
- Subcomponents should consume only the content slice they own.

## Optional Governance Section
- `override`
  - Purpose: explicit exception metadata when generated decisions intentionally contradict docs-backed rules.
  - Required fields when present: `reason`, `approved_by`, `sources`.

## State Authority Model
Each `state.fields[]` entry must define:
- `authority`: `ui` or `app`
- Optional `sync`: `none` | `optimistic` | `authoritative`

Semantics:
- `authority = "ui"`
  - May be mutated by Datastar local expressions (`data-on:*`) and by `events.ui_transitions`.
- `authority = "app"`
  - MUST NOT be mutated by local `data-on:*` expressions.
  - MUST NOT be mutated by `events.ui_transitions`.
  - May change only via `events.app_mappings.backend_responses` or `events.app_mappings.sse_events`.

## Events Split
`events` is explicitly split into:
- `handlers[]`: semantic triggers (`class = ui | app`).
- `ui_transitions[]`: client-local reducers for UI-authority fields.
- `app_mappings.backend_responses[]`: server response to app-state mapping.
- `app_mappings.sse_events[]`: SSE event to app-state mapping.
- `effects[]`: intents (for example, backend invocation). Intents do not directly mutate app state.

## UI Dispatch Convention
`ui.event_dispatch` must document dispatch syntax, for example:
- `syntax`: `@dispatch('<handler_id>')`

This declares semantic dispatch style. Projects may still use local Datastar expressions for `authority = "ui"` fields.

## Section Ownership
- `mds-orchestrator`: `meta`, `scope`, `pipeline`
- `mds-cms-content-modeler`: `content`
- `mds-ui-decomposer`: `ui`
- `mds-state-modeler`: `state`
- `mds-events-designer`: `events`
- `mds-backend-contracts`: `backend_contracts`
- `mds-codegen`: `codegen`
- `mds-verifier`: `verification`

## Cross-Reference Invariants
- `ui.bindings[].node_id` must exist in `ui.nodes[].id`.
- `events.handlers[].source_node_id` must exist in `ui.nodes[].id`.
- `events.ui_transitions[].updates[].field_id` must exist in `state.fields[].id`.
- `events.app_mappings.backend_responses[].updates[].field_id` must exist in `state.fields[].id`.
- `events.app_mappings.sse_events[].updates[].field_id` must exist in `state.fields[].id`.
- `events.effects[].action_id` must exist in `backend_contracts.actions[].id` when `type == "invoke_backend"`.
- `backend_contracts.actions[].input_type` and `output_type` must exist in `backend_contracts.types[].id`.
- `pipeline.execution_order` must include all required `mds-*` agents.
- `pipeline.parallel_groups` must include `mds-cms-content-modeler` in the same group as other spec-design agents for component creation.
- Docs-backed rules from `/docs` must be honored unless `override` is present and valid.

## Docs Policy
- `/docs` is evergreen source of truth.
- Docs skills must be consulted for Datastar bindings, JS events, backend contracts, SSE, and routing decisions.
- If output contradicts docs-backed rules and `override` is absent or invalid, verifier must fail.

## Verifier Failure Policy
`mds-verifier` must fail the run if any schema, reference, authority, or docs contradiction invariant fails.

## Minimal Example Skeleton
```json
{
  "meta": {"component_id": "example", "version": "0.1.0", "target": ["rust-maud", "datastar"]},
  "scope": {"description": "example"},
  "pipeline": {"execution_order": ["mds-orchestrator", "mds-docs-librarian", "mds-ui-decomposer", "mds-cms-content-modeler", "mds-state-modeler", "mds-events-designer", "mds-backend-contracts", "mds-codegen", "mds-verifier"], "required_agents": ["mds-orchestrator", "mds-docs-librarian", "mds-ui-decomposer", "mds-cms-content-modeler", "mds-state-modeler", "mds-events-designer", "mds-backend-contracts", "mds-codegen", "mds-verifier"], "parallel_groups": [["mds-ui-decomposer", "mds-cms-content-modeler", "mds-state-modeler", "mds-events-designer", "mds-backend-contracts"]]},
  "content": {"source": "cms", "root_type": "ExampleContent", "fixture_path": "tests/fixtures/cms/example.json"},
  "ui": {"event_dispatch": {"syntax": "@dispatch('<handler_id>')", "description": "semantic event dispatch"}, "nodes": [], "slots": [], "bindings": []},
  "state": {"fields": [{"id": "local_count", "type": "integer", "initial": 0, "authority": "ui", "sync": "optimistic"}, {"id": "server_count", "type": "integer", "initial": 0, "authority": "app", "sync": "authoritative"}], "derived": [], "persistence": []},
  "events": {"handlers": [], "ui_transitions": [], "app_mappings": {"backend_responses": [], "sse_events": []}, "effects": []},
  "backend_contracts": {"actions": [], "types": [], "validation": []}
}
```
