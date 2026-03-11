---
name: mds-snafu-error-design
version: 0.1.0
description: SNAFU-oriented Rust error design guidance for module-scoped contextual errors, library/application boundaries, source-to-context mapping, and deliberate shared-backtrace boundary wrappers.
scope: project
---

# mds-snafu-error-design

## Purpose
Use when designing or changing Rust error types, propagation, or response/reporting boundaries.

## Quality Priority
- Correctness is a gate.
- Among correct options, prefer the most readable, modular, extensible, expressive, and idiomatic error surface.

## Core Philosophy
- Categorize underlying errors by context, not just by source type.
- One underlying error type may appear in multiple domain-specific cases when the operation or business meaning differs.
- Prefer many cohesive error types over one monolithic error type for unrelated modules.
- Keep library/module error design separate from application/reporting concerns.

## Default Pattern
- Prefer one module-scoped `Error` type per cohesive Rust module or subsystem.
- Use `#[derive(Debug, Snafu)]` for structured errors with contextual fields and `source` chaining.
- Use struct-style errors when one failure mode dominates; use enum-style errors when a module has several related failure contexts.
- Put operation-specific context on the error case itself:
  - paths
  - ids
  - user-visible operation names
  - protocol phase or entity details when relevant
- Use `ResultExt::context` / `with_context`, `OptionExt::context` / `with_context`, and `ensure!` to attach context at the failure site.

## Library vs Application Boundary
- Libraries and internal modules should return specific contextual errors.
- Application, CLI, or HTTP boundaries may aggregate, report, or convert those errors.
- Prefer `snafu::Report` or `#[snafu::report]` at top-level application entrypoints when you want full chained reporting.
- Convert errors to transport responses only at the transport boundary, not inside domain or app logic.

## Source Mapping Rules
- The same `io::Error` may become different cases such as `ReadConfig`, `WriteCache`, or `LoadTemplate`.
- Do not collapse unrelated operations into one generic `Io` or `Other` case when useful context is known.
- Avoid boxing heterogeneous source errors just to reduce variant count unless you are at a deliberate aggregation boundary.

## Whatever Boundary
- `Whatever` is appropriate for:
  - app-edge stringly errors
  - prototypes
  - migration paths from ad-hoc error handling to structured errors
- `Whatever` is not the default for:
  - domain modules
  - library APIs
  - places where structured context is already known

## Shared Trace Boundary Wrapper
- If one outer error type must aggregate many inner cases and you want one trace per error value instead of per case, a wrapper like `Error { kind, trace }` is acceptable.
- Capture the backtrace in constructors or `From` impls so `?` records the boundary conversion site.
- Keep the enum parallel to the trace field, for example `ErrorKind` plus `Backtrace`.
- Reserve this for explicit boundary aggregation or interop layers.
- Do not use this pattern as an excuse to avoid module-local contextual errors.

## Good
```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read config file {path}"))]
    ReadConfig { path: String, source: std::io::Error },

    #[snafu(display("Could not write cache file {path}"))]
    WriteCache { path: String, source: std::io::Error },
}

fn load(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path).context(ReadConfigSnafu { path })
}
```

## Also Acceptable At A Boundary
```rust
#[derive(Debug)]
pub enum ErrorKind {
    User(user::Error),
    Session(session::Error),
}

#[derive(Debug)]
pub struct AppError {
    pub kind: ErrorKind,
    pub trace: Backtrace,
}
```

## Avoid
- one global error enum for unrelated modules
- generic `Io` / `Other` variants with no operation-specific context
- converting everything to `String`, `StatusCode`, or boxed trait objects deep inside the stack
- using a shared `ErrorKind + Backtrace` wrapper to flatten module boundaries too early
