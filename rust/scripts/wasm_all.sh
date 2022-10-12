#!/bin/sh

set -euxo pipefail

base_dir=$(dirname "$0")
wasm_script="$base_dir/wasm.sh"

targets=( "demo-panic" "demo-serde-wasm" "demo-tsify-wasm" "playground-wasm" "schema-parser-wasm" )

for target in "${targets[@]}"; do
  $wasm_script $target
done
