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

tab_set_content_module="crates/http/src/views/partials/components/composed/tab_set/content.rs"
tab_set_dir="crates/http/src/views/partials/components/composed/tab_set"
tab_component="crates/http/src/views/partials/components/primitives/tab.rs"
fixtures=(
  "tests/fixtures/cms/tab_set_showcase.json"
  "crates/http/src/views/partials/demo/layout/content/tab_set_showcase.json"
)

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

if rg --no-heading --line-number 'glyph_from_asset_ref' "$tab_set_dir" >/dev/null; then
  echo "${tab_set_dir}: legacy asset_ref->glyph mapping is disallowed."
  status=1
fi

if rg --no-heading --line-number 'maud_iconoir::regular' "$tab_set_dir" >/dev/null; then
  echo "${tab_set_dir}: tab_set should not bind icon tokens through maud_iconoir const maps."
  status=1
fi

if ! rg --no-heading --line-number 'use (super::Icon|crate::views::partials::components::primitives::Icon);' "$tab_component" >/dev/null; then
  echo "${tab_component}: tab component must compose primitive Icon."
  status=1
fi

if ! rg --no-heading --line-number 'pub\s+icon:\s+Option<Icon>' "$tab_component" >/dev/null; then
  echo "${tab_component}: tab component must carry an optional Icon field."
  status=1
fi

if rg --no-heading --line-number 'IconKey' "$tab_component" >/dev/null; then
  echo "${tab_component}: IconKey enum mapping is disallowed; use the shared Icon primitive."
  status=1
fi

if rg --no-heading --line-number 'maud_iconoir::regular' "$tab_component" >/dev/null; then
  echo "${tab_component}: tab component should render icons through the shared Icon primitive."
  status=1
fi

for fixture in "${fixtures[@]}"; do
  if [[ ! -f "$fixture" ]]; then
    echo "${fixture}: missing tab_set icon fixture."
    status=1
    continue
  fi

  if ! jq -e '(.tabs | length) > 0' "$fixture" >/dev/null; then
    echo "${fixture}: expected at least one tab entry."
    status=1
  fi

  if ! jq -e '.tabs | all(.icon == null or ((.icon | type == "object") and ((.icon | keys | sort) == ["key"])))' "$fixture" >/dev/null; then
    echo "${fixture}: tab icons must be null or strict token-only objects of the form {\"key\": \"...\"}."
    status=1
  fi

  if ! jq -e '.tabs | all(.icon == null or (.icon.key | type == "string" and test("^[a-z0-9-]+$")))' "$fixture" >/dev/null; then
    echo "${fixture}: icon.key values must be lowercase dash tokens."
    status=1
  fi
done

exit "$status"
