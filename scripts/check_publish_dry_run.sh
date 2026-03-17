#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

if ! command -v act >/dev/null 2>&1; then
  echo "error: act is required for the publish gate" >&2
  exit 1
fi

tmp_root="${ACT_TMPDIR:-$repo_root/artifacts/act-tmp}"
mkdir -p "$tmp_root"
export TMPDIR="$tmp_root"
export TMP="$tmp_root"
export TEMP="$tmp_root"

docker_tmp_root="${ACT_DOCKER_TMPDIR:-$repo_root/artifacts/act-docker-tmp}"
mkdir -p "$docker_tmp_root"
export DOCKER_TMPDIR="$docker_tmp_root"

echo "Running workspace tests (without doctests)..."
cargo test --workspace --lib --tests

echo "Running local GitHub Actions gate under act..."
act -j stringy-check
