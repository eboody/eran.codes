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
- `component_spec.content` (CMS contract shell for downstream agents)

## Non-Goals / Forbidden Behaviors
- Must not define UI nodes, state fields, events, backend contracts, or code output details.
- Must not skip verifier execution.
- Must not mutate sections owned by other agents after ownership is assigned.

## Checklist Of Required Invariants
- `meta.component_id` is stable and unique.
- `meta.component_id` is generic/reusable (not screenshot- or campaign-specific wording).
- `meta.target` includes `rust-maud` and `datastar`.
- `pipeline.execution_order` matches orchestration policy.
- `pipeline.required_agents` includes all mds agents.
- `pipeline.parallel_groups` includes `mds-cms-content-modeler` with other spec-design agents.
- `pipeline.execution_order` includes `mds-styling-system` after `mds-codegen` and before `mds-verifier`.
- `content.source` is `cms`.
- `content.root_type` is typed `*Content`.
- `content.fixture_path` points to a fixture path to be materialized by downstream generation.
- `design.reuse_scan` is present and records which reusable components were evaluated/reused/created.

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
  "content": {
    "source": "cms",
    "root_type": "UserProfileCardContent",
    "fixture_path": "tests/fixtures/cms/user_profile_card.json"
  },
  "design": {
    "reuse_scan": {
      "checked_components": [
        "crates/http/src/views/partials/components/tab.rs"
      ],
      "reused": [
        "tab"
      ],
      "created": []
    }
  },
  "pipeline": {
    "execution_order": [
      "mds-orchestrator",
      "mds-docs-librarian",
      "mds-ui-decomposer",
      "mds-cms-content-modeler",
      "mds-state-modeler",
      "mds-events-designer",
      "mds-backend-contracts",
      "mds-codegen",
      "mds-styling-system",
      "mds-verifier"
    ],
    "required_agents": [
      "mds-orchestrator",
      "mds-docs-librarian",
      "mds-ui-decomposer",
      "mds-cms-content-modeler",
      "mds-state-modeler",
      "mds-events-designer",
      "mds-backend-contracts",
      "mds-codegen",
      "mds-styling-system",
      "mds-verifier"
    ],
    "parallel_groups": [
      [
        "mds-ui-decomposer",
        "mds-cms-content-modeler",
        "mds-state-modeler",
        "mds-events-designer",
        "mds-backend-contracts"
      ]
    ]
  }
}
```
