FROM rust:latest AS base-builder

RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN cargo install wasm-opt
RUN cargo install grass
RUN cargo install css-minifier

RUN apt update

RUN apt install -y musl-tools

RUN mkdir -p /build

FROM base-builder as builder

COPY ./ /build/

RUN (cd /build/css; make)
RUN (cd /build/frontend; trunk build --release index.html)
RUN (cd /build/backend; cargo build --target=x86_64-unknown-linux-musl --release)

FROM scratch as runner

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/backend /linkdoku
COPY --from=builder /build/backend/linkdoku-config-bitio-scaleway-beta.yaml /linkdoku-config.yaml

HEALTHCHECK --start-period=30s --interval=5m --timeout=15s \
    CMD [ "/linkdoku --healthcheck" ]

ENTRYPOINT [ "/linkdoku" ]
