FROM rust:1.54-bullseye AS build

RUN apt-get update && apt-get install -y clang libudev-dev
RUN rustup component add rustfmt

WORKDIR /
RUN wget -q "https://github.com/solana-labs/solana/releases/download/v1.7.10/solana-release-x86_64-unknown-linux-gnu.tar.bz2" \
    && tar -xf /solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && rm /solana-release-x86_64-unknown-linux-gnu.tar.bz2
ENV PATH="$PATH:/solana-release/bin"

RUN mkdir /build
COPY Cargo.lock Cargo.toml  /build/
COPY cli/ /build/cli
COPY keys/ /build/keys
COPY flag-program/ /build/flag-program
COPY program/ /build/program

WORKDIR /build/program
RUN cargo build-bpf

WORKDIR /build/
RUN cargo build --release

RUN cp /build/target/deploy/store.so /build/ && cargo run --release --bin store-cli -- -k /build/keys/rich-boi.json initialize-ledger


FROM ubuntu:20.04

WORKDIR /
RUN apt-get update && apt-get install -y wget
RUN wget -q "https://github.com/solana-labs/solana/releases/download/v1.7.10/solana-release-x86_64-unknown-linux-gnu.tar.bz2" \
    && tar -xf /solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && rm /solana-release-x86_64-unknown-linux-gnu.tar.bz2
ENV PATH="$PATH:/solana-release/bin"

COPY --from=build /build/ledger/ /ledger/
COPY --from=build /build/target/release/store-cli /usr/bin/
COPY --from=build /build/keys/ /keys/
COPY --from=build /build/target/release/libflag.so /solana-release/bin/libflagloader_program.so

ENV FLAG="ALLES!{Nothing is secret on  🅱️ L O C K C H A I N}"
EXPOSE 1024

CMD ["/bin/sh", "-c", "((sleep 2; store-cli -u http://localhost:1024 -k /keys/rich-boi.json setup /keys/flag-depot.json $(dd if=/dev/urandom count=8 bs=1 | od -A n -t u8)) &); solana-test-validator -l /ledger/ --rpc-port 1024 --dynamic-port-range 1025-65535"]
