#!/usr/bin/env bash
set -euo pipefail

root="crates/app/src"
fail=0

collect_hasher_calls() {
  if command -v rg >/dev/null 2>&1; then
    rg --no-heading --line-number 'hasher\.(hash|verify)\(' "$root"
  elif command -v grep >/dev/null 2>&1; then
    grep -RnoE 'hasher\.(hash|verify)\(' "$root"
  else
    echo "error: rg or grep is required for this check."
    exit 1
  fi
}

scan_window_for_violations() {
  local file="$1"
  local line="$2"
  local end_line=$((line + 12))
  local window
  window="$(sed -n "${line},${end_line}p" "$file")"

  if ! printf '%s\n' "$window" | grep -q '\.map_err('; then
    return
  fi

  if printf '%s\n' "$window" | grep -qE 'Error::Repo\(|Error::Repository\('; then
    echo "${file}:${line}: hasher errors must not be mapped to repository errors."
    fail=1
  fi

  if printf '%s\n' "$window" | grep -q 'to_string('; then
    echo "${file}:${line}: hasher errors must stay typed; avoid to_string() in map_err."
    fail=1
  fi
}

while IFS=: read -r file line _; do
  scan_window_for_violations "$file" "$line"
done < <(collect_hasher_calls)

if ((fail != 0)); then
  echo
  echo "error: typed hasher error policy violated in app services."
  echo "hint: map hasher failures to dedicated typed variants (for example Error::Hashing)."
  exit 1
fi

exit 0
