# infra crate

Concrete implementations for external mechanisms.

## Responsibilities
- Implements app-defined traits (repositories, hashing, clocks).
- Owns SQL, migrations, and DB query efficiency.
- Maps database rows into domain entities.
- Implements credential hashing with argon2.

## Migrations
Migrations live in `crates/infra/migrations/`.

```bash
cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations
```

## Boundaries
- Depends on `domain` and `app` traits.
- Must not accept plaintext passwords for persistence.
