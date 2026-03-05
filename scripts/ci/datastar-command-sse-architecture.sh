#!/usr/bin/env bash
set -euo pipefail

status=0

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for datastar architecture checks."
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for datastar architecture checks."
  exit 1
fi

# 1) Datastar command handlers (explicitly marked) must be StatusCode-based and non-JSON.
mapfile -t markers < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*datastar-command\s+[A-Za-z_][A-Za-z0-9_]*' \
    crates src generated \
    | sed -E 's#^([^:]+):([0-9]+):.*datastar-command[[:space:]]+([A-Za-z_][A-Za-z0-9_]*).*#\1|\2|\3#'
)

if ((${#markers[@]} == 0)); then
  echo "datastar-architecture: no '// ci: datastar-command <handler>' markers found."
  echo "error: mark Datastar command handlers so CI can enforce command+SSE constraints."
  exit 1
fi

for marker in "${markers[@]}"; do
  file="${marker%%|*}"
  rest="${marker#*|}"
  line="${rest%%|*}"
  handler="${rest##*|}"

  if rg -U --no-heading --line-number \
    "pub\\s+async\\s+fn\\s+${handler}\\b[\\s\\S]{0,260}->\\s*Json<" "$file" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' must not return Json<...>."
    status=1
  fi

  if ! rg -U --no-heading --line-number \
    "pub\\s+async\\s+fn\\s+${handler}\\b[\\s\\S]{0,260}->\\s*(?:axum::http::)?StatusCode" "$file" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' must return StatusCode."
    status=1
  fi

  if ! rg --no-heading --line-number \
    'StatusCode::(NO_CONTENT|ACCEPTED)' "$file" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' must return 204 or 202."
    status=1
  fi

  if ! rg --no-heading --line-number \
    'patch_signals\(|event\("datastar-patch-signals"\)' "$file" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' file must emit datastar-patch-signals."
    status=1
  fi
done

# 2) Generated Datastar handlers must not use Json return types.
if rg --no-heading --line-number -g 'generated/**/handler.rs' -- '->\s*Json<' generated >/dev/null; then
  echo "error: generated Datastar handlers must not return Json<...>; use command + SSE patches."
  status=1
fi

# 3) Datastar specs must enforce command+SSE conventions.
spec_files=()
while IFS= read -r f; do
  spec_files+=("$f")
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

for spec in "${spec_files[@]}"; do
  if ! jq -e '((.meta.target // []) | index("datastar")) != null' "$spec" >/dev/null 2>&1; then
    continue
  fi

  if ! jq -e '.events.app_mappings.backend_responses == []' "$spec" >/dev/null; then
    echo "${spec}: Datastar specs must set events.app_mappings.backend_responses to []."
    status=1
  fi

  if ! jq -e '(.events.handlers // [] | map(select(.class == "app")) | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: Datastar specs must define at least one app handler."
    status=1
  fi

  if ! jq -e '(.events.handlers // [] | map(select(.class == "app") | .trigger == "sse:datastar-patch-signals") | all)' "$spec" >/dev/null; then
    echo "${spec}: app handlers must use trigger sse:datastar-patch-signals."
    status=1
  fi

  if ! jq -e '(.events.app_mappings.sse_events // [] | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: Datastar specs must define sse_events mappings."
    status=1
  fi

  if ! jq -e '(.events.app_mappings.sse_events // [] | map(.event_name == "datastar-patch-signals") | all)' "$spec" >/dev/null; then
    echo "${spec}: sse_events.event_name must be datastar-patch-signals."
    status=1
  fi
done

# 4) Keep Datastar signal names snake_case in specs and generated artifacts.
if rg --no-heading --line-number 'serverCount|serverConnected' generated tests/fixtures >/dev/null; then
  echo "error: camelCase Datastar signal names detected; use snake_case (server_count/server_connected)."
  status=1
fi

exit "$status"
