#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for log-componentization checks."
  exit 1
fi

status=0
trace_log_root="crates/http/src/trace_log"

count_matches() {
  local pattern="$1"
  shift
  (rg --no-heading --line-number --glob '*.rs' -- "$pattern" "$@" || true) \
    | wc -l \
    | tr -d ' '
}

# Legacy type names should not remain after rename.
if rg --no-heading --line-number '\b(LiveLog|NetworkLog|TraceLog)\b' \
  crates/http/src/views/partials \
  "${trace_log_root}" \
  >/dev/null; then
  echo "log-componentization-contract: found legacy log type names (LiveLog/NetworkLog/TraceLog)."
  rg --no-heading --line-number '\b(LiveLog|NetworkLog|TraceLog)\b' \
    crates/http/src/views/partials \
    "${trace_log_root}" || true
  status=1
fi

# Trace log store should patch using renamed log composite.
if ! rg --no-heading --line-number 'TransportLogSet::builder\(\)' "${trace_log_root}/store.rs" >/dev/null; then
  echo "${trace_log_root}/store.rs: expected TransportLogSet::builder() usage."
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

count_chat_request_id=$(count_matches 'LogFieldKey::RequestId' crates/http/src/handlers/demo/chat/post_flow.rs)
if [[ "${count_chat_request_id}" -lt "2" ]]; then
  echo "crates/http/src/handlers/demo/chat/post_flow.rs: expected RequestId field to be recorded on chat incoming + broadcast events."
  status=1
fi

if ! rg --no-heading --line-number 'upsert_context_field\(fields,\s*LogFieldKey::RequestId' "${trace_log_root}/layer.rs" >/dev/null; then
  echo "${trace_log_root}/layer.rs: expected append_context_fields to inject request ids into traced events."
  status=1
fi

if ! rg --no-heading --line-number 'append_context_fields_does_not_duplicate_existing_request_id' "${trace_log_root}/layer.rs" >/dev/null; then
  echo "${trace_log_root}/layer.rs: expected regression test for pre-existing request ids in append_context_fields."
  status=1
fi

# Log components may use scoped inline_css!, but raw css! blocks are disallowed.
if rg --no-heading --line-number '\bcss!\(' \
  crates/http/src/views/partials/components/logs \
  crates/http/src/views/partials/demo/log \
  >/dev/null; then
  echo "log-componentization-contract: raw css! blocks found in log components; prefer scoped inline_css! surfaces."
  rg --no-heading --line-number '\bcss!\(' \
    crates/http/src/views/partials/components/logs \
    crates/http/src/views/partials/demo/log || true
  status=1
fi

# Helper de-duplication: these should live in vm/* exactly once.
count_group_by_request=$(count_matches 'fn\s+group_by_request\b' crates/http/src/views/partials/demo/log)
if [[ "${count_group_by_request}" != "1" ]]; then
  echo "log-componentization-contract: expected exactly one group_by_request helper in demo/log vm layer, found ${count_group_by_request}."
  status=1
fi

count_field_text=$(count_matches 'fn\s+field_text\b' crates/http/src/views/partials/demo/log)
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

exit "${status}"
