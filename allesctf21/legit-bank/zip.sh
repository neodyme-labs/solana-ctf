#!/bin/bash
set -exuo pipefail

rm -rf public/legit-bank.zip
SRCDIR="$(dirname $(realpath "${BASH_SOURCE[0]}"))"
echo $SRCDIR

WORKDIR="$(mktemp -d)"
trap "rm -rf '$WORKDIR'" EXIT

cd "$WORKDIR"
mkdir legit-bank
find "$SRCDIR/deploy/" -maxdepth 1 -mindepth 1 -not -name "target" -and -not -name "ledger" -exec cp -r -t "legit-bank/" "{}" "+"
cp -r "$SRCDIR/public/keys" legit-bank/
rm legit-bank/keys/bank-manager.json
rm legit-bank/keys/flag-depot.json
cp -r "$SRCDIR/public/Dockerfile" legit-bank/
zip -r $SRCDIR/public/legit-bank.zip legit-bank

cd "$SRCDIR"
shasum=$(sha256sum public/legit-bank.zip | cut -d " " -f 1)
sed -i "/sha256sum: /s/\".*\"/\"$shasum\"/" legit-bank.cue
