# Project Audit

Date: 2026-02-24  
Scope: entire workspace (`domain`, `app`, `infra`, `http`, `utils`, root wiring, CI scripts, and key docs)

## Executive Summary
The architecture is still strong overall, but this audit found one critical correctness bug and several medium/high issues in auth/session safety, error semantics, and operational guardrails.

Overall status:
- Boundary discipline: strong
- Correctness: moderate (one critical gap)
- Security posture: moderate
- Operational readiness: moderate
- Test confidence: moderate-low for edge/failure cases

Top risks:
1. Chat rate limiter currently does not enforce an upper bound (critical).
2. Session auth hash is currently the password hash material reused in session state (high).
3. Several malformed client inputs are mapped to `500 Internal Server Error` instead of `4xx` (medium).

## Methodology
Static review:
- Full pass through workspace crates and wiring.
- Boundary checks against `AGENTS.md` architecture rules.
- Tracing/session/auth flow review across `http`, `app`, and `infra`.
- SQL and migration review.

Execution checks:
- `cargo check` (pass)
- `cargo test --workspace` (pass, 14 tests)
- `bash scripts/ci/no-string-fields.sh` (pass)
- `bash scripts/ci/partials-render.sh` (pass)
- `bash scripts/ci/stringy-check.sh` (fail)
- `cargo-clippy --workspace --all-targets -- -D warnings` with `RUSTC_WRAPPER=` (fail, multiple findings)
- `cargo-fmt --all -- --check` via `cargo-fmt` (fail)

## Boundary Map
- `domain`: invariants and core types (`Username`, `Email`, chat IDs, statuses). No serde leakage in domain entities.
- `app`: use-case orchestration and traits (`Repository`, `PasswordHasher`, `RateLimiter`, etc.).
- `infra`: SQL, hashing, repository implementations, migrations.
- `http`: transport/routing, auth middleware, session + SSE integration, Maud views.
- `src/main.rs`: composition root and layer wiring.

Boundary direction remains mostly correct (`domain -> app -> infra/http -> main`).

## Findings (Ordered by Severity)

### P0-1: Chat Rate Limiter Does Not Actually Block Over-Limit Traffic
Severity: Critical

Evidence:
- `crates/infra/src/repo/chat.rs:499`
- `crates/infra/src/repo/chat.rs:508`

Details:
- The counter stops incrementing at max (`ELSE chat_rate_limits.count`).
- Allowed condition is `count <= $4`.
- Once count reaches max, subsequent requests continue to be allowed.

Impact:
- App-level anti-spam/rate-limit policy is bypassed in practice.

Recommendation:
- Change evaluation to block when at/over threshold for active window.
- Example fixes:
  - compute `allowed` from pre-increment count and allow only if `< max`, or
  - continue incrementing count and enforce a strict post-update threshold check that rejects after max.
- Add deterministic tests around requests 1..(max+2).

### P1-1: Session Auth Hash Reuses Password Hash Material
Severity: High

Evidence:
- `crates/app/src/auth.rs:148`
- `crates/app/src/auth.rs:164`
- `crates/app/src/auth.rs:195`
- `crates/http/src/auth.rs:18`
- `crates/http/src/auth.rs:82`

Details:
- `SessionHash::from_password_hash` currently returns the password hash string.
- That value is converted to `session_hash_bytes` and fed into `AuthUser::session_auth_hash`.

Impact:
- Credential hash material is reused in session state surfaces.
- Increases blast radius if session storage/serialization is exposed.

Recommendation:
- Replace with a dedicated, non-reversible auth-session verifier:
  - derive with keyed HMAC over credential version, or
  - store and use a dedicated `session_version`/`auth_version` per user.
- Keep password hash confined to credential verification paths.

### P1-2: Registration Has TOCTOU Race and Can Return 500 Instead of `EmailTaken`
Severity: High

Evidence:
- `crates/app/src/user/mod.rs:45`
- `crates/app/src/user/mod.rs:61`
- `crates/infra/src/repo/user.rs:59`
- `crates/infra/src/repo/user.rs:70`

Details:
- App pre-checks `find_by_email` then inserts.
- Concurrent requests can both pass pre-check; one insert fails on unique constraint.
- Infra maps SQL error to generic repo text; handler then becomes internal error path.

Impact:
- User-facing correctness issue under concurrency.
- False 500s for a normal business conflict.

Recommendation:
- Map SQL unique violation (`23505`) to typed `EmailTaken`.
- Keep pre-check for UX only; rely on DB constraint as final authority.
- Add concurrent registration test.

### P2-1: Email Invariant Is Too Weak (Any Non-Empty String Passes)
Severity: Medium

Evidence:
- `crates/domain/src/user/mod.rs:15`
- `crates/domain/src/user/mod.rs:17`
- `crates/app/src/user/mod.rs:77`

Details:
- `Email` currently validates only `not_empty`.
- Backend accepts strings that are not valid emails if client-side validation is bypassed.

Impact:
- Invalid emails can be persisted.
- Boundary-check demo semantics are misleading for `"not-an-email"`.

Recommendation:
- Add domain-level email syntax validation (pragmatic RFC subset).
- Add tests for invalid/valid canonical examples.

### P2-2: Malformed Chat Inputs Mapped to 500 Instead of 4xx
Severity: Medium

Evidence:
- `crates/http/src/handlers/demo/chat.rs:92`
- `crates/http/src/handlers/demo/chat.rs:423`
- `crates/http/src/handlers/demo/chat.rs:430`
- `crates/http/src/handlers/demo/chat.rs:437`
- `crates/http/src/handlers/demo/chat.rs:446`

Details:
- UUID/body/reason parse errors return `Error::Internal`.
- Invalid moderation decision also maps to `Internal`.

Impact:
- Incorrect HTTP semantics (`500` for client input errors).
- Noisy operational signals and weaker diagnostics.

Recommendation:
- Map parse failures to typed `app::chat::Error::InvalidId` / domain validation errors, then to `400`.

### P2-3: Per-Session Trace Log Clear Causes Cross-Tab Interference
Severity: Medium

Evidence:
- `crates/http/src/handlers/sse.rs:257`
- `crates/http/src/handlers/sse.rs:269`
- `crates/http/src/sse/mod.rs:16`

Details:
- Trace log cleanup key is only `session_id`.
- When one tab disconnects, `clear_session` drops logs for all tabs sharing session.

Impact:
- Multi-tab observability becomes inconsistent and surprising.

Recommendation:
- Clear only when last stream for a session is gone, or
- track trace entries by full stream key (`session_id + sse_tab_id`) and aggregate intentionally.

### P2-4: Critical Flow Errors Are Silently Ignored in Chat Bootstrapping
Severity: Medium

Evidence:
- `crates/http/src/chat_demo.rs:35`
- `crates/http/src/handlers/demo/chat.rs:212`

Details:
- `join_room` errors are discarded in two places.
- Failures can be deferred into later confusing states (`NotMember`, empty context, etc.).

Impact:
- Hidden failures reduce debuggability and can produce inconsistent behavior.

Recommendation:
- Handle explicitly with typed “already joined” vs actual failure mapping.

### P2-5: Surreal Demo State Can Leak and Is Session-Scoped, Not Tab-Scoped
Severity: Medium

Evidence:
- `crates/http/src/state.rs:24`
- `crates/http/src/state.rs:39`
- `crates/http/src/handlers/sse.rs:83`
- `crates/http/src/handlers/sse.rs:159`

Details:
- Guard/cancel maps are keyed by `SessionId` and entries are not cleaned after completion/disconnect.

Impact:
- Memory growth over time.
- Cross-tab cancellation/serialization interference.

Recommendation:
- Key by stream key (session + tab), and remove entries on task completion / stream drop.

### P2-6: Tracing Guidance Says Not To Log Raw Payloads, But Chat Body Is Logged
Severity: Medium

Evidence:
- `docs/tracing.md:54`
- `crates/http/src/handlers/demo/chat.rs:160`
- `crates/http/src/handlers/demo/chat.rs:262`
- `crates/http/src/handlers/demo/chat.rs:397`

Details:
- Raw message bodies are included in trace fields.

Impact:
- Potential sensitive content exposure in live logs.

Recommendation:
- Redact or hash payload text; keep size/type metadata in live streams.

### P3-1: CI Guardrail Fails Due String-Literal Pattern in View Icon
Severity: Low

Evidence:
- `crates/http/src/views/partials/demo/layout/capability_showcase.rs:58`
- `scripts/ci/stringy-check.sh`

Details:
- `Text::from("==")` triggers `contains("...")` pattern in guardrail script.

Impact:
- CI/lint signal is red despite non-policy logic usage.

Recommendation:
- Replace icon literals with typed icon enum/variant rendering.

### P3-2: `DbConfig.max_connections` Is Defined But Not Applied
Severity: Low

Evidence:
- `crates/infra/src/config.rs:12`
- `crates/infra/src/lib.rs:19`

Details:
- Config carries `max_connections`, but pool is built with `PgPool::connect` (default options).

Impact:
- Config value is dead; operational tuning is ineffective.

Recommendation:
- Use `sqlx::postgres::PgPoolOptions::new().max_connections(cfg.db.max_connections)`.

### P3-3: Local Tooling Aliases in Repo Are Broken (`fmt`, `clippy`)
Severity: Low

Evidence:
- `.cargo/config.toml:14`
- `.cargo/config.toml:15`

Details:
- Alias definitions recursively point to same subcommand name.
- `cargo fmt` / `cargo clippy` fail in this workspace context without binary workaround.

Impact:
- Developer friction, potential CI confusion if aliases are used.

Recommendation:
- Remove recursive aliases or rename them (e.g., `xfmt`, `xclippy`).

### P3-4: Committed Local Session Secret in `.cargo/config.toml`
Severity: Low

Evidence:
- `.cargo/config.toml:6`

Details:
- A concrete `SESSION_SECRET` value is checked into repo-local config.

Impact:
- Policy drift versus “do not commit secrets”; easy copy/paste into non-local environments.

Recommendation:
- Replace with placeholder and document local setup via `.env`/shell export.

### P3-5: Formatting/Lint Debt Is Non-Zero
Severity: Low

Evidence:
- `crates/app/src/chat/mod.rs:430`
- `crates/app/src/chat/mod.rs:434`
- clippy output includes multiple denied warnings across `http`/`infra`.

Details:
- `cargo-fmt --check` and `cargo-clippy -D warnings` both fail.

Impact:
- Reduced guardrail reliability.

Recommendation:
- Fix lint/format backlog and make both checks mandatory in CI.

## Testing Gaps
Missing coverage in high-risk paths:
- Rate limiter boundary behavior (`N`, `N+1`, window rollover).
- Concurrent registration duplicate email conflict mapping.
- Chat parse/validation failure status mapping (`400` vs `500`).
- Multi-tab SSE disconnect behavior and trace-log isolation.
- Email validator behavior (valid/invalid matrix).

## Documentation Drift
- `docs/auth-sessions.md` references session middleware location in `crates/http/src/lib.rs`, but current implementation is in `crates/http/src/router/layers.rs`.
  - Evidence: `docs/auth-sessions.md:20`, `crates/http/src/router/layers.rs:179`
- `docs/tracing.md` requires app service instrumentation; many public app chat methods are not instrumented.
  - Evidence: `docs/tracing.md:34`, `crates/app/src/chat/mod.rs:241`

## Prioritized Remediation Plan
1. P0 immediate:
   - Fix rate limiter SQL logic and add boundary tests.
2. P1 this sprint:
   - Replace session hash strategy.
   - Fix duplicate registration conflict mapping.
3. P2 next:
   - Correct parse/error mappings to 4xx.
   - Fix multi-tab trace cleanup and surreal state lifecycle.
   - Redact/harden payload logging.
4. P3 cleanup:
   - Resolve guardrail false positive.
   - Apply `max_connections`.
   - Repair `.cargo` aliases and secret handling.
   - Clear clippy/rustfmt debt.

## Conclusion
The architecture remains fundamentally solid, but policy-critical behavior (rate limiting), auth/session hardening, and error semantics need targeted fixes before calling this production-grade. The path to “strong” is clear and finite: one critical bug fix, two high-priority hardening changes, then medium-level reliability and observability cleanup.

## Addendum: Portfolio Content, Formatting, and Visual Audit
Date: 2026-02-24  
Scope: portfolio-facing UX in `crates/http/src/views` and `crates/http/static/app.css`

### Portfolio UX Summary
- Content clarity: `6/10`
- Information architecture + formatting: `6/10`
- Visual polish + distinctiveness: `5.5/10`
- Portfolio conversion readiness: `4.5/10`
- Technical credibility: `8/10`

### High-Priority Findings
1. The hero and top-level messaging emphasize platform implementation over portfolio conversion.
   - Evidence:
     - `crates/http/src/views/partials/demo/layout/home_hero.rs:18`
     - `crates/http/src/views/partials/demo/layout/home_hero.rs:19`
     - `crates/http/src/views/page.rs:27`
2. Capability showcase copy reads as generic SaaS language and is not consistently grounded in this app’s actual demo surfaces.
   - Evidence:
     - `crates/http/src/views/partials/demo/layout/capability_showcase.rs:27`
     - `crates/http/src/views/partials/demo/layout/capability_showcase.rs:59`
     - `crates/http/src/views/partials/demo/layout/capability_showcase.rs:91`
3. The strongest proof surface (chat capstone) is functionally gated for anonymous visitors, creating early friction for portfolio reviewers.
   - Evidence:
     - `crates/http/src/views/pages/home.rs:379`
     - `crates/http/src/router/routes.rs:90`
4. Showcase CTA targets point at `#live-chat-demo` instead of the actual live chat section anchor (`#chat-demo`), reducing flow clarity.
   - Evidence:
     - `crates/http/src/views/partials/demo/layout/capability_showcase.rs:53`
     - `crates/http/src/views/partials/demo/chat/chat_demo_section.rs:16`
     - `crates/http/src/views/pages/home.rs:358`
5. Page structure is dense before payoff; scanning cost is high because multiple heavy sections appear before direct interaction.
   - Evidence:
     - `crates/http/src/views/pages/home.rs:24`
     - `crates/http/src/views/pages/home.rs:72`
     - `crates/http/src/views/pages/home.rs:138`
     - `crates/http/src/views/pages/home.rs:141`

### Medium-Priority Findings
1. Editorial consistency drift (title casing and “realtime”/“real-time” variations).
   - Evidence:
     - `crates/http/src/views/partials/demo/layout/professionalism_in_practice_tabs.rs:15`
     - `crates/http/src/views/pages/home.rs:144`
2. Tab control implementation uses hidden radio inputs and labels; semantics are weaker than explicit tab roles and relationships.
   - Evidence:
     - `crates/http/src/views/partials/demo/layout/tabbed_showcase.rs:198`
     - `crates/http/src/views/partials/demo/layout/tabbed_showcase.rs:206`
3. Several text sizes are too small for comfortable reading in portfolio contexts.
   - Evidence:
     - `crates/http/static/app.css:39`
     - `crates/http/static/app.css:609`
     - `crates/http/static/app.css:716`
4. Some log/table contrast choices rely on low-opacity light colors and can degrade in light mode readability.
   - Evidence:
     - `crates/http/static/app.css:635`
     - `crates/http/static/app.css:657`
     - `crates/http/static/app.css:700`
5. Log panel heading hierarchy is not semantic (`<p><strong>...</strong></p>` instead of heading elements).
   - Evidence:
     - `crates/http/src/views/partials/demo/log/panel.rs:23`

### Strengths Worth Preserving
- Architecture communication is coherent and specific.
- Professionalism section ties claims to concrete code paths/snippets.
- Live backend/network/chat flow panels create strong “show, don’t tell” proof.

### Prioritized UX Remediation
1. Rewrite hero/top-nav messaging for portfolio conversion while keeping platform context.
2. Replace generic capability copy with app-accurate capabilities tied to real routes/components.
3. Provide a no-login proof path (read-only live chat/demo visibility).
4. Reduce scan cost by trimming/reordering dense sections and aligning CTA anchors.
5. Raise small text sizes and improve contrast tokens in log/table surfaces.
6. Improve semantics and accessibility in tab/log heading structures.
