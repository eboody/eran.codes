# Stringy Checks Audit (2026-02-10)

## Goal
Remove direct string literal checks in runtime logic and replace them with enums/newtypes to make intent explicit and resilient.

## Findings
A repo-wide scan for direct string comparisons and stringy prefix/suffix checks returned no remaining occurrences:

```
rg -n "== \"|!= \"|starts_with\(\"|ends_with\(\"|contains\(\"" crates
```

## Key refactors applied
- Trace log filtering now uses `LogTargetKind` + `LogMessageKind` enums.
- Field name selection uses `FieldName` enum (no direct string comparisons).
- Sentinel values (`"-"`) are wrapped in `FieldValue` enum, so callers deal in `Option` and typed variants.
- Link moderation uses `LinkPrefix` enum (app/chat).
- Redirect “next” sanitization uses `NextPath` newtype (handlers/auth).
- Chat sender handling uses `ChatSender` enum where applicable.
- All log/UI pills are enum-driven (`PillVariant` and `BadgeKind`).

## Remaining string usage
String literals remain only as *data* (labels, copy, routes) rather than control flow checks.

If you want further tightening (e.g., converting static copy into enums/constants for tests), we can do that too, but core logic is now enum-driven.
