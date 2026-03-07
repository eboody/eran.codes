#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for tab/icon reference checks."
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for tab/icon reference checks."
  exit 1
fi

status=0

tab_set_component="crates/http/src/views/partials/components/composed/tab_set/mod.rs"
tab_set_content_module="crates/http/src/views/partials/components/composed/tab_set/content.rs"
tab_set_showcase="crates/http/src/views/partials/demo/layout/tab_set_showcase.rs"
tab_component="crates/http/src/views/partials/components/primitives/tab.rs"
fixture="tests/fixtures/cms/tab_set_showcase.json"
runtime_fixture="crates/http/src/views/partials/demo/layout/content/tab_set_showcase.json"

if ! rg --no-heading --line-number '^//\s*ci:\s*descriptive-module-import\s+crate::views::partials::components::tab_set$' "$tab_set_component" >/dev/null; then
  echo "${tab_set_component}: expected descriptive-module-import marker for tab_set namespace."
  status=1
fi

for module in tab pane content; do
  if ! rg --no-heading --line-number "pub\(crate\) mod ${module}" "$tab_set_component" >/dev/null; then
    echo "${tab_set_component}: expected module-scoped surface 'pub(crate) mod ${module}'."
    status=1
  fi
done

if ! rg --no-heading --line-number 'use crate::views::partials::components::primitives::Icon;' "$tab_set_content_module" >/dev/null; then
  echo "${tab_set_content_module}: content module must use shared primitive Icon contract."
  status=1
fi

if ! rg --no-heading --line-number 'pub\s+icon:\s+Option<Icon>' "$tab_set_content_module" >/dev/null; then
  echo "${tab_set_content_module}: tab content must expose icon as Option<Icon>."
  status=1
fi

if rg --no-heading --line-number 'IconKey' "$tab_set_content_module" >/dev/null; then
  echo "${tab_set_content_module}: IconKey enum mapping is disallowed; use CMS icon tokens through primitive Icon."
  status=1
fi

if rg --no-heading --line-number 'glyph_from_asset_ref' crates/http/src/views/partials/components/composed/tab_set >/dev/null; then
  echo "crates/http/src/views/partials/components/composed/tab_set: legacy asset_ref->glyph mapping is disallowed."
  status=1
fi

if rg --no-heading --line-number 'maud_iconoir::regular' crates/http/src/views/partials/components/composed/tab_set >/dev/null; then
  echo "crates/http/src/views/partials/components/composed/tab_set: tab_set should not bind icon tokens through maud_iconoir const maps."
  status=1
fi

if rg --no-heading --line-number 'use\s+crate::views::partials::components::tab_set::' "$tab_set_showcase" >/dev/null; then
  echo "${tab_set_showcase}: avoid leaf imports; import tab_set namespace and qualify members."
  status=1
fi

if ! rg --no-heading --line-number 'use crate::views::partials::components::\{tab_set, Tab, TabInteraction\};' "$tab_set_showcase" >/dev/null; then
  echo "${tab_set_showcase}: expected namespace-style tab_set import with shared Tab primitive."
  status=1
fi

if ! rg --no-heading --line-number 'TabInteraction::DatastarLocal' "$tab_set_showcase" >/dev/null; then
  echo "${tab_set_showcase}: tabs must use TabInteraction::DatastarLocal for ui-local Datastar switching."
  status=1
fi

if ! rg --no-heading --line-number 'tab_set::pane::Item::from_content' "$tab_set_showcase" >/dev/null; then
  echo "${tab_set_showcase}: panes must be composed from shared Tab + content via module-scoped pane::Item::from_content."
  status=1
fi

if ! rg --no-heading --line-number 'icon:\s+tab\.icon\.clone\(\)' "$tab_set_showcase" >/dev/null; then
  echo "${tab_set_showcase}: showcase tabs should pass CMS icon tokens via shared Icon primitive."
  status=1
fi

if ! rg --no-heading --line-number 'use (super::Icon|crate::views::partials::components::primitives::Icon);' "$tab_component" >/dev/null; then
  echo "${tab_component}: tab component must compose primitive Icon."
  status=1
fi

if ! rg --no-heading --line-number 'render_content\(&icon, &self\.text\)' "$tab_component" >/dev/null; then
  echo "${tab_component}: tab component should render icon+label through shared render_content helper."
  status=1
fi

if [[ ! -f "$fixture" ]]; then
  echo "${fixture}: missing CMS fixture for tab_set icon contract checks."
  status=1
else
  if ! jq -e '(.tabs | length) > 0' "$fixture" >/dev/null; then
    echo "${fixture}: expected at least one tab entry."
    status=1
  fi

  if ! jq -e '.tabs | all(.icon != null and (.icon | type == "object") and ((.icon | keys | sort) == ["key"]))' "$fixture" >/dev/null; then
    echo "${fixture}: each tab icon must be strict token-only object {\"key\": \"...\"}."
    status=1
  fi

  if ! jq -e '.tabs | all(.icon.key | type == "string" and test("^[a-z0-9-]+$"))' "$fixture" >/dev/null; then
    echo "${fixture}: icon.key values must be lowercase dash tokens."
    status=1
  fi
fi

if [[ ! -f "$runtime_fixture" ]]; then
  echo "${runtime_fixture}: missing runtime CMS fixture for tab_set showcase."
  status=1
elif ! cmp -s "$fixture" "$runtime_fixture"; then
  echo "${runtime_fixture}: runtime fixture must stay in sync with ${fixture}."
  status=1
fi

for spec in tests/fixtures/tab_set.component_spec.json generated/tab_set/resolved.component_spec.json; do
  if [[ ! -f "$spec" ]]; then
    echo "${spec}: missing tab_set spec fixture for tab/icon contract checks."
    status=1
    continue
  fi

  if ! jq -e '.meta.component_id == "tab_set"' "$spec" >/dev/null; then
    echo "${spec}: meta.component_id must be tab_set."
    status=1
  fi

  if ! jq -e '.content.root_type == "TabSetContent"' "$spec" >/dev/null; then
    echo "${spec}: content.root_type must be TabSetContent."
    status=1
  fi

  if ! jq -e '.content.notes | test("icon token"; "i")' "$spec" >/dev/null; then
    echo "${spec}: content.notes should document token-based icon contract."
    status=1
  fi

  if ! jq -e '(.design.render_contract.primitive_reuse // [] | index("tab")) != null' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.primitive_reuse must include \"tab\"."
    status=1
  fi

  if ! jq -e '(.design.render_contract.primitive_reuse // [] | index("icon")) != null' "$spec" >/dev/null; then
    echo "${spec}: design.render_contract.primitive_reuse must include \"icon\"."
    status=1
  fi

  if ! jq -e '.design.render_contract.interaction_mode == "ui_local_datastar"' "$spec" >/dev/null; then
    echo "${spec}: tab_set interaction_mode must be ui_local_datastar."
    status=1
  fi
done

exit "$status"
