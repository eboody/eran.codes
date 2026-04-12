#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

echo "Running workspace tests..."
cargo test --workspace --lib --tests

echo "Running Docker runtime smoke..."
bash scripts/check_docker_runtime_smoke.sh
