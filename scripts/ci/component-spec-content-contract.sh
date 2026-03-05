#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for component spec content checks."
  exit 1
fi

status=0
found=0

while IFS= read -r spec; do
  found=1

  if ! jq -e '.content | type == "object"' "$spec" >/dev/null; then
    echo "${spec}: missing top-level content object."
    status=1
  fi

  if ! jq -e '.content.source == "cms"' "$spec" >/dev/null; then
    echo "${spec}: content.source must be \"cms\"."
    status=1
  fi

  if ! jq -e '.content.root_type | type == "string" and test("^[A-Z][A-Za-z0-9]*Content$")' "$spec" >/dev/null; then
    echo "${spec}: content.root_type must be a typed model ending with Content."
    status=1
  fi

  fixture_path="$(jq -r '.content.fixture_path // ""' "$spec")"
  if [[ -z "$fixture_path" ]]; then
    echo "${spec}: content.fixture_path is required."
    status=1
  elif [[ ! -f "$fixture_path" ]]; then
    echo "${spec}: content.fixture_path does not exist: ${fixture_path}"
    status=1
  fi

  if ! jq -e '(.pipeline.required_agents // []) | index("mds-cms-content-modeler") != null' "$spec" >/dev/null; then
    echo "${spec}: pipeline.required_agents must include mds-cms-content-modeler."
    status=1
  fi

  if ! jq -e '(.pipeline.parallel_groups // []) | map((index("mds-cms-content-modeler") != null) and (index("mds-ui-decomposer") != null)) | any' "$spec" >/dev/null; then
    echo "${spec}: pipeline.parallel_groups must run mds-cms-content-modeler with mds-ui-decomposer."
    status=1
  fi
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

if [[ "$found" -eq 0 ]]; then
  echo "component-spec-content-contract: no component_spec files found; skipping."
fi

exit "$status"
