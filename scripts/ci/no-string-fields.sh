#!/usr/bin/env bash
set -euo pipefail

if command -v rg >/dev/null 2>&1; then
  if rg -U --no-heading --line-number \
    --glob 'crates/**' \
    --glob 'src/**' \
    --glob 'tests/**' \
    'struct\s+\w+\s*\{[^}]*:\s*(?:std::string::)?String' ; then
    echo "\nerror: struct fields must not use String. Use enums or newtypes instead."
    exit 1
  fi
elif command -v grep >/dev/null 2>&1; then
  if grep -RnoE 'struct[[:space:]]+[A-Za-z0-9_]+[[:space:]]*\{[^}]*:[[:space:]]*(std::string::)?String' crates src tests; then
    echo "\nerror: struct fields must not use String. Use enums or newtypes instead."
    exit 1
  fi
else
  echo "error: rg or grep is required for this check."
  exit 1
fi

exit 0
