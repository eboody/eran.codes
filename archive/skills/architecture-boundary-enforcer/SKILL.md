---
name: architecture-boundary-enforcer
description: Enforce clean boundaries between domain, app, http, and infra, including auth/session/security ownership and DTO/model separation.
---

# Architecture Boundary Enforcer

## When To Use

Use this skill for:
- New endpoints or request flows
- DTO/command/entity/row type design
- Auth/session/security behavior changes
- Repository/trait placement and dependency direction
- Persistence mapping changes

## Ownership

This specialist owns:
- Boundary map across `domain`, `app`, `http`, `infra`
- Type ownership at each boundary
- Auth/password handling flow and failure mapping

## Local Sources First

Before proposing changes, consult `docs/reference-map.md` and then read the smallest relevant set from:
- `docs/writing-style.md`
- `docs/project-audit.md`
- `docs/refactor-plan.md`
- `docs/code-audit/01-architecture-map.md`
- `docs/auth-sessions.md`

## Workflow

1. Load local docs from `docs/reference-map.md` and note which files apply.
2. Map the flow:
   - request DTO -> app command -> domain entities -> infra rows -> response DTO
3. Validate dependency direction:
   - `domain` has no HTTP/DB/framework details
   - `app` orchestrates policy and defines external traits
   - `http` handles transport mapping and response shaping
   - `infra` implements app traits and external mechanisms
4. Validate validation placement:
   - pure/stable invariants in domain newtypes
   - contextual checks in app services
5. Validate auth handling:
   - plaintext password only in HTTP parsing + app service scope
   - hashing via app trait, infra implementation
   - no plaintext password accepted by repositories
6. Validate error boundaries:
   - infra errors mapped to app errors before HTTP status mapping

## Boundary Checklist

- Domain types do not derive serde for transport convenience.
- Domain/app types do not import transport or persistence details.
- HTTP DTOs are network-shaped; domain invariants are not HTTP-shaped.
- DB row structs stay in infra.
- Credential data is never exposed by response DTOs.

## Output Contract

Always return:
- `boundary_map`: crate-level ownership of each type group
- `minimal_types_traits`: minimum structs/enums/traits needed
- `flow`: request -> app -> domain -> infra -> response summary
- `failure_modes`: key failures and where each is handled
- `sources_used`: exact local files consulted

## Guardrails

- Prefer the smallest type set that preserves boundary safety.
- Reject convenience shortcuts that leak HTTP/DB concerns inward.
- Keep recommendations implementable in current workspace structure.
