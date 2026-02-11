# utils crate

Shared helpers with no layer-specific dependencies.

## Responsibilities
- Small, reusable utilities that can be used across layers.
- Keep this crate lightweight and free of HTTP/DB concerns.

## Boundaries
- Should not depend on `http` or `infra`.
