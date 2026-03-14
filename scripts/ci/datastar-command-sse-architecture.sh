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

declare -A seen_files=()

add_candidate_file() {
  local candidate=$1
  [[ -n "$candidate" && -f "$candidate" ]] || return 0
  seen_files["$candidate"]=1
}

local_module_file() {
  local base_dir=$1
  local module_name=$2

  if [[ -f "$base_dir/${module_name}.rs" ]]; then
    printf '%s\n' "$base_dir/${module_name}.rs"
    return 0
  fi

  if [[ -f "$base_dir/${module_name}/mod.rs" ]]; then
    printf '%s\n' "$base_dir/${module_name}/mod.rs"
    return 0
  fi

  return 1
}

handler_scope() {
  local file=$1
  local handler=$2

  awk -v handler="$handler" '
    !capturing && $0 ~ "pub[[:space:]]+async[[:space:]]+fn[[:space:]]+" handler "[[:space:]]*\\(" {
      capturing = 1
    }

    capturing && $0 ~ "^[[:space:]]*pub[[:space:]]+async[[:space:]]+fn[[:space:]]+" &&
      $0 !~ "pub[[:space:]]+async[[:space:]]+fn[[:space:]]+" handler "[[:space:]]*\\(" {
      exit
    }

    capturing { print }
  ' "$file"
}

unique_definition_file() {
  local symbol=$1
  local -a matches=()

  mapfile -t matches < <(
    rg --no-heading --line-number -g '*.rs' \
      "fn[[:space:]]+${symbol}\\b" crates src generated \
      | cut -d: -f1 \
      | awk '!seen[$0]++'
  )

  if ((${#matches[@]} == 1)); then
    printf '%s\n' "${matches[0]}"
  fi
}

implementation_files_for_handler() {
  local file=$1
  local handler=$2
  local base_dir
  local scope
  local local_modules=()
  local symbol_files=()
  local symbol
  local resolved

  seen_files=()
  add_candidate_file "$file"

  base_dir=$(dirname "$file")
  scope=$(handler_scope "$file" "$handler")

  while IFS= read -r module_name; do
    [[ -n "$module_name" ]] || continue
    resolved=$(local_module_file "$base_dir" "$module_name" || true)
    [[ -n "$resolved" ]] || continue
    add_candidate_file "$resolved"
    local_modules+=("$resolved")
  done < <(
    printf '%s\n' "$scope" \
      | grep -oE '\b[A-Za-z_][A-Za-z0-9_]*::' \
      | sed 's/::$//' \
      | awk '!seen[$0]++'
  )

  while IFS= read -r symbol; do
    [[ -n "$symbol" ]] || continue
    resolved=$(unique_definition_file "$symbol" || true)
    [[ -n "$resolved" ]] || continue
    add_candidate_file "$resolved"
  done < <(
    {
      printf '%s\n' "$scope"
      for module_file in "${local_modules[@]}"; do
        sed -n '1,240p' "$module_file"
      done
    } \
      | grep -oE '(\.|::)[A-Za-z_][A-Za-z0-9_]*\(' \
      | sed -E 's/^(\.|::)([A-Za-z_][A-Za-z0-9_]*)\($/\2/' \
      | awk '!seen[$0]++'
  )

  printf '%s\n' "${!seen_files[@]}" | sort
}

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
  mapfile -t implementation_files < <(implementation_files_for_handler "$file" "$handler")

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
    'StatusCode::(NO_CONTENT|ACCEPTED)' "${implementation_files[@]}" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' must return 204 or 202."
    status=1
  fi

  if ! rg --no-heading --line-number \
    'patch_signals\(|event\("datastar-patch-signals"\)|patch_elements\(' "${implementation_files[@]}" >/dev/null; then
    echo "${file}:${line}: Datastar command '${handler}' implementation must emit an SSE patch event."
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
