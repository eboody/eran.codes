# mds-cms-content-modeler

## Purpose
Define and validate the CMS-shaped content contract for component generation.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic contract shape.

## Inputs It Expects
- `component_spec.meta`
- `component_spec.scope`
- Existing `component_spec.content` (if regenerating)
- Decomposed UI regions from `component_spec.ui` (if already available)

## Outputs It Must Produce
- `component_spec.content`

## Non-Goals / Forbidden Behaviors
- Must not generate Rust/Maud source code.
- Must not define state transition or backend transport behavior.
- Must not hardcode marketing copy literals into templates.

## Checklist Of Required Invariants
- `content.source` is `cms`.
- `content.root_type` is a typed `*Content` model.
- `content.fixture_path` points to a checked-in fixture file.
- Content contract is decomposition-friendly: child components can consume specific slices.
- `content.root_type` naming is generic/library-style (avoid campaign/request-specific names).
- Must cooperate with `design.reuse_scan` so component creation records reuse-first evaluation against `views/partials/components`.
- Favor content contracts that keep mapping, validation, and rendering boundaries explicit rather than bundling many concerns into one root shape.

## Minimal Valid Output Snippet
```json
{
  "content": {
    "source": "cms",
    "root_type": "TabSetContent",
    "fixture_path": "tests/fixtures/cms/tab_set.json",
    "notes": "All feature copy, CTA labels, tabs, and logo refs come from CMS payload."
  }
}
```
