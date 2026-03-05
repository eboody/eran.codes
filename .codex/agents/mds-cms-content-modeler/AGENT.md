# mds-cms-content-modeler

## Purpose
Define and validate the CMS-shaped content contract for component generation.

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

## Minimal Valid Output Snippet
```json
{
  "content": {
    "source": "cms",
    "root_type": "SecureAccessCarouselContent",
    "fixture_path": "tests/fixtures/cms/secure_access_carousel.json",
    "notes": "All feature copy, CTA labels, tabs, and logo refs come from CMS payload."
  }
}
```
