# infra

`infra` is the mechanism layer: Postgres, hashing, migrations, and repository implementations that satisfy the contracts defined in `app`.

## What it owns
- SQL repositories and query shaping
- Argon2 hashing and auth persistence
- migrations and database-backed session support
- infra-specific configuration parsing

## What it intentionally avoids
- HTTP or view logic
- business policy that belongs in `app`
- accepting invalid raw values where domain types should exist first

## Read it like this
- [src/config.rs](./src/config.rs)
- [src/auth.rs](./src/auth.rs)
- [src/repo/README.md](./src/repo/README.md)
- [migrations/](./migrations/)

## Local database workflow

```bash
cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations
```

## Why this crate exists
`infra` should be able to change query strategy, indexes, or hashing mechanics without dragging transport or policy concerns into the same layer.

## Read next
- [app](../app/README.md)
- [http](../http/README.md)
