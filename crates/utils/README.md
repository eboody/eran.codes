# utils crate

Shared helpers with no layer-specific dependencies.

## Responsibilities
- Small, reusable utilities that can be used across layers.
- Keep this crate lightweight and free of HTTP/DB concerns.
- Developer tooling binaries (for example `visual_snapshot`) that are not layer-specific.

## Boundaries
- Should not depend on `http` or `infra`.
