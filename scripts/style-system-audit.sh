#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for style-system audit."
  exit 1
fi

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

APP_CSS="crates/http/static/app.css"
VIEW_ROOT="crates/http/src/views"

if [[ ! -f "$APP_CSS" ]]; then
  echo "error: expected global stylesheet at ${APP_CSS}."
  exit 1
fi

echo "== Shared utilities/packages without consumers =="
packages_reported=0
while IFS= read -r selector; do
  pkg=${selector#.}
  consumer_count=$(rg --fixed-strings --glob '*.rs' --glob '*.md' --glob '*.json' "$pkg" "$VIEW_ROOT" docs generated tests/fixtures 2>/dev/null | wc -l | tr -d ' ')
  if [[ "$consumer_count" == "0" ]]; then
    echo "$pkg"
    packages_reported=1
  fi
done < <(rg -o '^\.(?:ui|u)-[a-z0-9-]+' "$APP_CSS" | sort -u)

if [[ "$packages_reported" == "0" ]]; then
  echo "none"
fi

echo
echo "== Semantic tokens defined but only referenced once =="
tokens_reported=0
while IFS= read -r token; do
  token_count=$(rg -o --fixed-strings -- "$token" "$APP_CSS" "$VIEW_ROOT" docs generated tests/fixtures 2>/dev/null | wc -l | tr -d ' ')
  if [[ "$token_count" == "1" ]]; then
    echo "$token"
    tokens_reported=1
  fi
done < <(rg -o --no-filename '^\s*--(ui|surface|text|border|accent|space|radius|shadow|motion|control|portfolio)-[a-z0-9-]+' "$APP_CSS" | sed 's/^\s*//' | sort -u)

if [[ "$tokens_reported" == "0" ]]; then
  echo "none"
fi

echo
echo "== Raw literal hotspots in app.css =="
literal_output=$(rg -o --no-filename '#[0-9A-Fa-f]{3,8}|hsl\([^)]*\)|\b[0-9]+(?:\.[0-9]+)?(?:px|rem)\b' "$APP_CSS" \
  | sort \
  | uniq -c \
  | sort -nr \
  | awk '$1 > 1 { printf "%s %s\n", $1, $2 }' \
  | sed -n '1,20p')

if [[ -n "$literal_output" ]]; then
  echo "$literal_output"
else
  echo "none"
fi

echo
echo "== Repeated multi-package class clusters in view code =="
cluster_output=$(rg -o --no-filename '(?:ui|u)-[a-z0-9-]+(?:(?: (?:ui|u)-[a-z0-9-]+)+)' "$VIEW_ROOT" \
  | sort \
  | uniq -c \
  | sort -nr \
  | awk '$1 > 1 { $1=$1; print }' \
  | sed -n '1,20p')

if [[ -n "$cluster_output" ]]; then
  echo "$cluster_output"
else
  echo "none"
fi

echo
echo "== Direct Open Props usage in authored surfaces =="
open_props_output=$(rg -n -- '--(size|gray|stone|sand|red|pink|purple|indigo|blue|cyan|teal|green|lime|yellow|orange|choco|shadow|radius|font|animation|ease|duration|aspect|gradient|brand)-' \
  "$APP_CSS" "$VIEW_ROOT" generated tests/fixtures \
  | sed -n '1,40p')

if [[ -n "$open_props_output" ]]; then
  echo "$open_props_output"
else
  echo "none"
fi

echo
echo "Audit complete. This script reports migration candidates only and never fails the build."
exit 0
