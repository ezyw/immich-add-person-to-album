# syntax=docker/dockerfile:1
# Build the Rust executable
FROM rust:1-bookworm AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release --locked \
    && cp target/release/ImmichPersonToAlbum /build/immich-person-to-album

# Package the executable
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/immich-person-to-album /usr/local/bin/immich-person-to-album

ENTRYPOINT ["immich-person-to-album"]