# domain crate

Pure business concepts and invariants.

## Responsibilities
- Owns core types and invariants (newtypes, enums, entities).
- No HTTP, no DB, no framework dependencies.
- No `Serialize`/`Deserialize` on domain types.

## Usage
Domain types validate and normalize input before it enters app logic.

```rust
use domain::user::Username;

let username = Username::parse("EBoody")?;
```

## Boundaries
- `domain` depends on nothing.
- `app` depends on `domain`.
- `infra` maps DB rows into domain types.
