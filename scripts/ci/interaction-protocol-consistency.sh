#!/usr/bin/env bash
set -euo pipefail

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for interaction protocol consistency checks."
  exit 1
fi

status=0

while IFS= read -r spec; do
  has_override="$(jq -r 'has("override")' "$spec")"

  if ! jq -e '(.state.fields // [] | length) > 0' "$spec" >/dev/null; then
    continue
  fi

  if ! jq -e '(.state.fields // [] | all(has("interaction_scope")))' "$spec" >/dev/null; then
    echo "${spec}: every state field must define interaction_scope."
    status=1
  fi

  if ! jq -e '(.state.fields // [] | all(has("authority_rationale") and (.authority_rationale | type == "string") and (.authority_rationale | length > 0)))' "$spec" >/dev/null; then
    echo "${spec}: every state field must define non-empty authority_rationale."
    status=1
  fi

  if ! jq -e '(.state.fields // [] | map(select(.interaction_scope == "app") | .authority == "app") | all)' "$spec" >/dev/null; then
    echo "${spec}: app interaction_scope fields must use authority = app."
    status=1
  fi

  if [[ "$has_override" != "true" ]]; then
    if ! jq -e '(.state.fields // [] | map(select(.interaction_scope == "presentation") | .authority == "ui") | all)' "$spec" >/dev/null; then
      echo "${spec}: presentation interaction_scope fields must default to authority = ui unless override is present."
      status=1
    fi
  fi

  app_field_ids="$(jq -r '.state.fields[]? | select(.interaction_scope == "app") | .id' "$spec" | sort -u)"
  if [[ -n "$app_field_ids" ]]; then
    sse_field_ids="$(jq -r '.events.app_mappings.sse_events[]?.updates[]?.field_id // empty' "$spec" | sort -u)"
    while IFS= read -r field_id; do
      [[ -z "$field_id" ]] && continue
      if ! grep -Fxq "$field_id" <(printf '%s\n' "$sse_field_ids"); then
        echo "${spec}: app scope field '${field_id}' is missing from events.app_mappings.sse_events updates."
        status=1
      fi
    done < <(printf '%s\n' "$app_field_ids")
  fi

  # protocol_mode consistency
  while IFS= read -r handler; do
    handler_id="$(jq -r '.id' <<<"$handler")"
    mode="$(jq -r '.protocol_mode // ""' <<<"$handler")"
    [[ -z "$mode" ]] && continue

    if ! jq -e '.protocol_rationale | type == "string" and length > 0' <<<"$handler" >/dev/null; then
      echo "${spec}: handler '${handler_id}' defines protocol_mode but missing protocol_rationale."
      status=1
    fi

    has_effect="$(jq -e --arg id "$handler_id" '(.events.effects // [] | map(select(.handler_id == $id and .type == "invoke_backend")) | length) > 0' "$spec" >/dev/null && echo yes || echo no)"

    if [[ "$mode" == "command_sse" && "$has_effect" == "no" ]]; then
      class="$(jq -r '.class // ""' <<<"$handler")"
      if [[ "$class" == "ui" ]]; then
        echo "${spec}: handler '${handler_id}' uses command_sse but has no invoke_backend effect."
        status=1
      fi
    fi

    if [[ "$mode" == "ui_local" && "$has_effect" == "yes" && "$has_override" != "true" ]]; then
      echo "${spec}: handler '${handler_id}' is ui_local but invokes backend without override."
      status=1
    fi
  done < <(jq -c '.events.handlers[]?' "$spec")

  # tab heuristic: tab ui handlers should default ui_local unless override.
  if [[ "$has_override" != "true" ]]; then
    while IFS= read -r handler; do
      handler_id="$(jq -r '.id' <<<"$handler")"
      source_node_id="$(jq -r '.source_node_id // ""' <<<"$handler")"
      mode="$(jq -r '.protocol_mode // ""' <<<"$handler")"
      class="$(jq -r '.class // ""' <<<"$handler")"
      if [[ "$class" == "ui" && ( "$handler_id" == *tab* || "$source_node_id" == *tab* ) ]]; then
        if [[ "$mode" != "ui_local" ]]; then
          echo "${spec}: tab-like UI handler '${handler_id}' must use protocol_mode=ui_local unless override is present."
          status=1
        fi
      fi
    done < <(jq -c '.events.handlers[]?' "$spec")
  fi
done < <(find tests/fixtures generated -type f \( -name '*.component_spec.json' -o -name 'resolved.component_spec.json' \) 2>/dev/null | sort)

exit "$status"
