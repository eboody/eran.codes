#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

if ! command -v docker >/dev/null 2>&1; then
  echo "error: docker is required for the Docker runtime smoke check" >&2
  exit 1
fi

project="eran_codes_smoke_${USER:-local}_$$"
image_tag="eran_codes-smoke:${USER:-local}-$$"
network_name="${project}_net"
postgres_container="${project}_postgres"
app_container="${project}_app"
browser_smoke_mode="${DOCKER_SMOKE_BROWSER_MODE:-smoke}"
skip_browser_smoke="${DOCKER_SMOKE_SKIP_BROWSER_SMOKE:-0}"
session_secret="${DOCKER_SMOKE_SESSION_SECRET:-BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBw}"
database_url="postgresql://app:app@postgres:5432/app"

cleanup() {
  docker rm -f "$app_container" >/dev/null 2>&1 || true
  docker rm -f "$postgres_container" >/dev/null 2>&1 || true
  docker image rm -f "$image_tag" >/dev/null 2>&1 || true
  docker network rm "$network_name" >/dev/null 2>&1 || true
}

trap cleanup EXIT

echo "Starting ephemeral Postgres for Docker smoke check..."
docker network create "$network_name" >/dev/null
docker run -d \
  --name "$postgres_container" \
  --network "$network_name" \
  --network-alias postgres \
  -e POSTGRES_USER=app \
  -e POSTGRES_PASSWORD=app \
  -e POSTGRES_DB=app \
  postgres:16 >/dev/null

for _attempt in $(seq 1 30); do
  if docker exec "$postgres_container" pg_isready -U app -d app >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

if ! docker exec "$postgres_container" pg_isready -U app -d app >/dev/null 2>&1; then
  echo "error: postgres did not become ready for Docker smoke check" >&2
  docker logs "$postgres_container" >&2 || true
  exit 1
fi

echo "Building runtime image for Docker smoke check..."
docker build -t "$image_tag" . >/dev/null

echo "Booting runtime image and waiting for /health..."
docker run -d \
  --name "$app_container" \
  --network "$network_name" \
  -e HOST=0.0.0.0 \
  -e PORT=3000 \
  -e DATABASE_URL="$database_url" \
  -e SESSION_SECRET="$session_secret" \
  -p 127.0.0.1::3000 \
  "$image_tag" >/dev/null

host_port="$(
  docker port "$app_container" 3000/tcp \
    | sed -n 's/^127\.0\.0\.1:\([0-9]\+\)$/\1/p' \
    | head -n 1
)"

if [[ -z "$host_port" ]]; then
  echo "error: failed to resolve published host port for Docker smoke check" >&2
  docker logs "$app_container" >&2 || true
  exit 1
fi

for _attempt in $(seq 1 30); do
  health_status="$(
    docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}missing{{end}}' "$app_container"
  )"
  case "$health_status" in
    healthy)
      break
      ;;
    unhealthy)
      echo "error: runtime image became unhealthy during Docker smoke check" >&2
      docker logs "$app_container" >&2 || true
      exit 1
      ;;
  esac
  sleep 2
done

health_status="$(
  docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}missing{{end}}' "$app_container"
)"

if [[ "$health_status" != "healthy" ]]; then
  echo "error: runtime image did not become healthy during Docker smoke check" >&2
  docker logs "$app_container" >&2 || true
  exit 1
fi

curl --fail --silent --show-error "http://127.0.0.1:${host_port}/health" >/dev/null

if [[ "$skip_browser_smoke" != "1" ]]; then
  echo "Running portfolio browser smoke against runtime image..."
  PORTFOLIO_SMOKE_BASE_URL="http://127.0.0.1:${host_port}" \
  PORTFOLIO_SMOKE_MODE="$browser_smoke_mode" \
  PORTFOLIO_SMOKE_USE_BASELINES=0 \
  PORTFOLIO_SMOKE_CURRENT_DIR="${PORTFOLIO_SMOKE_CURRENT_DIR:-artifacts/visual/current/docker-smoke}" \
  bash scripts/check_portfolio_browser_smoke.sh
fi
