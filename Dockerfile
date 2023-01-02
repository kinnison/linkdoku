FROM rust:latest AS builder

RUN apt update
RUN apt install -y build-essential libpq-dev libssl-dev

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN cargo install wasm-opt

RUN mkdir -p /build

COPY ./ /build/

RUN (cd /build/frontend; trunk build --release index.html)
RUN (cd /build/backend; cargo build --release)

FROM debian:bullseye-slim as runner

RUN apt update
RUN apt install -y libssl1.1 libpq5 ca-certificates

COPY --from=builder /build/target/release/backend /linkdoku
COPY --from=builder /build/backend/linkdoku-config-bitio-scaleway-beta.yaml /linkdoku-config.yaml

ENTRYPOINT [ "/linkdoku" ]
