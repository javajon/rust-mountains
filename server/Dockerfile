# Prepare recipe with cargo chef. Pay installation cost once, will be cached from second build onwards
# https://github.com/LukeMathWalker/cargo-chef
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM rust:1.48 as prepper
WORKDIR app
RUN cargo install cargo-chef --version 0.1.6
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---
# Pay dependency cost once, will be cached from second build onwards
FROM rust:1.48 as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=prepper /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ---
# Copy cached dependencies, load source and build
FROM rust:1.48 as builder
WORKDIR app
RUN rustup component add rustfmt
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# ---
# Final distillation
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/mountains_server .
EXPOSE 8321
ENTRYPOINT [ "./mountains_server" ]
