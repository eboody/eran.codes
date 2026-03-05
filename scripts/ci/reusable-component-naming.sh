#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for reusable component naming checks."
  exit 1
fi

status=0
deny_regex='(secure_remote_access|feature_tab|top_feature|screenshot|image_[0-9]+)'
snake_regex='^[a-z][a-z0-9_]*$'

while IFS= read -r spec; do
  has_override="$(jq -r 'has("override")' "$spec")"

  component_id="$(jq -r '.meta.component_id // ""' "$spec")"
  if [[ ! "$component_id" =~ $snake_regex ]]; then
    echo "${spec}: meta.component_id must be snake_case."
    status=1
  fi
  if [[ "$has_override" != "true" ]] && [[ "$component_id" =~ $deny_regex ]]; then
    echo "${spec}: meta.component_id appears request-specific; use generic reusable naming."
    status=1
  fi

  root_type="$(jq -r '.content.root_type // ""' "$spec")"
  root_type_lc="$(printf '%s' "$root_type" | tr '[:upper:]' '[:lower:]')"
  if [[ "$has_override" != "true" ]] && [[ "$root_type_lc" =~ $deny_regex ]]; then
    echo "${spec}: content.root_type appears request-specific; use generic library naming."
    status=1
  fi

  fixture_path="$(jq -r '.content.fixture_path // ""' "$spec")"
  fixture_name="$(basename "$fixture_path")"
  if [[ "$has_override" != "true" ]] && [[ "$fixture_name" =~ $deny_regex ]]; then
    echo "${spec}: content.fixture_path filename appears request-specific; use generic naming."
    status=1
  fi

  if ! jq -e '.design.reuse_scan.checked_components | length > 0' "$spec" >/dev/null; then
    echo "${spec}: design.reuse_scan.checked_components must contain at least one evaluated reusable component."
    status=1
  fi

  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if [[ "$path" != crates/http/src/views/partials/components/* ]]; then
      echo "${spec}: reuse scan entry must point into crates/http/src/views/partials/components: ${path}"
      status=1
      continue
    fi
    if [[ ! -e "$path" ]]; then
      echo "${spec}: reuse scan checked component not found: ${path}"
      status=1
    fi
  done < <(jq -r '.design.reuse_scan.checked_components[]? // empty' "$spec")

  while IFS= read -r name; do
    [[ -z "$name" ]] && continue
    if [[ "$has_override" != "true" ]] && [[ "$name" =~ $deny_regex ]]; then
      echo "${spec}: root-level ui node id '${name}' appears request-specific; use generic naming."
      status=1
    fi
  done < <(jq -r '.ui.nodes[]? | select(.parent_id == null) | .id // empty' "$spec")

  while IFS= read -r name; do
    [[ -z "$name" ]] && continue
    if [[ "$has_override" != "true" ]] && [[ "$name" =~ $deny_regex ]]; then
      echo "${spec}: ui slot '${name}' appears request-specific; use generic naming."
      status=1
    fi
  done < <(jq -r '.ui.slots[]? | .name // empty' "$spec")
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

# Reusable library files should use generic names.
while IFS= read -r file; do
  base="$(basename "$file" .rs)"
  if [[ "$base" =~ $deny_regex ]]; then
    echo "${file}: reusable component filename appears request-specific; use generic naming."
    status=1
  fi
done < <(find crates/http/src/views/partials/components -type f -name '*.rs' 2>/dev/null | sort)

exit "$status"
