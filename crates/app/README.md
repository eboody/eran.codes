# app

`app` is the policy and orchestration layer. It turns validated domain values into use cases, coordinates repositories and side-effect mechanisms, and decides which rules are enforced before state changes happen.

## What it owns
- auth, user, and chat services
- traits for repositories, hashers, clocks, id generators, rate limiters, and moderation queues
- contextual errors that describe failed operations rather than transport details

## What it intentionally avoids
- HTTP extractors, cookies, sessions, or response mapping
- direct SQL or storage-specific types
- view concerns

## Read it like this
- [src/auth.rs](./src/auth.rs) for authentication orchestration
- [src/user/README.md](./src/user/README.md) for registration and identity policy
- [src/chat/README.md](./src/chat/README.md) for posting, moderation, and rate limiting

## Why this crate exists
This is where the system decides what is allowed. `http` translates requests into app calls, and `infra` supplies the concrete mechanisms.

## Read next
- [domain](../domain/README.md)
- [infra](../infra/README.md)
