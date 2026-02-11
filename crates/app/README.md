# app crate

Use-case orchestration and policy.

## Responsibilities
- Defines commands and services for business operations.
- Performs contextual validation (uniqueness, authorization, rate limits).
- Defines traits for external mechanisms (repositories, hashing, clocks).
- Converts untrusted input into domain types early.

## Boundaries
- Depends on `domain`.
- Must not depend on HTTP DTOs or serde types.
- Traits here are implemented by `infra`.

## Examples
```rust
use app::chat::Service;

let service = Service::builder()
    .with_repo(repo)
    .with_rate_limiter(rate_limiter)
    .with_audit_log(audit_log)
    .build();
```
