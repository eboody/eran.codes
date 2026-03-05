#!/usr/bin/env bash
set -euo pipefail

visual_path_pattern='^(crates/http/src/views/|crates/http/static/)'
audit_dir="${VISUAL_AUDIT_DIR:-artifacts/visual/audits/latest}"
manifest="$audit_dir/signoff.env"
ux_file="$audit_dir/ux-signoff.md"
ui_file="$audit_dir/ui-signoff.md"
fullstack_file="$audit_dir/fullstack-rust-hiring-manager-signoff.md"
systems_file="$audit_dir/rust-systems-hiring-manager-signoff.md"

changed_files="${VISUAL_CHANGED_FILES:-}"
if [[ -z "$changed_files" ]]; then
  changed_files="$(git diff --name-only HEAD || true)"
fi
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

fail=0
require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "visual-expert-signoff: missing $path"
    fail=1
  fi
}

require_file "$manifest"
require_file "$ux_file"
require_file "$ui_file"
require_file "$fullstack_file"
require_file "$systems_file"

if [[ "$fail" -ne 0 ]]; then
  echo "visual-expert-signoff: failed"
  exit 1
fi

set -a
source "$manifest"
set +a

expected_commit_sha="${VISUAL_EXPECTED_COMMIT_SHA:-$(git rev-parse HEAD)}"
expected_tree_sha="${VISUAL_EXPECTED_TREE_SHA:-$(git rev-parse HEAD^{tree})}"

require_env() {
  local key="$1"
  local expected="$2"
  local actual="${!key:-}"
  if [[ "$actual" != "$expected" ]]; then
    echo "visual-expert-signoff: expected $key=$expected but got ${actual:-<unset>}"
    fail=1
  fi
}

require_env_nonempty() {
  local key="$1"
  local actual="${!key:-}"
  if [[ -z "$actual" ]]; then
    echo "visual-expert-signoff: expected $key to be set"
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

require_snapshot_file() {
  local key="$1"
  local path="${!key:-}"
  if [[ -z "$path" ]]; then
    echo "visual-expert-signoff: expected $key to be set"
    fail=1
    return
  fi
  if [[ ! -f "$path" ]]; then
    echo "visual-expert-signoff: snapshot for $key not found ($path)"
    fail=1
  fi
}

require_env UX_RESULT pass
require_env UI_RESULT pass
require_env FULLSTACK_RUST_HM_RESULT pass
require_env RUST_SYSTEMS_HM_RESULT pass
require_env UX_AGENT_AVAILABLE true
require_env UI_AGENT_AVAILABLE true
require_env FULLSTACK_RUST_HM_AGENT_AVAILABLE true
require_env RUST_SYSTEMS_HM_AGENT_AVAILABLE true
require_env REVIEW_SCOPE visual
require_env REVIEWED_COMMIT_SHA "$expected_commit_sha"
require_env REVIEWED_TREE_SHA "$expected_tree_sha"
require_env_nonempty REVIEWED_AT

if ! git rev-parse --verify "${REVIEWED_COMMIT_SHA}^{commit}" >/dev/null 2>&1; then
  echo "visual-expert-signoff: REVIEWED_COMMIT_SHA is not a valid commit (${REVIEWED_COMMIT_SHA})"
  fail=1
fi

require_file_pattern "$ux_file" '^reviewer:\s*ux-expert$' "missing ux reviewer tag"
require_file_pattern "$ux_file" '^agent_status:\s*available$' "ux agent unavailable"
require_file_pattern "$ux_file" '^result:\s*pass$' "ux result is not pass"
require_file_pattern "$ux_file" "^reviewed_commit_sha:\\s*${REVIEWED_COMMIT_SHA}$" "ux reviewed commit mismatch"
require_file_pattern "$ux_file" '^snapshot_set:\s*\S+$' "ux snapshot_set missing"
require_file_pattern "$ux_file" '^snapshot_set:.*home\.png.*showcase_tab_0\.png.*showcase_tab_1\.png.*showcase_tab_2\.png.*showcase_tab_3\.png.*$' "ux snapshot_set must include home + tab 0..3 captures"
require_file_pattern "$ux_file" '^component:\s*\S+$' "ux component missing"

require_file_pattern "$ui_file" '^reviewer:\s*ui-expert$' "missing ui reviewer tag"
require_file_pattern "$ui_file" '^agent_status:\s*available$' "ui agent unavailable"
require_file_pattern "$ui_file" '^result:\s*pass$' "ui result is not pass"
require_file_pattern "$ui_file" "^reviewed_commit_sha:\\s*${REVIEWED_COMMIT_SHA}$" "ui reviewed commit mismatch"
require_file_pattern "$ui_file" '^snapshot_set:\s*\S+$' "ui snapshot_set missing"
require_file_pattern "$ui_file" '^snapshot_set:.*home\.png.*showcase_tab_0\.png.*showcase_tab_1\.png.*showcase_tab_2\.png.*showcase_tab_3\.png.*$' "ui snapshot_set must include home + tab 0..3 captures"
require_file_pattern "$ui_file" '^component:\s*\S+$' "ui component missing"

require_file_pattern "$fullstack_file" '^reviewer:\s*fullstack-rust-hiring-manager-expert$' "missing fullstack reviewer tag"
require_file_pattern "$fullstack_file" '^agent_status:\s*available$' "fullstack agent unavailable"
require_file_pattern "$fullstack_file" '^result:\s*pass$' "fullstack result is not pass"
require_file_pattern "$fullstack_file" "^reviewed_commit_sha:\\s*${REVIEWED_COMMIT_SHA}$" "fullstack reviewed commit mismatch"
require_file_pattern "$fullstack_file" '^snapshot_set:\s*\S+$' "fullstack snapshot_set missing"
require_file_pattern "$fullstack_file" '^snapshot_set:.*home\.png.*showcase_tab_0\.png.*showcase_tab_1\.png.*showcase_tab_2\.png.*showcase_tab_3\.png.*$' "fullstack snapshot_set must include home + tab 0..3 captures"
require_file_pattern "$fullstack_file" '^component:\s*\S+$' "fullstack component missing"

require_file_pattern "$systems_file" '^reviewer:\s*rust-systems-engineer-hiring-manager-expert$' "missing rust-systems reviewer tag"
require_file_pattern "$systems_file" '^agent_status:\s*available$' "rust-systems agent unavailable"
require_file_pattern "$systems_file" '^result:\s*pass$' "rust-systems result is not pass"
require_file_pattern "$systems_file" "^reviewed_commit_sha:\\s*${REVIEWED_COMMIT_SHA}$" "rust-systems reviewed commit mismatch"
require_file_pattern "$systems_file" '^snapshot_set:\s*\S+$' "rust-systems snapshot_set missing"
require_file_pattern "$systems_file" '^snapshot_set:.*home\.png.*showcase_tab_0\.png.*showcase_tab_1\.png.*showcase_tab_2\.png.*showcase_tab_3\.png.*$' "rust-systems snapshot_set must include home + tab 0..3 captures"
require_file_pattern "$systems_file" '^component:\s*\S+$' "rust-systems component missing"

require_snapshot_file SNAPSHOT_HOME
require_snapshot_file SNAPSHOT_SHOWCASE_TAB_0
require_snapshot_file SNAPSHOT_SHOWCASE_TAB_1
require_snapshot_file SNAPSHOT_SHOWCASE_TAB_2
require_snapshot_file SNAPSHOT_SHOWCASE_TAB_3

if [[ "$fail" -ne 0 ]]; then
  echo "visual-expert-signoff: failed"
  exit 1
fi

echo "visual-expert-signoff: pass"
