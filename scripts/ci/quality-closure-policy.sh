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
  "Agents must not present output as complete/final when they are aware of material quality gaps" \
  "missing quality-closure guardrail against premature completion"

require_pattern \
  ".codex/AGENTS.md" \
  "When such gaps remain, agents must explicitly call them out and ask for another pass before treating the task as done." \
  "missing mandatory next-pass prompt requirement"

require_pattern \
  ".codex/agents/mds-orchestrator/AGENT.md" \
  "Must not declare the run complete when known quality gaps remain; require explicit next-pass proposal." \
  "orchestrator must require next-pass handoff for unresolved quality gaps"

require_pattern \
  ".codex/agents/mds-verifier/AGENT.md" \
  "MUST FAIL when unresolved material quality gaps are present without explicit next-pass handoff." \
  "verifier must fail unresolved quality gaps lacking next-pass handoff"

require_pattern \
  ".codex/skills/mds-component-spec/SKILL.md" \
  "Do not mark a component request as fully complete if known material gaps remain." \
  "component-spec skill missing quality-closure completion policy"

exit "$status"
