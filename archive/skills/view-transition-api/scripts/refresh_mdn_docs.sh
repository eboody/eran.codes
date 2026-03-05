#!/usr/bin/env bash
set -euo pipefail

skill_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
refs_dir="$skill_root/references"
list_file="${LIST_FILE:-$refs_dir/mdn-files.txt}"
out_root="${OUT_ROOT:-$refs_dir/mdn}"
log_file="${LOG_FILE:-$refs_dir/download-failures.log}"
tree_json="${TREE_JSON:-${TMPDIR:-/tmp}/mdn-content-tree.json}"
blob_json="${BLOB_JSON:-${TMPDIR:-/tmp}/mdn-blob.json}"
retry_tries="${RETRY_TRIES:-4}"
retry_wait="${RETRY_WAIT_SECS:-1}"
dns_timeout="${DNS_TIMEOUT_SECS:-4}"

if [[ ! -f "$list_file" ]]; then
  echo "missing list file: $list_file" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required" >&2
  exit 1
fi

fetch_to_file() {
  local url="$1"
  local dest="$2"
  wget -q --retry-on-host-error --tries="$retry_tries" --waitretry="$retry_wait" --dns-timeout="$dns_timeout" -O "$dest" "$url"
}

have_tree=0
if fetch_to_file "https://api.github.com/repos/mdn/content/git/trees/main?recursive=1" "$tree_json"; then
  have_tree=1
fi

mkdir -p "$out_root"
: > "$log_file"

ok_count=0
fail_count=0

while IFS= read -r path; do
  [[ -z "$path" ]] && continue

  rel="${path#files/en-us/}"
  dest="$out_root/$rel"
  mkdir -p "$(dirname "$dest")"

  raw_url="https://raw.githubusercontent.com/mdn/content/main/$path"
  tmp_file="$(mktemp)"
  downloaded=0

  if fetch_to_file "$raw_url" "$tmp_file"; then
    mv "$tmp_file" "$dest"
    downloaded=1
  else
    rm -f "$tmp_file"
  fi

  if [[ "$downloaded" -eq 0 && "$have_tree" -eq 1 ]]; then
    blob_url="$(jq -r --arg p "$path" '.tree[] | select(.path == $p) | .url' "$tree_json")"
    if [[ -n "$blob_url" ]] && fetch_to_file "$blob_url" "$blob_json"; then
      if jq -e '.encoding == "base64" and (.content | length > 0)' "$blob_json" >/dev/null 2>&1; then
        tmp_file="$(mktemp)"
        if jq -r '.content' "$blob_json" | tr -d '\n' | base64 -d > "$tmp_file"; then
          mv "$tmp_file" "$dest"
          downloaded=1
        else
          rm -f "$tmp_file"
        fi
      fi
    fi
  fi

  if [[ "$downloaded" -eq 1 ]]; then
    ok_count=$((ok_count + 1))
  else
    fail_count=$((fail_count + 1))
    echo "$path" >> "$log_file"
  fi
done < "$list_file"

echo "Downloaded: $ok_count"
echo "Failed: $fail_count"
echo "Failure log: $log_file"
