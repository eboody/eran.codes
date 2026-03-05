---
name: animejs
description: Source-grounded anime.js guidance using a local mirror of official docs and references (v4, v3, v2, learn, and demos). Use when implementing, debugging, or reviewing anime.js code.
---

# Anime.js

## When To Use

Use this skill when work involves anime.js:
- Writing or refactoring animations
- Verifying parameter names, defaults, methods, callbacks, and utilities
- Migrating between anime.js versions
- Checking examples from official docs/demos before implementation

## Local Reference Set

Primary reference roots in this repo:
- `skills/animejs/references/animejs-site/animejs.com/documentation/**` (v4)
- `skills/animejs/references/animejs-site/animejs.com/v3/documentation/**` (v3)
- `skills/animejs/references/animejs-site/animejs.com/v2/documentation/**` (v2)
- `skills/animejs/references/animejs-site/animejs.com/learn/**`
- `skills/animejs/references/animejs-site/animejs.com/documentation-demos/**`

Useful indexes:
- `skills/animejs/references/html-files.txt`
- `skills/animejs/references/stats.env`

## Workflow

1. Default to v4 docs unless the codebase clearly targets v3/v2 behavior.
2. Search local mirrored docs first:
   - `skills/animejs/scripts/search.sh '<term>'`
3. Read the relevant mirrored page(s) before changing code.
4. Implement using API names and semantics exactly as documented.
5. If docs appear stale or missing, refresh mirror:
   - `skills/animejs/scripts/refresh_docs.sh`

## Output Expectations

- Keep recommendations source-grounded to local mirrored docs.
- Prefer explicit API names/options over memory-based guesses.
- When behavior differs by version, call it out and name the version.
- Include `sources_used` with exact local file paths consulted.
