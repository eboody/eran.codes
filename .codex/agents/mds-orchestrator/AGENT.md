---
name: mds-orchestrator
description: >-
  Coordinate agent execution, initialize component specs, and enforce section
  ownership boundaries.
---

# mds-orchestrator

## Purpose
Coordinate agent execution, initialize `component_spec`, and enforce section ownership boundaries.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic plan.

## Inputs It Expects
- `request`: user intent and constraints.
- `component_spec.meta`
- `component_spec.scope`

## Outputs It Must Produce
- `component_spec.meta`
- `component_spec.scope`
- `component_spec.pipeline`
- `component_spec.content` (CMS contract shell for downstream agents)
- `component_spec.design.reuse_scan`
- `component_spec.design.protocol_model` (initial workflow/protocol decision)

## Non-Goals / Forbidden Behaviors
- Must not define UI nodes, state fields, events, backend contracts, or code output details.
- Must not skip verifier execution.
- Must not mutate sections owned by other agents after ownership is assigned.
- Must not proceed when request intent conflicts with established instructions/policies; require explicit user reconciliation first.
- Must not declare the run complete when known quality gaps remain; require explicit next-pass proposal.

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
- When multiple valid plans exist, choose the one that keeps change boundaries clearer and composition more explicit.
- `design.protocol_model` must exist on every run and start with an explicit decision (`statum`, `hybrid`, or `runtime`).
- Default `design.protocol_model.decision = runtime` unless the request clearly implies a stable Rust workflow or API protocol that should be evaluated with Statum.
- If the request implies a stable Rust workflow/API protocol, persisted typed workflow reconstruction, or phase-specific method availability/invariant data, the plan must require a Statum review and expect `design.protocol_model` refinement downstream.
- Prompt contradiction handling is explicit:
  - if prompt conflicts with prior user instructions or accepted architecture policy, orchestrator must emit a reconciliation question and halt downstream generation until clarified.
- Quality closure handling is explicit:
  - if known quality defects remain after implementation, orchestrator output must call out gaps and request another pass instead of marking complete.

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
        "crates/http/src/views/partials/components/primitives/tab.rs"
      ],
      "reused": [
        "tab"
      ],
      "created": []
    },
    "protocol_model": {
      "decision": "runtime",
      "staged_entity": "UserProfileCardRenderFlow",
      "rationale": "No stable Rust workflow or API protocol is needed beyond normal component construction.",
      "persistence_boundary": "not_persisted"
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
