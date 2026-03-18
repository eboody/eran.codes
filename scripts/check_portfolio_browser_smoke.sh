#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

base_url="${PORTFOLIO_SMOKE_BASE_URL:-http://127.0.0.1:3000}"
current_dir="${PORTFOLIO_SMOKE_CURRENT_DIR:-artifacts/visual/current/portfolio-smoke}"
baseline_dir="${PORTFOLIO_SMOKE_BASELINE_DIR:-artifacts/visual/baseline/portfolio-smoke}"
mode="${PORTFOLIO_SMOKE_MODE:-smoke}"
wait_ms="${PORTFOLIO_SMOKE_WAIT_MS:-1400}"
click_wait_ms="${PORTFOLIO_SMOKE_CLICK_WAIT_MS:-900}"
assert_timeout_ms="${PORTFOLIO_SMOKE_ASSERT_TIMEOUT_MS:-6000}"
use_baselines="${PORTFOLIO_SMOKE_USE_BASELINES:-0}"
update_baselines="${PORTFOLIO_SMOKE_UPDATE_BASELINE:-0}"
prune_current="${PORTFOLIO_SMOKE_PRUNE_CURRENT:-1}"
desktop_width="${PORTFOLIO_SMOKE_DESKTOP_WIDTH:-1920}"
desktop_height="${PORTFOLIO_SMOKE_DESKTOP_HEIGHT:-1080}"
mobile_width="${PORTFOLIO_SMOKE_MOBILE_WIDTH:-390}"
mobile_height="${PORTFOLIO_SMOKE_MOBILE_HEIGHT:-844}"

mkdir -p "$current_dir"
mkdir -p "$baseline_dir"

if [[ "$prune_current" == "1" ]]; then
  find "$current_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.html' \) -delete
fi

run_case() {
  local name="$1"
  local path="$2"
  local viewport_width="$3"
  local viewport_height="$4"
  local color_scheme="$5"
  local click_selector="${6:-}"
  shift 6

  local output="$current_dir/${name}.png"
  local html="$current_dir/${name}.html"
  local url="${base_url%/}${path}"
  local args=(
    run -p utils --features visual-snapshot --bin visual_snapshot --
    --url "$url"
    --output "$output"
    --dump-html "$html"
    --wait-ms "$wait_ms"
    --viewport-width "$viewport_width"
    --viewport-height "$viewport_height"
    --color-scheme "$color_scheme"
    --assert-timeout-ms "$assert_timeout_ms"
  )

  if [[ -n "$click_selector" ]]; then
    args+=(--click-selector "$click_selector" --click-wait-ms "$click_wait_ms")
  fi

  while [[ $# -gt 0 ]]; do
    args+=(--assert-selector "$1")
    shift
  done

  if [[ "$use_baselines" == "1" ]]; then
    args+=(--baseline "$baseline_dir/${name}.png")
    if [[ "$update_baselines" == "1" ]]; then
      args+=(--update-baseline)
    fi
  fi

  echo "portfolio-browser-smoke: capturing ${name}"
  RUSTC_WRAPPER= cargo "${args[@]}"
}

home_assertions=(
  '[data-portfolio-page]'
  '.ui-portfolio-hero'
  '.ui-portfolio-proof-strip'
  '[data-nav-brand-mark-wrap]'
)

work_assertions=(
  '[data-portfolio-page]'
  '[data-nav-link][href="/work"][aria-current="page"]'
  '.ui-portfolio-work-card'
  'a[href="/open-source"]'
)

open_source_assertions=(
  '[data-portfolio-page]'
  '[data-nav-link][href="/open-source"][aria-current="page"]'
  '[data-portfolio-crate-switcher]'
  '#portfolio-crate-panel-0 [data-code-block]'
)

open_source_modum_assertions=(
  '[data-portfolio-page]'
  '[data-portfolio-crate-switcher-nav] [data-tab-id="crate_2"][aria-selected="true"]'
  '#portfolio-crate-panel-2'
  '#portfolio-crate-panel-2 [data-code-block]'
)

lab_assertions=(
  '[data-lab-page]'
  '#operations-surface'
  '[data-home-hero-card]'
)

run_theme_matrix() {
  local slug="$1"
  local path="$2"
  local selector_group_name="$3"
  local -n selector_group="$selector_group_name"

  run_case "${slug}-desktop-light" "$path" "$desktop_width" "$desktop_height" light "" "${selector_group[@]}"
  run_case "${slug}-desktop-dark" "$path" "$desktop_width" "$desktop_height" dark "" "${selector_group[@]}"
  run_case "${slug}-mobile-light" "$path" "$mobile_width" "$mobile_height" light "" "${selector_group[@]}"
  run_case "${slug}-mobile-dark" "$path" "$mobile_width" "$mobile_height" dark "" "${selector_group[@]}"
}

case "$mode" in
  smoke)
    run_case "home-desktop-light" "/" "$desktop_width" "$desktop_height" light "" "${home_assertions[@]}"
    run_case "work-desktop-light" "/work" "$desktop_width" "$desktop_height" light "" "${work_assertions[@]}"
    run_case "open-source-desktop-light" "/open-source" "$desktop_width" "$desktop_height" light "" "${open_source_assertions[@]}"
    run_case "open-source-modum-desktop-light" "/open-source" "$desktop_width" "$desktop_height" light \
      '[data-portfolio-crate-switcher-nav] [data-tab-id="crate_2"]' \
      "${open_source_modum_assertions[@]}"
    run_case "lab-desktop-light" "/lab" "$desktop_width" "$desktop_height" light "" "${lab_assertions[@]}"
    run_case "home-mobile-dark" "/" "$mobile_width" "$mobile_height" dark "" "${home_assertions[@]}"
    run_case "work-mobile-dark" "/work" "$mobile_width" "$mobile_height" dark "" "${work_assertions[@]}"
    run_case "open-source-mobile-dark" "/open-source" "$mobile_width" "$mobile_height" dark "" "${open_source_assertions[@]}"
    run_case "lab-mobile-dark" "/lab" "$mobile_width" "$mobile_height" dark "" "${lab_assertions[@]}"
    ;;
  matrix)
    run_theme_matrix "home" "/" home_assertions
    run_theme_matrix "work" "/work" work_assertions
    run_theme_matrix "open-source" "/open-source" open_source_assertions
    run_case "open-source-modum-desktop-light" "/open-source" "$desktop_width" "$desktop_height" light \
      '[data-portfolio-crate-switcher-nav] [data-tab-id="crate_2"]' \
      "${open_source_modum_assertions[@]}"
    run_case "open-source-modum-mobile-dark" "/open-source" "$mobile_width" "$mobile_height" dark \
      '[data-portfolio-crate-switcher-nav] [data-tab-id="crate_2"]' \
      "${open_source_modum_assertions[@]}"
    run_theme_matrix "lab" "/lab" lab_assertions
    ;;
  *)
    echo "error: unsupported PORTFOLIO_SMOKE_MODE=${mode}; expected smoke or matrix" >&2
    exit 1
    ;;
esac

echo "portfolio-browser-smoke: completed ${mode} coverage against ${base_url}"
