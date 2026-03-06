#!/usr/bin/env bash
set -euo pipefail

status=0

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for runtime/generated duplication checks."
  exit 1
fi

while IFS= read -r component_entry; do
  if [[ -f "$component_entry" ]]; then
    base="$(basename "$component_entry" .rs)"
  elif [[ -d "$component_entry" ]]; then
    [[ -f "${component_entry}/mod.rs" ]] || continue
    base="$(basename "$component_entry")"
  else
    continue
  fi

  generated_dir="generated/${base}"
  [[ -d "$generated_dir" ]] || continue

  while IFS= read -r dup_file; do
    [[ -z "$dup_file" ]] && continue
    echo "${dup_file}: duplicate generated implementation for runtime reusable component '${base}'."
    echo "Remove generated .rs implementation files after integrating '${base}' into crates/http/src/views/partials/components."
    status=1
  done < <(find "$generated_dir" -maxdepth 1 -type f -name '*.rs' 2>/dev/null | sort)
done < <(
  {
    find crates/http/src/views/partials/components -maxdepth 1 -type f -name '*.rs' 2>/dev/null
    find crates/http/src/views/partials/components -maxdepth 1 -type d 2>/dev/null
  } | sort -u
)

exit "$status"
