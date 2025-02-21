# Prepare recipe with cargo chef. Pay installation cost once, will be cached from second build onwards
# https://github.com/LukeMathWalker/cargo-chef
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
# https://medium.com/@eng.fernandosilva/multi-stage-docker-build-for-rust-using-custom-ubuntu-images-distro-less-way-2710465172c7

ARG RUST_VERSION=1.85.0
ARG GOLANG_VERSION=1.24.0
ARG CHISEL_VERSION=v1.1.9
ARG CHEF_VERSION=0.1.71
ARG UBUNTU_VERSION=24.04
ARG workdir=/usr/local/src
ARG CARGO_HOME=/usr/local/cargo

# --- Base OS with Chisel ---
FROM golang:${GOLANG_VERSION} AS chisel_builder
ARG CHISEL_VERSION
RUN go install github.com/canonical/chisel/cmd/chisel@${CHISEL_VERSION}  # Pin Chisel version

FROM ubuntu:${UBUNTU_VERSION} AS os
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /rootfs
COPY --from=chisel_builder /go/bin/chisel /usr/local/bin/chisel
ARG UBUNTU_VERSION
RUN chisel cut --release ubuntu-${UBUNTU_VERSION} --root /rootfs \
    base-files_base \
    base-files_release-info \
    ca-certificates_data \
    libgcc-s1_libs \
    libc6_libs

# --- Prepare Cargo Dependencies ---
FROM rust:${RUST_VERSION} AS prepper
ARG workdir
ARG CHEF_VERSION
WORKDIR ${workdir}
RUN cargo install cargo-chef --version ${CHEF_VERSION}
COPY Cargo.toml ./
COPY src src/
# Generate Cargo.lock if it doesn't exist
RUN touch Cargo.lock
RUN cargo chef prepare --recipe-path recipe.json

# --- Cache Dependencies ---
FROM rust:${RUST_VERSION} AS cacher
ARG workdir
ARG CARGO_HOME
ARG CHEF_VERSION
WORKDIR ${workdir}
RUN cargo install cargo-chef --version ${CHEF_VERSION}
COPY --from=prepper ${workdir}/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# --- Build Application ---
FROM rust:${RUST_VERSION} AS builder
ARG workdir
ARG CARGO_HOME
WORKDIR ${workdir}
RUN rustup component add rustfmt
COPY Cargo.toml ./
COPY src src/
COPY --from=cacher ${workdir}/target target
COPY --from=cacher ${CARGO_HOME} ${CARGO_HOME}
RUN cargo build --release

# --- Final Minimal Image ---
FROM scratch

LABEL org.opencontainers.image.description="Mountains server application"
LABEL org.opencontainers.image.source="https://github.com/yourusername/mountains-server"

ARG workdir
COPY --from=os /rootfs /
COPY --from=builder ${workdir}/target/release/mountains_server /

# Add a non-root user if your rootfs includes user definitions
# USER nobody:nogroup

EXPOSE 8321
ENTRYPOINT ["/mountains_server"]