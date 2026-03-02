#!/usr/bin/env bash
set -euo pipefail

markup_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/mod.rs"
styles_file="crates/http/src/views/partials/demo/layout/tabbed_showcase/styles.rs"
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

require_pattern 'inline_css!' "expected inline_css! block" "$styles_file"
require_pattern 'inline_js!' "expected inline_js! block" "$behavior_file"
require_pattern '\(Styles\.render\(\)\)' "expected Styles.render() splice in render template" "$markup_file"
require_pattern '\(Behavior\.render\(\)\)' "expected Behavior.render() splice in render template" "$markup_file"
require_pattern '\bme\(' "expected Surreal me() usage in inline_js" "$behavior_file"
require_pattern '\bany\(' "expected Surreal any() usage in inline_js" "$behavior_file"

forbid_pattern 'fn tabs_script\(' "legacy tabs_script helper should not exist" "$behavior_file"
forbid_pattern 'PreEscaped\(tabs_script\)' "render-local string script injection is not allowed" "$markup_file"
forbid_pattern 'maud_extensions::css\b' "use inline_css!/inline_js! macros for this component" "$markup_file"

exit "$status"
