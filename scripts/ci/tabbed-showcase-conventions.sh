#!/usr/bin/env bash
set -euo pipefail

render_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/render.rs"
styles_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/styles.rs"
styles_dir="crates/http/src/views/partials/demo/layout/tabbed_showcase/styles"
behavior_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/behavior.rs"
status=0

require_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if ! rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

forbid_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if rg -q -- "$pattern" "$file"; then
    echo "tabbed-showcase-conventions: $message"
    status=1
  fi
}

require_pattern '^mod base;' "expected styles submodule declarations" "$styles_file"
require_pattern '\(BaseStyles\.render\(\)\)' "expected BaseStyles.render() composition" "$styles_file"
if ! rg -q -- 'inline_css!' "$styles_dir"; then
  echo "tabbed-showcase-conventions: expected inline_css! blocks in styles submodules"
  status=1
fi
require_pattern 'inline_js!' "expected inline_js! block" "$behavior_file"
require_pattern '\(Styles\.render\(\)\)' "expected Styles.render() splice in render template" "$render_file"
require_pattern '\(Behavior\.render\(\)\)' "expected Behavior.render() splice in render template" "$render_file"
require_pattern '\bme\(' "expected Surreal me() usage in inline_js" "$behavior_file"
require_pattern '\bany\(' "expected Surreal any() usage in inline_js" "$behavior_file"

forbid_pattern 'fn tabs_script\(' "legacy tabs_script helper should not exist" "$behavior_file"
forbid_pattern 'PreEscaped\(tabs_script\)' "render-local string script injection is not allowed" "$render_file"
forbid_pattern 'maud_extensions::css\b' "use inline_css!/inline_js! macros for this component" "$render_file"

exit "$status"
