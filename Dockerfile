FROM rust:1.85-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/eran_codes /app/eran_codes
COPY crates/http/static /app/crates/http/static
COPY crates/infra/migrations /app/crates/infra/migrations

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["/app/eran_codes"]
