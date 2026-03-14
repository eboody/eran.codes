#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
snapshot_script="$script_dir/visual-snapshot.sh"

url="${VISUAL_LAB_URL:-http://127.0.0.1:3000/lab}"
current_dir="${VISUAL_CURRENT_DIR:-artifacts/visual/current/lab}"
baseline_dir="${VISUAL_BASELINE_DIR:-artifacts/visual/baseline/lab}"
wait_ms="${VISUAL_WAIT_MS:-1800}"
click_wait_ms="${VISUAL_CLICK_WAIT_MS:-2200}"
viewport_width="${VISUAL_VIEWPORT_WIDTH:-1440}"
viewport_height="${VISUAL_VIEWPORT_HEIGHT:-1400}"
prune_current="${VISUAL_PRUNE_CURRENT:-1}"
capture_full_page="${VISUAL_CAPTURE_FULL_PAGE:-0}"
capture_active_operations="${VISUAL_CAPTURE_ACTIVE_OPERATIONS:-0}"
compare_operations="${VISUAL_COMPARE_OPERATIONS:-0}"

operations_skip_baseline="1"
if [[ "$compare_operations" == "1" ]]; then
  operations_skip_baseline="0"
fi

run_lab_snapshot() {
  local name="$1"
  local element_selector="$2"
  local click_selector="${3:-}"
  local remove_data_init_selector="${4:-}"
  local skip_baseline="${5:-0}"

  VISUAL_URL="$url" \
  VISUAL_CURRENT_DIR="$current_dir" \
  VISUAL_BASELINE_DIR="$baseline_dir" \
  VISUAL_OUTPUT="$current_dir/$name.png" \
  VISUAL_BASELINE="$baseline_dir/$name.png" \
  VISUAL_WAIT_MS="$wait_ms" \
  VISUAL_CLICK_WAIT_MS="$click_wait_ms" \
  VISUAL_VIEWPORT_WIDTH="$viewport_width" \
  VISUAL_VIEWPORT_HEIGHT="$viewport_height" \
  VISUAL_CAPTURE_SHOWCASE_STATES=0 \
  VISUAL_PRUNE_CURRENT=0 \
  VISUAL_ELEMENT_SELECTOR="$element_selector" \
  VISUAL_CLICK_SELECTOR="$click_selector" \
  VISUAL_REMOVE_DATA_INIT_SELECTOR="$remove_data_init_selector" \
  VISUAL_SKIP_BASELINE="$skip_baseline" \
  VISUAL_UPDATE_BASELINE="${VISUAL_UPDATE_BASELINE:-0}" \
  "$snapshot_script"
}

if [[ "$prune_current" == "1" ]]; then
  mkdir -p "$current_dir"
  find "$current_dir" -maxdepth 1 -type f -name '*.png' -delete
fi

run_lab_snapshot "chat-surface" "#chat-demo"
run_lab_snapshot "operations-surface" "#operations-surface" "" "[data-op-filter]" "$operations_skip_baseline"

if [[ "$capture_active_operations" == "1" ]]; then
  run_lab_snapshot "operations-surface-active" "#operations-surface" "[data-burst-run]"
fi

if [[ "$capture_full_page" == "1" ]]; then
  VISUAL_URL="$url" \
  VISUAL_CURRENT_DIR="$current_dir" \
  VISUAL_BASELINE_DIR="$baseline_dir" \
  VISUAL_OUTPUT="$current_dir/full-page.png" \
  VISUAL_BASELINE="$baseline_dir/full-page.png" \
  VISUAL_WAIT_MS="$wait_ms" \
  VISUAL_CLICK_WAIT_MS="$click_wait_ms" \
  VISUAL_VIEWPORT_WIDTH="$viewport_width" \
  VISUAL_VIEWPORT_HEIGHT="$viewport_height" \
  VISUAL_CAPTURE_SHOWCASE_STATES=0 \
  VISUAL_PRUNE_CURRENT=0 \
  VISUAL_UPDATE_BASELINE="${VISUAL_UPDATE_BASELINE:-0}" \
  "$snapshot_script"
fi
