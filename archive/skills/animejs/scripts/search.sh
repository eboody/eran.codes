#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $(basename "$0") <regex-pattern>" >&2
  exit 2
fi

pattern="$1"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

rg -n --no-heading --glob '*.html' "$pattern" \
  "$root/references/animejs-site/animejs.com/documentation" \
  "$root/references/animejs-site/animejs.com/v3/documentation" \
  "$root/references/animejs-site/animejs.com/v2/documentation" \
  "$root/references/animejs-site/animejs.com/learn" \
  "$root/references/animejs-site/animejs.com/documentation-demos"
