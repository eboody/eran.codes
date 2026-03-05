# Architecture Map

## Target Boundary Model

Dependency direction is strict and one-way:

`domain <- app <- (http, infra) <- main`

- `domain` is core business logic and invariants.
- `app` orchestrates use-cases and declares ports/traits.
- `http` and `infra` are adapters.
- `main` is the composition root that wires concrete adapters into app services.
- `http` may create domain value objects only via validated constructors/newtypes; it must not enforce business policy (only transport parsing + basic well-formedness).
- `http` must not depend on `infra`. All I/O is reached via `app` ports.

No transport or persistence concern may leak into `domain` or `app` policy decisions.

## Responsibility Table

| Layer | Owns | Depends On | Must Not Own |
|---|---|---|---|
| `main` | runtime bootstrap, config loading, dependency wiring, router/server startup | `http`, `infra`, `app` (transitively `domain`) | business rules, SQL, request handling logic |
| `http` | Axum routing/handlers/middleware, request/response mapping, Maud rendering, Datastar event/SSE transport, auth/session edge concerns | `app` (may use `domain` public types as input/output shapes) | domain invariants, persistence logic, SQL |
| `app` | use-case orchestration, policy, transactions at service level, repository/port traits, typed app errors | `domain` | Axum types, Maud/Datastar rendering, SQL/DB drivers |
| `domain` | entities, value objects, invariants, domain errors, pure business rules | none | HTTP DTOs, database rows, framework/runtime concerns |
| `infra` | repository implementations, SQL, persistence mapping, external providers (hashing/storage), migration-facing concerns | `app`, `domain` | request routing, HTML rendering, UI event semantics |

`http` and `infra` are sibling adapters and must not depend on each other.

## Minimal Directory/Crate Map

```text
src/
  main.rs                  # composition root
crates/
  domain/                  # core business model and invariants
  app/                     # use-cases and ports
  http/                    # Axum handlers, routing, Maud views, Datastar/SSE transport
  infra/                   # repository adapters, SQL, external mechanisms
  utils/                   # cross-cutting utility helpers (no policy)
```

## Flow: Request Path

1. `http` receives request (Axum route/handler).
2. `http` parses transport DTOs into typed inputs.
3. `http` calls `app` service/use-case.
4. `app` executes policy using `domain` rules and `app` ports.
5. `infra` implementations fulfill port calls (DB/external IO).
6. `app` returns typed result/error.
7. `http` maps result/error to response shape, Maud HTML, or SSE/Datastar event stream.
8. `http` performs error presentation only (mapping), not error interpretation (policy).

## Flow: Data/Persistence Path

1. Boundary data enters at `http` (request) or `infra` (storage row/provider payload).
2. Convert early into typed forms (parse, do not validate late).
3. `domain` enforces invariants.
4. `app` coordinates read/write intent via ports.
5. `infra` maps between domain/app types and persistence schema.
6. Persistence details remain in `infra`; only typed results return upward.

## Datastar + Maud Placement

- Maud template composition is owned by `http` view modules.
- Datastar attribute/event wiring is owned by `http` transport/view integration.
- SSE event endpoint and stream lifecycle are owned by `http`.
- `domain` and `app` may expose typed state/events, but never Datastar/Maud/Axum primitives.

## Error Ownership Rule

- `domain` and `app` own error meaning.
- `http` maps `domain`/`app` errors to transport responses and presentation shapes.
- `http` must not reclassify policy outcomes; it only maps them.

## Anti-Patterns (Must Not Happen)

- `domain` importing Axum, Datastar, Maud, SQL, or session middleware types.
- `app` returning transport-shaped responses (status codes, headers, HTML fragments).
- `http` implementing business policy that belongs in `app`/`domain`.
- `http` calling `infra` repositories directly.
- `infra` deciding user-facing policy outcomes beyond persistence/external mechanism errors.
- UI event semantics (Datastar attributes/SSE framing) leaking into `domain` entities.
- Persistence schema/row types leaking into `domain` models.

## Verification Gate (Architecture)

The architecture verifier must fail when either condition is true:

1. Layer responsibility violates boundary model.
   - Example failures: SQL in `app`, Axum/Datastar/Maud in `domain`, policy logic in `infra`, or `http` directly coupled to `infra`.
2. Ownership is ambiguous across `http`/`app`/`domain`/`infra`.
   - Any responsibility assigned to multiple layers without a single owner is a failure.

Pass criteria:
- Every concern has exactly one owner layer.
- Dependencies follow `domain <- app <- (http, infra) <- main`.
- `http` and `infra` remain sibling adapters (no direct dependency).
- Transport/persistence details stay out of core business logic.

## Sources Used

- `/docs/reference-map.md`
- `/docs/code-audit/02-request-and-data-flows.md`
- `/docs/project-audit.md`
- `/docs/writing-style.md`
- `/docs/axum/index.md`
- `/docs/axum/routing/index.md`
- `/docs/axum/response/sse/index.md`
- `/docs/datastar/index.md`
- `/docs/datastar/reference/sse_events.md`
- `/docs/maud/index.md`
- `/docs/maud/render-trait.md`
