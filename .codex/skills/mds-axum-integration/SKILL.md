---
name: mds-axum-integration
version: 0.1.0
description: Axum integration patterns (routing, extractors, SSE endpoints, response mapping) for this repo.
scope: project
---

# mds-axum-integration

## Purpose
Standardize Axum integration decisions for backend contracts, routing, request extraction, and SSE transport for Datastar-connected components.

## Usage
- Use before defining `backend_contracts.actions` and routing/transport assumptions.
- Apply SSE-specific docs for stream semantics.
- Escalate conflicting route patterns via `mds-docs-librarian` and verifier gate.
