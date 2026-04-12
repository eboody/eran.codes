#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

base_url="${PORTFOLIO_SMOKE_BASE_URL:-http://127.0.0.1:3000}"
current_dir="${PORTFOLIO_SMOKE_CURRENT_DIR:-artifacts/visual/current/portfolio-smoke}"
baseline_dir="${PORTFOLIO_SMOKE_BASELINE_DIR:-artifacts/visual/baseline/portfolio-smoke}"
wait_ms="${PORTFOLIO_SMOKE_WAIT_MS:-1400}"
assert_timeout_ms="${PORTFOLIO_SMOKE_ASSERT_TIMEOUT_MS:-6000}"
desktop_width="${PORTFOLIO_SMOKE_DESKTOP_WIDTH:-1920}"
desktop_height="${PORTFOLIO_SMOKE_DESKTOP_HEIGHT:-1080}"
use_baselines="${PORTFOLIO_SMOKE_USE_BASELINES:-0}"
update_baselines="${PORTFOLIO_SMOKE_UPDATE_BASELINE:-0}"
prune_current="${PORTFOLIO_SMOKE_PRUNE_CURRENT:-1}"

mkdir -p "$current_dir"
mkdir -p "$baseline_dir"

if [[ "$prune_current" == "1" ]]; then
  find "$current_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.html' \) -delete
fi

run_case() {
  local name="$1"
  local path="$2"
  local focus_selector="$3"
  shift 3

  local output="$current_dir/${name}.png"
  local html="$current_dir/${name}.html"
  local url="${base_url%/}${path}"
  local baseline="$baseline_dir/${name}.png"
  local args=(
    run -p utils --features visual-snapshot --bin visual_snapshot --
    --url "$url"
    --output "$output"
    --dump-html "$html"
    --wait-ms "$wait_ms"
    --viewport-width "$desktop_width"
    --viewport-height "$desktop_height"
    --color-scheme light
    --assert-timeout-ms "$assert_timeout_ms"
  )

  if [[ -n "$focus_selector" ]]; then
    args+=(--element-selector "$focus_selector")
  fi

  if [[ "$path" == "/lab" ]]; then
    args+=(
      --normalize-text-selector '.ui-log-flow-item-id=>request'
      --normalize-text-selector '.ui-log-flow-item-time=>time'
      --normalize-text-selector '.ui-log-flow-detail-header .ui-pill=>request_id=req'
      --normalize-text-selector '[data-log-timestamp]=>time'
      --normalize-text-selector '.ui-log-flow-event .ui-pill-cluster .ui-pill:last-child=>latency_ms=xx'
    )
  fi

  while [[ $# -gt 0 ]]; do
    args+=(--assert-selector "$1")
    shift
  done

  if [[ "$use_baselines" == "1" && ( -f "$baseline" || "$update_baselines" == "1" ) ]]; then
    args+=(--baseline "$baseline")
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
  '.ui-portfolio-crate-showcase'
  '.ui-portfolio-work-section--current-proof'
  '[data-nav-link][href="/"][aria-current="page"]'
)

lab_assertions=(
  '[data-lab-page]'
  '#operations-surface'
  '[data-home-hero-card]'
  '[data-nav-link][href="/lab"][aria-current="page"]'
)

current_case_assertions=(
  '[data-portfolio-page]'
  '.ui-portfolio-current-proof-detail'
  'a[href="/lab"]'
  'a[href="/work"]'
)

run_case "home-desktop-light" "/" "" "${home_assertions[@]}"
run_case "lab-desktop-light" "/lab" "#operations-surface" "${lab_assertions[@]}"
run_case \
  "work-current-desktop-light" \
  "/work/sensitive-sync" \
  "" \
  "${current_case_assertions[@]}"

echo "portfolio-browser-smoke: completed against ${base_url}"
