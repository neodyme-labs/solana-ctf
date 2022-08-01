#!/bin/bash
set -exuo pipefail

rm -rf public/bugchain.zip
SRCDIR="$(dirname $(realpath "${BASH_SOURCE[0]}"))"
echo $SRCDIR

WORKDIR="$(mktemp -d)"
trap "rm -rf '$WORKDIR'" EXIT

cd "$WORKDIR"
mkdir bugchain
find "$SRCDIR/deploy/" -maxdepth 1 -mindepth 1 -not -name "target" -and -not -name "ledger" -exec cp -r -t "bugchain/" "{}" "+"
cp -r "$SRCDIR/public/"* bugchain/
zip -r $SRCDIR/public/bugchain.zip bugchain

cd "$SRCDIR"
shasum=$(sha256sum public/bugchain.zip | cut -d " " -f 1)
sed -i "/sha256sum: /s/\".*\"/\"$shasum\"/" bugchain.cue
