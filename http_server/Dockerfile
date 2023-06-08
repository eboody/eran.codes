FROM rust:latest

WORKDIR /builder

COPY . .

# RUN rustup component add rustfmt clippy

# RUN cargo fmt -- --check

# RUN cargo clippy -- -D warnings

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=0 /usr/local/cargo/bin/server /usr/local/bin/

EXPOSE 5001

CMD ["http_server"]
