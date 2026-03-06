#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for render-composition checks."
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for render-composition checks."
  exit 1
fi

status=0

while IFS= read -r spec; do
  if ! jq -e '((.meta.target // []) | index("datastar")) != null' "$spec" >/dev/null 2>&1; then
    continue
  fi

  if ! jq -e '.design.render_contract.composable_render == true' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.composable_render must be true."
    status=1
  fi

  if ! jq -e '.design.render_contract.children_as_props == true' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.children_as_props must be true."
    status=1
  fi

  if ! jq -e '(.design.render_contract.primitive_reuse // [] | type == "array")' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.primitive_reuse must be an array."
    status=1
  fi

  if ! jq -e '(.design.render_contract.interaction_mode | type == "string" and length > 0)' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.interaction_mode must be a non-empty string."
    status=1
  fi
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

mapfile -t composition_files < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*render-composition-component\b' \
    crates/http/src/views \
    | cut -d: -f1 \
    | sort -u
)

if ((${#composition_files[@]} == 0)); then
  echo "render-composition-contract: no files marked with '// ci: render-composition-component'."
  exit 1
fi

for file in "${composition_files[@]}"; do
  if ! rg --no-heading --line-number 'impl\s+(?:maud::)?Render\s+for' "$file" >/dev/null; then
    echo "${file}: expected at least one Render impl."
    status=1
  fi

  if ! rg --no-heading --line-number 'pub\s+children:\s*(?:&'"'"'a\s*\[[^]]+\]|Vec<[^>]+>)' "$file" >/dev/null \
    && ! rg --no-heading --line-number "pub\\s+(tabs|panels):\\s+[A-Za-z0-9_]+<'a>" "$file" >/dev/null; then
    echo "${file}: expected child composition props (children/tabs/panels)."
    status=1
  fi

  if ! rg --no-heading --line-number '@for\s+.+\s+in\s+.+(children|tabs|panels)' "$file" >/dev/null; then
    echo "${file}: expected child iteration/composition in render output."
    status=1
  fi
done

exit "$status"
