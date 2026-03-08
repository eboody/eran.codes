#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <path-to-cornucopia-rs-website>" >&2
  exit 1
fi

source_repo=$1
summary_file="$source_repo/book/SUMMARY.md"
source_book_dir="$source_repo/book"
skill_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out_dir="$skill_root/references/book"

if [[ ! -f "$summary_file" ]]; then
  echo "Missing SUMMARY file: $summary_file" >&2
  exit 1
fi

mkdir -p "$out_dir"
rm -f "$out_dir"/*.md

mapfile -t entries < <(awk 'match($0,/\[([^][]+)\]\(\.\/([^)]+\.md)\)/,m){print m[1]"|"m[2]}' "$summary_file")

if [[ ${#entries[@]} -eq 0 ]]; then
  echo "No chapter entries found in: $summary_file" >&2
  exit 1
fi

index=1
for entry in "${entries[@]}"; do
  title=${entry%%|*}
  rel=${entry#*|}
  source_file="$source_book_dir/$rel"

  number=$(printf "%02d" "$index")
  slug=$(printf "%s" "$title" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')
  out_file="$out_dir/$number-$slug.md"

  {
    printf -- "# %s\n\n" "$title"
    printf -- "Source: \`book/%s\` from cornucopia-rs/website.\n\n" "$rel"

    if [[ ! -f "$source_file" ]]; then
      printf -- "[Missing source file: %s]\n" "$rel"
    else
      first_line=$(sed -n '1p' "$source_file")
      if [[ "$first_line" =~ ^#[[:space:]]+ ]]; then
        sed -n '2,$p' "$source_file"
      else
        cat "$source_file"
      fi
    fi
  } > "$out_file"

  index=$((index + 1))
done

{
  printf -- "# Cornucopia Book Chapters\n\n"
  printf -- "This directory mirrors the Cornucopia mdBook in SUMMARY order.\n\n"

  index=1
  for entry in "${entries[@]}"; do
    title=${entry%%|*}
    number=$(printf "%02d" "$index")
    slug=$(printf "%s" "$title" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')
    printf -- "- [%s](./%s-%s.md)\n" "$title" "$number" "$slug"
    index=$((index + 1))
  done
} > "$out_dir/README.md"

echo "Synced ${#entries[@]} chapter files to $out_dir"
