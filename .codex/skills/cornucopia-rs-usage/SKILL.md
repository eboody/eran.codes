---
name: cornucopia-rs-usage
description: Comprehensive guidance for setting up, generating, and using Cornucopia in Rust + PostgreSQL projects. Use when tasks involve writing or annotating `queries/*.sql`, running the Cornucopia CLI/API (`schema` or `live` workflows), selecting sync vs async dependencies, handling nullability/type annotations, integrating generated query modules, or debugging Cornucopia validation/codegen errors.
---

# Cornucopia Rs Usage

## Route Quickly
- Setup, install, and dependencies: read `references/book/01-introduction.md` through `references/book/04-supported-types.md`.
- CLI/API generation workflows and diagnostics: read `references/book/05-using-cornucopia.md` through `references/book/08-error-reporting.md`.
- SQL authoring and annotations: read `references/book/09-writing-queries.md` through `references/book/11-type-annotations.md`.
- Runtime query execution patterns and connection types: read `references/book/12-using-your-generated-queries.md` through `references/book/14-database-connections.md`.
- Example-driven implementation: read `references/book/15-examples.md`.

## Gather Inputs First
Collect these before changing code or commands:
- Query directory (usually `queries/`) and intended generated file path (often `src/cornucopia.rs`).
- Generation mode: `schema` (ephemeral container), `live` (existing DB), or build-script API integration.
- Driver mode: sync (`cornucopia_sync`) or async (`cornucopia_async`).
- Whether async pooling is needed (`deadpool-postgres`).
- Schema file path(s) for `schema` mode, or connection URL for `live` mode.

## Apply This Workflow

### 1. Choose Generation Mode
- Use `schema` when reproducible generation against SQL schema files is preferred and container tooling is available (`docker` or `podman`).
- Use `live` when an existing database lifecycle is already managed externally.
- Use library API in `build.rs` when regeneration must happen automatically on schema/query changes.

### 2. Align Dependencies with Mode
- Always include `postgres-types` with `derive`.
- Sync mode: use `cornucopia_sync` + `postgres`.
- Async mode: use `cornucopia_async` + `tokio` + `tokio_postgres` + `futures`.
- Async pooling (optional): use `deadpool-postgres` and keep `cornucopia_async` deadpool support enabled.
- Extra types must align between crates and driver features (for example JSON/time/uuid/eui48 support).

### 3. Author SQL Correctly
Enforce these rules in every `.sql` file:
- Each Cornucopia query needs a `--!` annotation line.
- Named parameters are required (`:name`), indexed parameters (`$1`, `$2`) are invalid.
- Use optional nullability markers (`?`) only where needed.
- Use `--:` type annotations to share row/parameter structs across queries when reuse improves clarity.

Minimal examples:
```sql
--! authors_by_country (country?) : (age?)
SELECT id, name, age
FROM authors
WHERE nationality = :country;
```

```sql
--: Author(age?)

--! authors : Author
SELECT name, age FROM authors;
```

### 4. Generate and Re-Generate
- Prefer checking command syntax via `cornucopia --help`, then `cornucopia schema --help` or `cornucopia live --help` for exact flags in the current version.
- Re-run generation after any schema/query annotation/type change.
- If generation is automated through `build.rs`, ensure watched files include query and schema paths.

### 5. Integrate Generated Rust Queries
- Import generated module and submodules (`mod cornucopia; use cornucopia::<submodule>;`).
- Build query calls with:
  - `bind(&client, ...)` for direct parameter passing.
  - `params(&client, Struct { ... })` for explicit multi-parameter calls (import `Params` trait from sync/async client crate).
- Fetch results with `opt`, `one`, `iter`, or `all` based on row cardinality expectations.
- Apply `map(...)` when custom row mapping is needed without intermediate allocations.

### 6. Validate and Debug
When codegen fails:
- Start with reported file/line in SQL annotations.
- Check annotation names against actual selected columns and bind parameters.
- Check strict Rust keyword collisions and rename SQL aliases where needed.
- Confirm driver/features match enabled SQL type support.

## Connection Compatibility Checklist
- Sync queries accept: `postgres::Client`, `postgres::Transaction`.
- Async queries accept: `tokio_postgres::Client`, `tokio_postgres::Transaction`.
- Async + deadpool additionally accepts: `deadpool_postgres::Client`, `deadpool_postgres::Transaction`.

## Rebuild Book References
Run `scripts/sync_book_from_website.sh <path-to-cornucopia-website-repo>` to regenerate `references/book/*.md` from `book/SUMMARY.md` order.

## References
- `references/book/README.md`: ordered chapter index.
- `references/book/*.md`: chapter content mirrored from the Cornucopia website book.
