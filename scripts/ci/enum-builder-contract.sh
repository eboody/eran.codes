#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for enum-builder checks."
  exit 1
fi

status=0

semantic_string_pattern='(?:pub|pub\([^)]*\))\s+\w*(?:type|mode|kind|variant|role|scope|authority)\w*\s*:\s*(?:Option<)?(?:crate::types::Text|Text|String|&str|&'"'"'static str)(?:>)?'

if rg --no-heading --line-number \
  --glob 'crates/http/src/views/**/*.rs' \
  -e "$semantic_string_pattern"; then
  echo "\nerror: semantic finite-set fields must use enums/newtypes, not Text/String."
  status=1
fi

mapfile -t composition_files < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*render-composition-component\b' \
    crates/http/src/views \
    | cut -d: -f1 \
    | sort -u
)

for file in "${composition_files[@]}"; do
  if rg --no-heading --line-number '^\s*//\s*ci:\s*bon-builder-exempt\b' "$file" >/dev/null; then
    continue
  fi

  if ! rg --no-heading --line-number '#\[derive\([^\]]*\bBuilder\b' "$file" >/dev/null \
    && ! rg --no-heading --line-number '^\s*#\[bon\]' "$file" >/dev/null; then
    echo "${file}: expected bon-based builder usage (derive(Builder) or #[bon]) or // ci: bon-builder-exempt"
    status=1
  fi

done

exit "$status"
