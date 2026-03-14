#!/usr/bin/env bash
set -euo pipefail

url="${VISUAL_URL:-http://127.0.0.1:3000/}"
current_dir="${VISUAL_CURRENT_DIR:-artifacts/visual/current}"
baseline_dir="${VISUAL_BASELINE_DIR:-artifacts/visual/baseline}"
output="${VISUAL_OUTPUT:-$current_dir/home.png}"
baseline="${VISUAL_BASELINE:-$baseline_dir/home.png}"
wait_ms="${VISUAL_WAIT_MS:-1200}"
viewport_width="${VISUAL_VIEWPORT_WIDTH:-1440}"
viewport_height="${VISUAL_VIEWPORT_HEIGHT:-1024}"
click_wait_ms="${VISUAL_CLICK_WAIT_MS:-800}"
click_selector="${VISUAL_CLICK_SELECTOR:-}"
element_selector="${VISUAL_ELEMENT_SELECTOR:-}"
remove_data_init_selector="${VISUAL_REMOVE_DATA_INIT_SELECTOR:-}"
skip_baseline="${VISUAL_SKIP_BASELINE:-0}"
capture_showcase_states="${VISUAL_CAPTURE_SHOWCASE_STATES:-1}"
prune_current="${VISUAL_PRUNE_CURRENT:-1}"
prune_pattern="${VISUAL_PRUNE_PATTERN:-*.png}"
showcase_tab_0="$current_dir/showcase_tab_0.png"
showcase_tab_1="$current_dir/showcase_tab_1.png"
showcase_tab_2="$current_dir/showcase_tab_2.png"
showcase_tab_3="$current_dir/showcase_tab_3.png"
baseline_showcase_tab_0="$baseline_dir/showcase_tab_0.png"
baseline_showcase_tab_1="$baseline_dir/showcase_tab_1.png"
baseline_showcase_tab_2="$baseline_dir/showcase_tab_2.png"
baseline_showcase_tab_3="$baseline_dir/showcase_tab_3.png"

run_snapshot() {
  local snapshot_output="$1"
  local snapshot_baseline="$2"
  local click_selector="${3:-}"
  local element_selector="${4:-}"
  local args=(
    run -p utils --bin visual_snapshot --
    --url "$url"
    --output "$snapshot_output"
    --wait-ms "$wait_ms"
    --viewport-width "$viewport_width"
    --viewport-height "$viewport_height"
  )

  if [[ "$skip_baseline" != "1" ]]; then
    args+=(--baseline "$snapshot_baseline")
  fi

  if [[ "${VISUAL_UPDATE_BASELINE:-0}" == "1" && "$skip_baseline" != "1" ]]; then
    args+=(--update-baseline)
  fi

  if [[ -n "$click_selector" ]]; then
    args+=(--click-selector "$click_selector" --click-wait-ms "$click_wait_ms")
  fi

  if [[ -n "$element_selector" ]]; then
    args+=(--element-selector "$element_selector")
  fi

  if [[ -n "$remove_data_init_selector" ]]; then
    args+=(--remove-data-init-selector "$remove_data_init_selector")
  fi

  RUSTC_WRAPPER= cargo "${args[@]}"
}

remove_if_exists() {
  local path="$1"
  if [[ -f "$path" ]]; then
    rm -f "$path"
  fi
}

if [[ "$prune_current" == "1" ]]; then
  mkdir -p "$current_dir"
  find "$current_dir" -maxdepth 1 -type f -name "$prune_pattern" -delete
fi

run_snapshot "$output" "$baseline" "$click_selector" "$element_selector"

if [[ "$capture_showcase_states" == "1" ]]; then
  if curl -fsS "$url" | rg -q 'data-showcase-tabs'; then
    run_snapshot "$showcase_tab_0" "$baseline_showcase_tab_0" '[data-showcase-tabs] [data-tab-index="0"]'
    run_snapshot "$showcase_tab_1" "$baseline_showcase_tab_1" '[data-showcase-tabs] [data-tab-index="1"]'
    run_snapshot "$showcase_tab_2" "$baseline_showcase_tab_2" '[data-showcase-tabs] [data-tab-index="2"]'
    run_snapshot "$showcase_tab_3" "$baseline_showcase_tab_3" '[data-showcase-tabs] [data-tab-index="3"]'
  else
    remove_if_exists "$showcase_tab_0"
    remove_if_exists "$showcase_tab_1"
    remove_if_exists "$showcase_tab_2"
    remove_if_exists "$showcase_tab_3"
    echo "visual-snapshot: no showcase tabs detected at $url; skipping tab-state captures"
  fi
fi
