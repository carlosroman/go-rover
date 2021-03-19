ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION}

WORKDIR /app

RUN set -eu && \
    rustup component add \
        rustfmt \
        clippy;

COPY Cargo.toml Cargo.lock /app/

COPY src/main.rs /app/src/

RUN cargo fetch

