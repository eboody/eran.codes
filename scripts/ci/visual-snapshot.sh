#!/usr/bin/env bash
set -euo pipefail

url="${VISUAL_URL:-http://127.0.0.1:3000/}"
output="${VISUAL_OUTPUT:-artifacts/visual/current/home.png}"
baseline="${VISUAL_BASELINE:-artifacts/visual/baseline/home.png}"
wait_ms="${VISUAL_WAIT_MS:-1200}"
viewport_width="${VISUAL_VIEWPORT_WIDTH:-1440}"
viewport_height="${VISUAL_VIEWPORT_HEIGHT:-1024}"

args=(
  run -p utils --bin visual_snapshot --
  --url "$url"
  --output "$output"
  --baseline "$baseline"
  --wait-ms "$wait_ms"
  --viewport-width "$viewport_width"
  --viewport-height "$viewport_height"
)

if [[ "${VISUAL_UPDATE_BASELINE:-0}" == "1" ]]; then
  args+=(--update-baseline)
fi

RUSTC_WRAPPER= cargo "${args[@]}"
