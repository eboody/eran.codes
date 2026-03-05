#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mirror_root="$root/references/animejs-site"
site_root="$mirror_root/animejs.com"

mkdir -p "$mirror_root"

run_wget() {
  wget --mirror \
    --page-requisites \
    --convert-links \
    --adjust-extension \
    --no-parent \
    --domains animejs.com \
    -e robots=off \
    --directory-prefix "$mirror_root" \
    --tries=5 \
    --waitretry=2 \
    https://animejs.com/documentation/ \
    https://animejs.com/v3/documentation/ \
    https://animejs.com/v2/documentation/ \
    https://animejs.com/learn/ \
    https://animejs.com/documentation-demos/
}

rc=0
for attempt in 1 2 3; do
  set +e
  run_wget
  rc=$?
  set -e
  if [[ $rc -ne 4 ]]; then
    break
  fi
  sleep 2
done

# wget exits 8 when some URLs return HTTP errors (for example optional/legacy links).
# Keep that non-fatal while still failing on transport/runtime errors.
if [[ $rc -ne 0 && $rc -ne 8 ]]; then
  echo "animejs refresh failed with wget exit code: $rc" >&2
  exit "$rc"
fi

find "$site_root" -type f -name '*.html' | sort > "$root/references/html-files.txt"

{
  echo "documentation_html_count=$(find "$site_root/documentation" -type f -name '*.html' 2>/dev/null | wc -l)"
  echo "v3_html_count=$(find "$site_root/v3/documentation" -type f -name '*.html' 2>/dev/null | wc -l)"
  echo "v2_html_count=$(find "$site_root/v2/documentation" -type f -name '*.html' 2>/dev/null | wc -l)"
  echo "learn_html_count=$(find "$site_root/learn" -type f -name '*.html' 2>/dev/null | wc -l)"
  echo "demos_html_count=$(find "$site_root/documentation-demos" -type f -name '*.html' 2>/dev/null | wc -l)"
  echo "total_html_count=$(find "$site_root" -type f -name '*.html' 2>/dev/null | wc -l)"
} > "$root/references/stats.env"

echo "animejs docs refresh complete"
