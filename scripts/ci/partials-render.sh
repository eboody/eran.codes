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
  mapfile -t structs < <(rg -N --no-heading "pub struct ([A-Za-z0-9_]+)" "$root" \
    | sed -E 's/.*pub struct ([A-Za-z0-9_]+).*/\1/' \
    | sort -u)
else
  mapfile -t structs < <(grep -Rno "pub struct [A-Za-z0-9_]\+" "$root" \
    | sed -E 's/.*pub struct ([A-Za-z0-9_]+).*/\1/' \
    | sort -u)
fi

missing=()
for name in "${structs[@]}"; do
  if command -v rg >/dev/null 2>&1; then
    if ! rg -N "impl\\s+(?:maud::)?Render\\s+for\\s+${name}(?:\\s*<[^>]*>)?" "$root" >/dev/null; then
      missing+=("$name")
    fi
  else
    if ! grep -RnoE "impl[[:space:]]+(maud::)?Render[[:space:]]+for[[:space:]]+${name}(<[^>]*>)?" "$root" >/dev/null; then
      missing+=("$name")
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
