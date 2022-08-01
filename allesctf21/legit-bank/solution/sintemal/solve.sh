#!/bin/bash
cargo build-bpf
realpath=$(realpath .)
bank_initializer_program=$(solana program deploy ${realpath}/target/deploy/bank_initializer.so -k ../../deploy/keys/rich-boi.json | cut -d" " -f3)

cargo build

spl-token create-account "F1agMint11111111111111111111111111111111111" --owner ../../deploy/keys/rich-boi.json

rich_boi_path=$(realpath ../../deploy/keys/rich-boi.json)

${realpath}/target/debug/solution ${rich_boi_path} ${bank_initializer_program}

