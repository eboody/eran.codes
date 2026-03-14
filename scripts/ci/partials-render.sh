#!/usr/bin/env bash
set -euo pipefail

root="crates/http/src/views/partials"

if ! command -v rg >/dev/null 2>&1; then
  if ! command -v grep >/dev/null 2>&1; then
    echo "error: rg or grep is required for this check."
    exit 1
  fi
fi

if command -v rg >/dev/null 2>&1; then
  mapfile -t structs < <(rg -N --no-heading --line-number "^[[:space:]]*pub struct ([A-Za-z0-9_]+)" "$root" \
    | sed -E 's#^([^:]+):([0-9]+):.*pub struct ([A-Za-z0-9_]+).*$#\1|\2|\3#' \
    | sort -u)
else
  mapfile -t structs < <(grep -Rno "^[[:space:]]*pub struct [A-Za-z0-9_]\+" "$root" \
    | sed -E 's#^([^:]+):([0-9]+):.*pub struct ([A-Za-z0-9_]+).*$#\1|\2|\3#' \
    | sort -u)
fi

missing=()
for entry in "${structs[@]}"; do
  file="${entry%%|*}"
  rest="${entry#*|}"
  line="${rest%%|*}"
  name="${rest##*|}"

  if command -v rg >/dev/null 2>&1; then
    if rg -N "^\s*//\s*ci:\s*partials-render-file-exempt\b" "$file" >/dev/null; then
      continue
    fi

    if sed -n "$((line - 2)),$((line - 1))p" "$file" 2>/dev/null | rg -N "^\s*//\s*ci:\s*partials-render-exempt\b" >/dev/null; then
      continue
    fi
  fi

  if command -v rg >/dev/null 2>&1; then
    if ! rg -N "impl\\s+(?:maud::)?Render\\s+for\\s+${name}(?:\\s*<[^>]*>)?" "$root" >/dev/null; then
      missing+=("${name} (${file}:${line})")
    fi
  else
    if ! grep -RnoE "impl[[:space:]]+(maud::)?Render[[:space:]]+for[[:space:]]+${name}(<[^>]*>)?" "$root" >/dev/null; then
      missing+=("${name} (${file}:${line})")
    fi
  fi
done

if ((${#missing[@]} > 0)); then
  echo "error: missing Render impls for pub partial components:"
  for name in "${missing[@]}"; do
    echo "  - ${name}"
  done
  exit 1
fi

exit 0
