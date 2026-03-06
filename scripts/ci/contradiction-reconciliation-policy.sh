#!/usr/bin/env bash
set -euo pipefail

status=0

require_pattern() {
  local file="$1"
  local pattern="$2"
  local message="$3"
  if ! rg --no-heading --line-number --fixed-strings "$pattern" "$file" >/dev/null; then
    echo "${file}: ${message}"
    status=1
  fi
}

require_pattern \
  ".codex/AGENTS.md" \
  "If a new user prompt conflicts with previously established user instructions or accepted architecture constraints, agents must pause and ask the user to reconcile the conflict before implementing." \
  "missing stop-the-line contradiction reconciliation rule"

require_pattern \
  ".codex/AGENTS.md" \
  "Agents must not treat unrelated or ambiguous prompts as implicit permission to override existing instructions." \
  "missing explicit prohibition on implicit overrides"

require_pattern \
  ".codex/agents/mds-orchestrator/AGENT.md" \
  "Must not proceed when request intent conflicts with established instructions/policies; require explicit user reconciliation first." \
  "orchestrator must block on unresolved prompt conflicts"

require_pattern \
  ".codex/agents/mds-verifier/AGENT.md" \
  "MUST FAIL when unresolved prompt contradictions are detected (missing explicit user reconciliation)." \
  "verifier must fail unresolved prompt contradictions"

exit "$status"
