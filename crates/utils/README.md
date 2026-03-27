# utils

`utils` is the small shared-tools layer. It stays deliberately narrow so common helpers do not collapse the main boundary structure.

## What it owns
- cross-cutting helpers with no domain/app/http/infra allegiance
- environment and encoding utilities
- developer tooling binaries such as visual snapshot capture for guest and signed-in proof routes

## Read it like this
- [src/lib.rs](./src/lib.rs)
- [src/envs.rs](./src/envs.rs)
- [src/bin/visual_snapshot.rs](./src/bin/visual_snapshot.rs)

## Why this crate exists
It gives the workspace a place for genuinely reusable support code without turning `http` or `infra` into a dumping ground.

## Read next
- [root README](../../README.md)
