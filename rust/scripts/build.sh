#!/bin/sh

set -euxo pipefail

CRATE_FOLDER=$1
CRATE=$(echo "$CRATE_FOLDER" | tr '-' '_')
OUT_DIR="../nodejs"

cargo build -p $CRATE_FOLDER --release --target=wasm32-unknown-unknown

echo 'Creating out dir...'
mkdir -p $OUT_DIR/src/wasm;

echo 'Generating node module...'
wasm-bindgen \
  --target nodejs \
  --out-dir $OUT_DIR/src/wasm \
  target/wasm32-unknown-unknown/release/$CRATE.wasm;
