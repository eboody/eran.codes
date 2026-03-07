#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for log-componentization checks."
  exit 1
fi

status=0

# Legacy type names should not remain after rename.
if rg --no-heading --line-number '\b(LiveLog|NetworkLog|TraceLog)\b' \
  crates/http/src/views/partials \
  crates/http/src/trace_log.rs \
  >/dev/null; then
  echo "log-componentization-contract: found legacy log type names (LiveLog/NetworkLog/TraceLog)."
  rg --no-heading --line-number '\b(LiveLog|NetworkLog|TraceLog)\b' \
    crates/http/src/views/partials \
    crates/http/src/trace_log.rs || true
  status=1
fi

# Trace log store should patch using renamed log composite.
if ! rg --no-heading --line-number 'TransportLogSet::builder\(\)' crates/http/src/trace_log.rs >/dev/null; then
  echo "crates/http/src/trace_log.rs: expected TransportLogSet::builder() usage."
  status=1
fi

# Stable patch targets must remain present.
if ! rg --no-heading --line-number 'network-log-target' crates/http/src/views/partials/demo/log/transport_log_set.rs >/dev/null; then
  echo "transport_log_set.rs: expected stable patch target network-log-target."
  status=1
fi

if ! rg --no-heading --line-number 'FlowTimeline::builder\(\)' crates/http/src/views/partials/demo/log/transport_log_set.rs >/dev/null; then
  echo "transport_log_set.rs: expected FlowTimeline::builder() usage for request timeline composition."
  status=1
fi

if ! rg --no-heading --line-number 'request_flows\(' crates/http/src/views/partials/demo/log/transport_log_set.rs >/dev/null; then
  echo "transport_log_set.rs: expected vm::request_flows(...) mapping usage."
  status=1
fi

count_chat_request_id=$(rg --no-heading --line-number 'LogFieldKey::RequestId' crates/http/src/handlers/demo/chat.rs | wc -l | tr -d ' ')
if [[ "${count_chat_request_id}" -lt "2" ]]; then
  echo "crates/http/src/handlers/demo/chat.rs: expected RequestId field to be recorded on chat incoming + broadcast events."
  status=1
fi

# Reusable log components should consume global CSS, not inline css! blocks.
if rg --no-heading --line-number 'css!' \
  crates/http/src/views/partials/components/logs \
  crates/http/src/views/partials/demo/log \
  >/dev/null; then
  echo "log-componentization-contract: inline css! found in log components; use app.css ui-log-* packages."
  rg --no-heading --line-number 'css!' \
    crates/http/src/views/partials/components/logs \
    crates/http/src/views/partials/demo/log || true
  status=1
fi

# Helper de-duplication: these should live in vm/* exactly once.
count_group_by_request=$(rg --no-heading --line-number 'fn\s+group_by_request\b' crates/http/src/views/partials/demo/log -g '*.rs' | wc -l | tr -d ' ')
if [[ "${count_group_by_request}" != "1" ]]; then
  echo "log-componentization-contract: expected exactly one group_by_request helper in demo/log vm layer, found ${count_group_by_request}."
  status=1
fi

count_field_text=$(rg --no-heading --line-number 'fn\s+field_text\b' crates/http/src/views/partials/demo/log -g '*.rs' | wc -l | tr -d ' ')
if [[ "${count_field_text}" != "1" ]]; then
  echo "log-componentization-contract: expected exactly one field_text helper in demo/log vm layer, found ${count_field_text}."
  status=1
fi

for vm_file in \
  "crates/http/src/views/partials/demo/log/vm/network_tables.rs" \
  "crates/http/src/views/partials/demo/log/vm/chat_flow_rows.rs" \
  "crates/http/src/views/partials/demo/log/vm/request_flow/mod.rs"
do
  if ! rg --no-heading --line-number '#\[cfg\(test\)\]' "${vm_file}" >/dev/null; then
    echo "${vm_file}: expected vm-layer regression tests."
    status=1
  fi
done

APP_CSS="crates/http/static/app.css"
for cls in \
  ".ui-log-surface" \
  ".ui-log-panels" \
  ".ui-log-panel" \
  ".ui-log-scroll" \
  ".ui-log-flow-shell" \
  ".ui-log-flow-list" \
  ".ui-log-flow-item" \
  ".ui-log-flow-details" \
  ".ui-log-flow-detail" \
  ".ui-log-flow-event" \
  ".ui-log-groups" \
  ".ui-log-group" \
  ".ui-log-group-header" \
  ".ui-log-entry" \
  ".ui-log-table" \
  ".ui-log-empty"
do
  if ! rg -q "^\\s*${cls}\\b" "${APP_CSS}"; then
    echo "${APP_CSS}: missing required log package class ${cls}."
    status=1
  fi
done

exit "${status}"
