#!/usr/bin/env bash
set -euo pipefail

current_url="${VISUAL_CURRENT_URL:-http://127.0.0.1:3000/}"
reference_dir="${VISUAL_REFERENCE_DIR:-artifacts/visual/reference}"
wait_ms="${VISUAL_WAIT_MS:-1500}"
click_wait_ms="${VISUAL_CLICK_WAIT_MS:-700}"
viewport_width="${VISUAL_VIEWPORT_WIDTH:-1440}"
viewport_height="${VISUAL_VIEWPORT_HEIGHT:-1024}"

netbird_home_png="$reference_dir/netbird_home.png"
netbird_app_png="$reference_dir/netbird_app_home.png"
current_home_png="$reference_dir/current_home.png"
current_tab_0_png="$reference_dir/current_showcase_tab_0.png"
current_tab_1_png="$reference_dir/current_showcase_tab_1.png"
current_tab_2_png="$reference_dir/current_showcase_tab_2.png"
current_tab_3_png="$reference_dir/current_showcase_tab_3.png"

run_snapshot() {
  local url="$1"
  local output="$2"
  local click_selector="${3:-}"
  local args=(
    run -p utils --bin visual_snapshot --
    --url "$url"
    --output "$output"
    --wait-ms "$wait_ms"
    --viewport-width "$viewport_width"
    --viewport-height "$viewport_height"
  )

  if [[ -n "$click_selector" ]]; then
    args+=(--click-selector "$click_selector" --click-wait-ms "$click_wait_ms")
  fi

  RUSTC_WRAPPER= cargo "${args[@]}"
}

mkdir -p "$reference_dir"
rm -f \
  "$netbird_home_png" \
  "$netbird_app_png" \
  "$current_home_png" \
  "$current_tab_0_png" \
  "$current_tab_1_png" \
  "$current_tab_2_png" \
  "$current_tab_3_png"

run_snapshot "https://netbird.io/" "$netbird_home_png"
run_snapshot "https://app.netbird.io/" "$netbird_app_png"
run_snapshot "$current_url" "$current_home_png"

if curl -fsS "$current_url" | rg -q 'data-showcase-tabs'; then
  run_snapshot "$current_url" "$current_tab_0_png" '[data-showcase-tabs] [data-tab-index="0"]'
  run_snapshot "$current_url" "$current_tab_1_png" '[data-showcase-tabs] [data-tab-index="1"]'
  run_snapshot "$current_url" "$current_tab_2_png" '[data-showcase-tabs] [data-tab-index="2"]'
  run_snapshot "$current_url" "$current_tab_3_png" '[data-showcase-tabs] [data-tab-index="3"]'
else
  echo "visual-reference-capture: no showcase tabs detected at $current_url; skipping tab states"
fi

echo "visual-reference-capture: wrote fresh reference captures to $reference_dir"
