#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for cms-content-model checks."
  exit 1
fi

status=0

# Files opt into this rule with:
#   // ci: cms-content-component
mapfile -t marked_files < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*cms-content-component\b' \
    crates/http/src/views generated \
    | cut -d: -f1 \
    | sort -u
)

if ((${#marked_files[@]} == 0)); then
  echo "cms-content-model: no files marked with '// ci: cms-content-component'; skipping."
  exit 0
fi

for file in "${marked_files[@]}"; do
  # Long human-readable copy should come from typed *Content + fixture, not inline literals.
  # We flag long string literals with spaces, excluding common placeholder/debug allowance.
  while IFS= read -r hit; do
    lineno="${hit%%:*}"
    text="${hit#*:}"

    if [[ "$text" =~ [Pp]laceholder|[Dd]ebug|TODO ]]; then
      continue
    fi

    echo "${file}:${lineno}: hardcoded copy in cms-content component template."
    echo "error: move copy into typed *Content populated by fixture."
    status=1
  done < <(
    rg --no-heading --line-number \
      '"[^"\n]{24,}"' "$file" \
      | awk -F: '
          {
            line=$0
            # Keep only likely copy strings with spaces and letters.
            if (line ~ /"[A-Za-z][^"]* [^"]*"/) print $2 ":" substr(line, index(line,$3))
          }
        '
  )
done

exit "$status"
