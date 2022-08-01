#!/bin/bash
set -exuo pipefail

rm -rf public/secret-store.zip
SRCDIR="$(dirname $(realpath "${BASH_SOURCE[0]}"))"
echo $SRCDIR

WORKDIR="$(mktemp -d)"
trap "rm -rf '$WORKDIR'" EXIT

cd "$WORKDIR"
mkdir secret-store
find "$SRCDIR/deploy/" -maxdepth 1 -mindepth 1 -not -name "target" -and -not -name "ledger" -exec cp -r -t "secret-store/" "{}" "+"
cp -r "$SRCDIR/public/"* secret-store/
zip -r $SRCDIR/public/secret-store.zip secret-store

cd "$SRCDIR"
shasum=$(sha256sum public/secret-store.zip | cut -d " " -f 1)
sed -i "/sha256sum: /s/\".*\"/\"$shasum\"/" secret-store.cue
