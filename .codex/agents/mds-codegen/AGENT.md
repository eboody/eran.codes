---
name: mds-codegen
description: >-
  Generate Rust Maud and Datastar component code from a verified component
  spec.
---

# mds-codegen

## Purpose
Generate Rust Maud + Datastar component code from a verified `component_spec`.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic generated code.

## Inputs It Expects
- `component_spec.design`
- `component_spec.ui`
- `component_spec.state`
- `component_spec.events`
- `component_spec.backend_contracts`
- `component_spec.pipeline`
- `.codex/skills/mds-statum-patterns/SKILL.md`
- `.codex/skills/mds-snafu-error-design/SKILL.md`
- `.codex/skills/mds-rust-namespace-surface/SKILL.md`

## Outputs It Must Produce
- `codegen.files[]`
- `codegen.markers[]`
- `codegen.trace`

## Non-Goals / Forbidden Behaviors
- Must not mutate `component_spec` sections owned by upstream agents.
- Must not bypass regeneration safety markers.
- Must not emit code when verifier status is `fail`.
- Must not choose clever, compressed, or monolithic code when a clearer decomposed design is available.

## Mapping Rules (Authority-Aware)
- Map `events.ui_transitions` to client-local Datastar behavior for `authority = "ui"` fields.
- Respect handler protocol selection:
  - `protocol_mode = ui_local` => local interaction wiring only.
  - `protocol_mode = command_sse` => command endpoint intent + SSE convergence for app-authority updates.
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
- Prefer importing/reusing existing components from `crates/http/src/views/partials/components` before generating new feature-local structures.
- For reusable components, generate typed child render components and compose them as props instead of flattening repeated markup into one render block.
- Reuse existing primitives (for example `Tab`, `Icon`) before creating parallel equivalents.
- When adding a new reusable component, use generic library-style names (for example `tab_item`, `tab_set`) instead of request-specific wording.
- Prefer module-scoped type families and namespace usage (for example `tab_set::pane::Body`, `application::Service`) over flat prefixed type naming.
- When a module path is intended API vocabulary, keep generic companion nouns under that module (for example `user::Id`, `user::Repository`) instead of flattening them into parent surfaces.
- For modules marked with `// ci: descriptive-module-import <full_module_path>`, import the namespace itself and qualify from it; do not generate leaf `use` or `pub use` against that module path.
- If `design.protocol_model.decision = statum`, generate the stable workflow core with `#[state]`, `#[machine]`, concrete-state `#[transition]` impls, and keep helpers/orchestration outside transition blocks.
- If `design.protocol_model.decision = hybrid`, generate the stable core with `statum` and leave dynamic policy branches in regular methods or explicit runtime guards.
- If `design.protocol_model.persistence_boundary = validators`, generate or preserve a `#[validators]` rehydration boundary instead of trusting persisted status flags directly.
- Use plain `bon` for command/context assembly and `statum` for lifecycle legality; do not generate builder chains that simulate protocol steps.
- When generating Statum-backed machines, keep machine construction Bon-backed instead of hand-rolling alternative constructors.
- Generate module-scoped contextual error types near the code that owns the failure semantics instead of one broad cross-module error enum.
- Prefer `#[derive(Debug, Snafu)]` custom errors with contextual fields and `source` chaining.
- Use struct-style SNAFU errors for single cohesive failure modes and enum-style SNAFU errors for several related contexts in one module.
- Use `.context(...)`, `.with_context(...)`, `OptionExt`, and `ensure!` to attach context at the failure site instead of flattening everything into manual string errors.
- The same underlying source type may map into multiple contextual variants when the operation differs.
- Keep library/module errors specific and convert them to HTTP/application responses only at the outer boundary.
- Use `Whatever` only for app-edge stringly errors, prototypes, or explicit migration paths.
- If a deliberate outer aggregation layer is required, a parallel `ErrorKind` plus `Backtrace` wrapper is acceptable, but preserve the inner contextual error types.
- Keep long-lived context on the machine, and keep phase-specific invariant data on the state variants.
- Emit markup that can consume shared global `app.css` package classes for reusable patterns.
- Keep scoped `inline_css!` for exception-only component-specific behavior.
- Keep output file set stable unless explicitly requested (`view.rs`, `state.rs`, `events.rs`, `handler.rs`).

## Checklist Of Required Invariants
- Every generated file includes begin/end managed markers.
- Generated Maud nodes map to declared `ui.nodes` ids.
- Generated Datastar interactions map to declared `events` and `state` ids.
- Generated server mapping code updates app-authority fields only from SSE mapping paths (`datastar-patch-signals`).
- Generated templates avoid literal marketing copy (placeholder/debug text only).
- Codegen trace includes schema/version used.
- Generated reusable component roots and declared child parts should each `impl Render`.
- Generated Rust imports preserve marked namespace roots; do not leaf-import or parent-`pub use` their companion nouns.
- Generated Statum workflows keep `#[transition]` blocks protocol-only and place helper/branch/orchestration methods in regular `impl` blocks.
- Generated persisted typed workflows rehydrate through validators when `design.protocol_model.persistence_boundary = validators`.
- Generated Rust error surfaces prefer contextual SNAFU types over opaque boxed catch-alls unless an explicit boundary wrapper is justified.

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
      "schema": "mds-component-spec@0.4.0"
    }
  }
}
```
