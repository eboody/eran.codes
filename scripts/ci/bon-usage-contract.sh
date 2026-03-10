#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for bon usage checks."
  exit 1
fi

status=0
pattern='\.maybe_[A-Za-z0-9_]+\s*\(\s*Some\s*\('

# Concrete values in Rust code should use direct setters, not maybe_*(Some(...)).
if rg --pcre2 -U --no-heading --line-number \
  --glob 'crates/**/*.rs' \
  "$pattern"; then
  echo
  echo "error: replace .maybe_*(Some(value)) in Rust code with direct setters (.field(value))."
  status=1
fi

# Bon docs examples should follow the same rule.
if rg --pcre2 -U --no-heading --line-number \
  --glob 'docs/bon/**/*.md' \
  "$pattern"; then
  echo
  echo "error: replace .maybe_*(Some(value)) in docs/bon examples with direct setters (.field(value))."
  status=1
fi

# Bon docs must remain in the docs lookup allowlist.
if ! rg --quiet '/docs/bon/' .codex/skills/mds-repo-docs-index/SKILL.md; then
  echo "error: mds-repo-docs-index must include /docs/bon/ in authoritative doc roots."
  status=1
fi

# Bon skill must exist and be required by the local agent policy.
if [[ ! -f ".codex/skills/mds-bon-patterns/SKILL.md" ]]; then
  echo "error: missing bon skill at .codex/skills/mds-bon-patterns/SKILL.md"
  status=1
fi

if ! rg --quiet '\.codex/skills/mds-bon-patterns/SKILL\.md' .codex/AGENTS.md; then
  echo "error: .codex/AGENTS.md must require mds-bon-patterns in docs-first rules."
  status=1
fi

# Bon setter guidance must be explicit in builder-related skills/docs.
if ! rg --quiet 'maybe_field\(Some\(value\)\)' .codex/skills/mds-maud-patterns/SKILL.md; then
  echo "error: mds-maud-patterns must document the maybe_field(Some(value)) anti-pattern."
  status=1
fi

if ! rg --quiet 'maybe_field\(Some\(value\)\)' .codex/skills/mds-component-spec/SKILL.md; then
  echo "error: mds-component-spec must document the maybe_field(Some(value)) anti-pattern."
  status=1
fi

if ! rg --quiet 'maybe_field\(Some\(value\)\)' docs/writing-style.md; then
  echo "error: docs/writing-style.md must document the maybe_field(Some(value)) anti-pattern."
  status=1
fi

exit "$status"
