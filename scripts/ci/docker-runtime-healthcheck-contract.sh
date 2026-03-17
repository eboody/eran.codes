#!/usr/bin/env bash
set -euo pipefail

dockerfile="Dockerfile"

if [[ ! -f "$dockerfile" ]]; then
  echo "error: ${dockerfile} not found"
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required for this check."
  exit 1
fi

if ! rg -q 'apt-get install -y --no-install-recommends .*curl' "$dockerfile"; then
  echo "error: runtime image must install curl for image-level healthchecks"
  exit 1
fi

if ! rg -q 'apt-get install -y --no-install-recommends .*wget' "$dockerfile"; then
  echo "error: runtime image must install wget for platform healthchecks"
  exit 1
fi

if ! rg -q 'CMD curl --fail --silent --show-error "http://127\.0\.0\.1:\$\{PORT\}/health"' "$dockerfile"; then
  echo "error: Dockerfile healthcheck must probe http://127.0.0.1:\${PORT}/health with curl"
  exit 1
fi
