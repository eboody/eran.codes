# Refactor Plan (Prioritized)

Date: 2026-02-16
Source: `docs/project-audit.md`

## Status
- Completed (2026-02-17): Priority 1
- Completed (2026-02-17): Priority 2 (core repository conversion paths)
- Completed (2026-02-17): Priority 3
- Pending: Priority 4

## Priority 1: Tighten Request Metadata Typing
Goal: remove remaining plain-string metadata paths in HTTP request context.

Why first:
- High leverage across tracing/logging.
- Low risk to business behavior.
- Improves consistency with existing `http::types` direction.

Scope:
1. Refactor `crates/http/src/request.rs` to return typed wrappers (`ClientIp`, `UserAgent`, `RoutePath`, etc.) instead of `Option<String>`.
2. Keep string conversion at boundaries only (header parsing + final rendering).
3. Add tests for fallback logic and header precedence with typed outputs.

Exit criteria:
- No plain `Option<String>` return values for request metadata in `request.rs`.
- Tests cover major header extraction paths.

## Priority 2: Normalize Infra Conversion Boundaries
Goal: make DB row-to-domain conversion paths more explicit and consistent.

Why second:
- Most correctness risk is here.
- Important for long-term migration safety and schema evolution.

Scope:
1. In `crates/infra/src/repo/chat.rs`, `crates/infra/src/repo/user.rs`, and `crates/infra/src/auth.rs`, centralize conversions into small typed helper functions.
2. Replace repetitive inline conversions with domain-aware constructors.
3. Ensure all conversion failures map into typed infra/app errors with clear context.

Exit criteria:
- Conversion logic is centralized in each repo module.
- Fewer repeated `row.get::<String, _>(...)` + ad-hoc parsing chains.

## Priority 3: Type Log “Extras” Instead of Formatting Ad-Hoc Strings
Goal: reduce string assembly in log row rendering and improve display invariants.

Why third:
- Primarily maintainability/readability gain.
- Builds on already componentized log UI.

Scope:
1. Introduce typed log extra entries (field/value pairs with display kind) used by:
   - `crates/http/src/views/partials/demo/log/live_log.rs`
   - `crates/http/src/views/partials/demo/log/trace_log.rs`
2. Push display decisions into enums/newtypes instead of `Vec<String>`.
3. Reuse existing pill/row components for consistent rendering.

Exit criteria:
- No ad-hoc extras string concatenation in live/trace log renderers.
- Extras render path uses typed units.

## Priority 4: SSE Per-Tab Identity
Goal: support per-tab SSE stream identity while preserving current session model.

Why fourth:
- Architectural enhancement, not immediate correctness issue.
- Requires protocol/client coordination and therefore more careful rollout.

Scope:
1. Add tab identifier handling to SSE keying logic.
2. Keep backward-compatible behavior for existing sessions.
3. Document flow and edge-cases in SSE docs.

Exit criteria:
- Multiple tabs for one authenticated session can independently receive expected stream behavior.
- Docs updated with keying model.

## Nice-to-Have (After Priority 1-4)
1. Expand CI checks for typed request metadata usage in HTTP modules.
2. Add a small style guide for trace/log event naming to reduce future drift.
3. Add integration tests validating live vs diagnostic trace separation end-to-end.
