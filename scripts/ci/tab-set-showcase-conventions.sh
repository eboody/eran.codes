#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for tab_set showcase convention checks."
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for tab_set showcase convention checks."
  exit 1
fi

status=0
component_file="crates/http/src/views/partials/components/composed/tab_set/mod.rs"
pane_item_file="crates/http/src/views/partials/components/composed/tab_set/pane/item.rs"
showcase_file="crates/http/src/views/partials/demo/layout/tab_set_showcase.rs"

require_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if ! rg --no-heading --line-number -- "$pattern" "$file" >/dev/null; then
    echo "tab-set-showcase-conventions: $message"
    status=1
  fi
}

forbid_pattern() {
  local pattern="$1"
  local message="$2"
  local file="$3"
  if rg --no-heading --line-number -- "$pattern" "$file" >/dev/null; then
    echo "tab-set-showcase-conventions: $message"
    status=1
  fi
}

require_pattern '^//\s*ci:\s*descriptive-module-import\s+crate::views::partials::components::tab_set$' \
  "expected descriptive-module-import marker for tab_set namespace" \
  "$component_file"
require_pattern 'pub\(crate\)\s+struct\s+ContentProps<' \
  "expected ContentProps companion for content-driven composition" \
  "$component_file"
require_pattern 'pub\(crate\)\s+fn\s+from_content\(props:\s+ContentProps<' \
  "expected Component::from_content(...) helper" \
  "$component_file"
require_pattern 'component_signals\(&self\.signal_name,\s*&self\.active_tab_id\)' \
  "component render should emit signals from the configured signal_name" \
  "$component_file"
require_pattern 'tabs_from_content\(' \
  "tab_set should assemble tabs behind the component boundary" \
  "$component_file"
require_pattern 'panes_from_content\(' \
  "tab_set should assemble panes behind the component boundary" \
  "$component_file"
require_pattern 'show_expr\(&self\.signal_name,\s*&self\.tab_value\)' \
  "pane visibility should follow the configured tab signal" \
  "$pane_item_file"

require_pattern '^use crate::views::partials::components::tab_set;' \
  "showcase should import the tab_set namespace surface" \
  "$showcase_file"
require_pattern 'tab_set::Component::from_content\(' \
  "showcase must compose the live component through Component::from_content(...)" \
  "$showcase_file"
require_pattern 'tab_set::ContentProps::builder\(' \
  "showcase must use the tab_set namespace surface for content props" \
  "$showcase_file"
require_pattern '"u-surface-card tab-set-showcase"' \
  "showcase should keep the shared card shell plus tab-set root class" \
  "$showcase_file"
forbid_pattern 'TabInteraction::DatastarLocal' \
  "showcase should not wire Datastar tab interaction directly" \
  "$showcase_file"
forbid_pattern 'tab_set::pane::Item::from_content' \
  "showcase should not assemble panes directly" \
  "$showcase_file"
forbid_pattern 'THEME\.gray' \
  "showcase should not inject theme palette directly when the component can default it" \
  "$showcase_file"

for spec in tests/fixtures/tab_set.component_spec.json generated/tab_set/resolved.component_spec.json; do
  if [[ ! -f "$spec" ]]; then
    echo "tab-set-showcase-conventions: missing tab_set spec ${spec}"
    status=1
    continue
  fi

  if ! jq -e '.styling.mode == "hybrid"' "$spec" >/dev/null; then
    echo "tab-set-showcase-conventions: ${spec} must keep styling.mode = hybrid."
    status=1
  fi

  if ! jq -e '(.styling.global_packages // [] | index("u-surface-card")) != null' "$spec" >/dev/null; then
    echo "tab-set-showcase-conventions: ${spec} should document the shared card shell package."
    status=1
  fi

  if ! jq -e '(.styling.scoped_exceptions // [] | length) > 0' "$spec" >/dev/null; then
    echo "tab-set-showcase-conventions: ${spec} should document the scoped tab_set exceptions."
    status=1
  fi
done

exit "$status"
