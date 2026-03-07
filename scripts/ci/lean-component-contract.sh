#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for lean-component checks."
  exit 1
fi

status=0

request_flow_dir="crates/http/src/views/partials/demo/log/vm/request_flow"
legacy_request_flow_file="crates/http/src/views/partials/demo/log/vm/request_flow.rs"
required_request_flow_files=(mod.rs kind.rs event_builder.rs pills.rs tests.rs)
request_flow_line_cap=220

if [[ -f "${legacy_request_flow_file}" ]]; then
  echo "${legacy_request_flow_file}: monolithic request_flow file is disallowed; use request_flow/ module family."
  status=1
fi

for file in "${required_request_flow_files[@]}"; do
  if [[ ! -f "${request_flow_dir}/${file}" ]]; then
    echo "${request_flow_dir}/${file}: missing required request_flow module file."
    status=1
  fi
done

for file in "${request_flow_dir}"/*.rs; do
  [[ -e "${file}" ]] || continue
  line_count="$(wc -l < "${file}" | tr -d ' ')"
  if ((line_count > request_flow_line_cap)); then
    echo "${file}: ${line_count} lines exceeds lean cap (${request_flow_line_cap})."
    status=1
  fi
done

surface_file="crates/http/src/views/partials/components/logs/primitives/surface.rs"
if ! rg --no-heading --line-number 'pub\s+children:\s+Vec<logs::primitives::Panel>' "${surface_file}" >/dev/null; then
  echo "${surface_file}: Surface.children must be typed as Vec<logs::primitives::Panel>."
  status=1
fi

mapfile -t markup_field_files < <(
  rg --no-heading --line-number \
    --glob 'crates/http/src/views/partials/components/composed/**/*.rs' \
    'pub\s+\w+\s*:\s*(?:Option<\s*)?maud::Markup(?:\s*>)?' \
    | cut -d: -f1 \
    | sort -u
)

for file in "${markup_field_files[@]}"; do
  if ! rg --no-heading --line-number '^\s*//\s*ci:\s*markup-slot-exempt\b' "${file}" >/dev/null; then
    echo "${file}: maud::Markup component props require explicit // ci: markup-slot-exempt <reason>."
    status=1
  fi
done

exit "${status}"
