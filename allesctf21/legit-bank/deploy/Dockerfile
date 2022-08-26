FROM rust:1.54.0-bullseye AS build

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

RUN cp /build/target/deploy/bank.so /build/ && cargo run --release --bin bank-cli -- -k /build/keys/rich-boi.json initialize-ledger


FROM ubuntu:20.04

RUN apt-get update && apt-get install -y wget
RUN wget -q "https://github.com/solana-labs/solana/releases/download/v1.7.10/solana-release-x86_64-unknown-linux-gnu.tar.bz2" \
    && tar -xf /solana-release-x86_64-unknown-linux-gnu.tar.bz2 \
    && rm /solana-release-x86_64-unknown-linux-gnu.tar.bz2
ENV PATH="$PATH:/solana-release/bin"

COPY --from=build /build/ledger/ /ledger/
COPY --from=build /build/target/release/bank-cli /usr/bin/
COPY --from=build /build/keys/ /keys/
COPY --from=build /build/target/release/libflag.so /solana-release/bin/libflagloader_program.so

ENV FLAG="ALLES!{Some Smart Contracts are not very smart :(}"
EXPOSE 1024

CMD ["/bin/sh", "-c", "((sleep 2; bank-cli -k /keys/bank-manager.json setup /keys/flag-depot.json /keys/bank-manager.json) &); solana-test-validator -l /ledger/ --rpc-port 1024 --dynamic-port-range 1025-65535"]
