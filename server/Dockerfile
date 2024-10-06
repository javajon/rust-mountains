# Prepare recipe with cargo chef. Pay installation cost once, will be cached from second build onwards
# https://github.com/LukeMathWalker/cargo-chef
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
# https://medium.com/@eng.fernandosilva/multi-stage-docker-build-for-rust-using-custom-ubuntu-images-distro-less-way-2710465172c7

ARG workdir=/usr/local/src

FROM ubuntu:24.04 as os
RUN apt-get update && apt-get update && apt-get install -y wget
WORKDIR /tmp
RUN wget https://go.dev/dl/go1.23.2.linux-amd64.tar.gz
RUN tar -xvf go1.23.2.linux-amd64.tar.gz
RUN mv go /usr/local
RUN GOBIN=/usr/local/bin/ /usr/local/go/bin/go install github.com/canonical/chisel/cmd/chisel@latest
WORKDIR /rootfs
RUN chisel cut --release ubuntu-24.04 --root /rootfs \
    base-files_base \
    base-files_release-info \
    ca-certificates_data \
    libgcc-s1_libs \
    libc6_libs

FROM rust:1.81.0 as prepper
ARG workdir
WORKDIR ${workdir}
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---
# Pay dependency cost once, will be cached from second build onwards
FROM rust:1.81.0 as cacher
ARG workdir
WORKDIR ${workdir}
RUN cargo install cargo-chef
COPY --from=prepper ${workdir}/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ---
# Copy cached dependencies, load source and build
FROM rust:1.81.0 as builder
ARG workdir
WORKDIR ${workdir}
RUN rustup component add rustfmt
COPY . .
COPY --from=cacher ${workdir}/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

# ---
# Final distillation
FROM scratch
ARG workdir
COPY --from=os /rootfs /
COPY --from=builder ${workdir}/target/release/mountains_server .
EXPOSE 8321
ENTRYPOINT ["./mountains_server"]
