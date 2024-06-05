FROM clux/muslrust:stable AS chef
ARG MASTER_BACKEND_PORT
RUN cargo install cargo-chef
WORKDIR /agent-wireguard

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /agent-wireguard/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM linuxserver/wireguard
COPY --from=builder /agent-wireguard/target/x86_64-unknown-linux-musl/release/agent-wireguard /agent-wireguard
ENTRYPOINT ["/agent-wireguard"]

EXPOSE 7070
EXPOSE 51820/udp