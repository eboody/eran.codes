# domain

`domain` holds the business vocabulary of the system: validated ids, normalized inputs, entities, and finite states that mean the same thing no matter how the app is delivered.

## What it owns
- `user` identities and invariants such as usernames, emails, and user ids
- `chat` concepts such as room ids, message ids, message bodies, statuses, and moderation reasons
- module-local errors for invalid domain input

## What it intentionally avoids
- HTTP request/response shapes
- SQL or persistence concerns
- serde-driven transport contracts
- framework-specific behavior

## Read it like this
- [src/user/README.md](./src/user/README.md)
- [src/chat/README.md](./src/chat/README.md)
- [src/error.rs](./src/error.rs)

## Why this crate exists
If a value survives construction here, the rest of the system can depend on its meaning instead of repeatedly defending against stringly or malformed input.

## Read next
- [app](../app/README.md)
- [root README](../../README.md)
