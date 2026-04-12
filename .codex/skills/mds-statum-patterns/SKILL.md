---
name: mds-statum-patterns
version: 0.1.0
description: Statum workflow guidance for stable Rust workflows and API protocols, including typestate fit, hybrid boundaries, validators-based rehydration, and Bon-backed machine construction.
scope: project
---

# mds-statum-patterns

## Purpose
Use `statum` when Rust workflows or API protocols have stable ordered phases and invalid protocol moves should be unrepresentable.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic workflow surface.

## Use
- Use before changing stable Rust workflows, API protocols, persisted status reconstruction, or legal transition modeling.
- Pair with `mds-bon-patterns` when a flow needs both construction ergonomics and lifecycle legality.
- Record the result in `component_spec.design.protocol_model`.

## Source Docs (Authoritative)
- This skill file is the repo-local Statum rule surface.
- The repo no longer mirrors Statum docs under `/docs/statum`.
- When deeper Statum reference is needed, inspect current `statum` usage in the repo and use the upstream crate docs instead of assuming repo-local mirrors exist.

## Fit Checklist
Ask all six before choosing a runtime-only model:
1. Does the entity move through a finite set of meaningful phases?
2. Are legal transitions mostly known at compile time?
3. Is an invalid transition materially expensive?
4. Does API behavior differ by state in a meaningful way?
5. Is some data valid only in specific phases?
6. Is the workflow stable enough to justify type-level encoding?

Interpretation:
- `5-6 yes`: strong `statum` candidate.
- `3-4 yes`: hybrid candidate; encode the stable spine with `statum` and keep volatile edges runtime-validated.
- `0-2 yes`: runtime model is likely better; record why.

## Strong Candidate Rules
- Define business-phase states first with `#[state]`.
- Put long-lived IDs, repositories, and shared context on `#[machine]`.
- Keep legal edges in concrete-state `#[transition]` impls only.
- Keep constructors, branching helpers, formatting, and orchestration glue in regular `impl` blocks.
- Put phase-specific invariant data on the state variant, not on the machine root.
- Use `#[validators]` for persisted workflow rehydration when dynamic storage facts must reconstruct typed machines.

## Hybrid Boundary Rules
- Keep the stable protocol core in `statum`.
- Keep user-authored, tenant-configured, plugin-defined, or highly volatile policy branches in runtime validation.
- Record the boundary explicitly in `design.protocol_model.runtime_edges` and `design.protocol_model.rationale`.

## Bon Boundary
- Use plain `bon` to assemble commands, DTOs, and machine context when no broader typed protocol is needed.
- Statum-generated machines still use Bon-backed builders for machine construction.
- Do not use plain-Bon builder steps to simulate protocol stages that should be typed transitions.
- If method availability should change by phase, that is a `statum` concern, not a builder concern.

## Rehydration Rules
- Persisted statuses are dynamic facts, not typed guarantees.
- Prefer `#[validators]` when reconstructing typed machines from rows or external records.
- Keep validator style consistent within a type; if any validator is async, treat the generated builder surface as async too.

## Good
```rust
#[state]
pub enum ReviewState {
    Draft,
    InReview(ReviewData),
    Published(PublishMeta),
}

#[machine]
pub struct DocumentMachine<ReviewState> {
    id: String,
    repo: Arc<dyn DocumentRepo>,
}

#[transition]
impl DocumentMachine<Draft> {
    fn submit_for_review(self, reviewer: String) -> DocumentMachine<InReview> {
        self.transition_with(ReviewData { reviewer })
    }
}
```

## Avoid
- builder chains that mimic lifecycle stages
- generic `impl Machine<S>` transition methods that reintroduce illegal edges
- helper methods stuffed into `#[transition]` impl blocks
- trusting persisted status flags directly instead of validating them at the rehydration boundary
