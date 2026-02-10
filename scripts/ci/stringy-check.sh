#!/usr/bin/env bash
set -euo pipefail

patterns=(
  '==\s*"'
  '!=\s*"'
  'starts_with\("'
  'starts_with\(r#"'
  'ends_with\("'
  'ends_with\(r#"'
  'contains\("'
  'contains\(r#"'
)

args=()
for pattern in "${patterns[@]}"; do
  args+=( -e "$pattern" )
done

if rg --no-heading --line-number \
  --glob 'crates/**' \
  --glob 'src/**' \
  --glob 'tests/**' \
  "${args[@]}"; then
  echo "\nerror: string literal checks found. Use enums/newtypes instead."
  exit 1
fi

exit 0
