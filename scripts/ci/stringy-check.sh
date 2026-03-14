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

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg or grep is required for this check."
  exit 1
fi

status=0

while IFS= read -r hit; do
  file=${hit%%:*}
  rest=${hit#*:}
  line=${rest%%:*}
  content=${rest#*:}

  if [[ "$file" == */tests.rs ]]; then
    continue
  fi

  if [[ "$content" =~ (^|[^[:alnum:]_])(assert|debug_assert)(_eq|_ne)?!\( ]]; then
    continue
  fi

  if rg --quiet '^\s*//\s*ci:\s*string-literal-check-exempt\b' "$file"; then
    continue
  fi

  echo "${file}:${line}:${content}"
  status=1
done < <(
  rg --no-heading --line-number \
    --glob 'crates/**/views/**' \
    "${args[@]}" || true
)

if [[ "$status" -ne 0 ]]; then
  echo "\nerror: string literal checks found. Use enums/newtypes instead."
  exit 1
fi

exit 0
