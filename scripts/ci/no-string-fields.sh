#!/usr/bin/env bash
set -euo pipefail

if rg -U --no-heading --line-number \
  --glob 'crates/**' \
  --glob 'src/**' \
  --glob 'tests/**' \
  'struct\s+\w+\s*\{[^}]*:\s*(?:std::string::)?String' ; then
  echo "\nerror: struct fields must not use String. Use enums or newtypes instead."
  exit 1
fi

exit 0
