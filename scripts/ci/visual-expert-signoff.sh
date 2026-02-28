#!/usr/bin/env bash
set -euo pipefail

visual_path_pattern='^(crates/http/src/views/|crates/http/static/)'
audit_dir='docs/visual-signoff/latest'
manifest="$audit_dir/signoff.env"
ux_file="$audit_dir/ux-signoff.md"
ui_file="$audit_dir/ui-signoff.md"

changed_files="${VISUAL_CHANGED_FILES:-}"
if [[ -z "$changed_files" ]]; then
  changed_files="$(git show --pretty='' --name-only HEAD || true)"
fi

if [[ -z "$changed_files" ]]; then
  echo "visual-expert-signoff: no changed files detected; skipping"
  exit 0
fi

if ! printf '%s\n' "$changed_files" | rg -q "$visual_path_pattern"; then
  echo "visual-expert-signoff: no visual file changes; skipping"
  exit 0
fi

if [[ ! -f "$manifest" ]]; then
  echo "visual-expert-signoff: missing $manifest"
  exit 1
fi
if [[ ! -f "$ux_file" ]]; then
  echo "visual-expert-signoff: missing $ux_file"
  exit 1
fi
if [[ ! -f "$ui_file" ]]; then
  echo "visual-expert-signoff: missing $ui_file"
  exit 1
fi

set -a
source "$manifest"
set +a

fail=0
require_env() {
  local key="$1"
  local expected="$2"
  local actual="${!key:-}"
  if [[ "$actual" != "$expected" ]]; then
    echo "visual-expert-signoff: expected $key=$expected but got ${actual:-<unset>}"
    fail=1
  fi
}

require_file_pattern() {
  local file="$1"
  local pattern="$2"
  local message="$3"
  if ! rg -q "$pattern" "$file"; then
    echo "visual-expert-signoff: $message ($file)"
    fail=1
  fi
}

require_env UX_RESULT pass
require_env UI_RESULT pass
require_env UX_AGENT_AVAILABLE true
require_env UI_AGENT_AVAILABLE true

require_file_pattern "$ux_file" '^reviewer:\s*ux-expert$' "missing ux reviewer tag"
require_file_pattern "$ux_file" '^agent_status:\s*available$' "ux agent unavailable"
require_file_pattern "$ux_file" '^result:\s*pass$' "ux result is not pass"

require_file_pattern "$ui_file" '^reviewer:\s*ui-expert$' "missing ui reviewer tag"
require_file_pattern "$ui_file" '^agent_status:\s*available$' "ui agent unavailable"
require_file_pattern "$ui_file" '^result:\s*pass$' "ui result is not pass"

if [[ "$fail" -ne 0 ]]; then
  echo "visual-expert-signoff: failed"
  exit 1
fi

echo "visual-expert-signoff: pass"
