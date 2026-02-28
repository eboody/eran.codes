#!/usr/bin/env bash
set -euo pipefail

file="crates/http/src/views/partials/demo/layout/tabbed_showcase.rs"
status=0

require_pattern() {
  local pattern="$1"
  local message="$2"
  if ! rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

forbid_pattern() {
  local pattern="$1"
  local message="$2"
  if rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

require_pattern 'inline_css!' "expected inline_css! block"
require_pattern 'inline_js!' "expected inline_js! block"
require_pattern '\(css\(\)\)' "expected (css()) splice in render template"
require_pattern '\(js\(\)\)' "expected (js()) splice in render template"
require_pattern '\bme\(' "expected Surreal me() usage in inline_js"
require_pattern '\bany\(' "expected Surreal any() usage in inline_js"

forbid_pattern 'fn tabs_script\(' "legacy tabs_script helper should not exist"
forbid_pattern 'PreEscaped\(tabs_script\)' "render-local string script injection is not allowed"
forbid_pattern 'maud_extensions::css\b' "use inline_css!/inline_js! macros for this component"

exit "$status"
