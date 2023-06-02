FROM rust:latest

WORKDIR /builder

COPY . .

# RUN rustup component add rustfmt clippy

# RUN cargo fmt -- --check

# RUN cargo clippy -- -D warnings

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=0 /usr/local/cargo/bin/tonic_api /usr/local/bin/

EXPOSE 5001

CMD ["tonic_api"]
