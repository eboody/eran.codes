#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for style-system checks."
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for style-system checks."
  exit 1
fi

status=0

APP_CSS="crates/http/static/app.css"
if [[ ! -f "$APP_CSS" ]]; then
  echo "error: expected global stylesheet at ${APP_CSS}."
  exit 1
fi

for cls in ".ui-surface-card" ".ui-tabs" ".ui-tab" ".ui-panel" ".ui-preview-frame" ".ui-feature-list" ".ui-cta"; do
  if ! rg -q "^\s*\\${cls}\b|^\s*${cls}\b" "$APP_CSS"; then
    echo "${APP_CSS}: missing reusable package class ${cls}."
    status=1
  fi
done

while IFS= read -r spec; do
  if ! jq -e '((.meta.target // []) | index("datastar")) != null' "$spec" >/dev/null 2>&1; then
    continue
  fi

  if ! jq -e '.styling.mode == "hybrid"' "$spec" >/dev/null; then
    echo "${spec}: styling.mode must be \"hybrid\"."
    status=1
  fi

  if ! jq -e '(.styling.global_packages // [] | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: styling.global_packages must include reusable classes."
    status=1
  fi

  if ! jq -e '(.styling.tokens_used // [] | length) > 0' "$spec" >/dev/null; then
    echo "${spec}: styling.tokens_used must declare consumed tokens."
    status=1
  fi

  if ! jq -e '(.pipeline.required_agents // []) | index("mds-styling-system") != null' "$spec" >/dev/null; then
    echo "${spec}: pipeline.required_agents must include mds-styling-system."
    status=1
  fi

  if ! jq -e '
    (.pipeline.execution_order // []) as $order
    | ($order | index("mds-codegen")) as $codegen
    | ($order | index("mds-styling-system")) as $styling
    | ($order | index("mds-verifier")) as $verifier
    | ($codegen != null and $styling != null and $verifier != null and $codegen < $styling and $styling < $verifier)
  ' "$spec" >/dev/null; then
    echo "${spec}: execution_order must place mds-styling-system after mds-codegen and before mds-verifier."
    status=1
  fi
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

# Files opting into the style-system component policy:
#   // ci: style-system-component
mapfile -t style_components < <(
  rg --no-heading --line-number -g '*.rs' \
    '^\s*//\s*ci:\s*style-system-component\b' \
    crates/http/src/views \
    | cut -d: -f1 \
    | sort -u
)

for file in "${style_components[@]}"; do
  if rg --no-heading --line-number 'inline_css!' "$file" >/dev/null; then
    echo "${file}: style-system components should avoid inline_css! and consume global package classes."
    status=1
  fi

  if ! rg --no-heading --line-number 'class="[^"]*ui-' "$file" >/dev/null; then
    echo "${file}: expected usage of reusable ui-* package classes."
    status=1
  fi
done

exit "$status"
