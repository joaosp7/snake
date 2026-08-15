#!/usr/bin/env bash
set -euo pipefail

readonly package_name="raylib-rs"
readonly wasm_target="wasm32-unknown-unknown"
readonly output_dir="${1:-dist}"
readonly wasm_binary="target/${wasm_target}/release/${package_name}.wasm"

rm -rf "$output_dir"
mkdir -p "$output_dir"

cargo build --release --target "$wasm_target"
cp "$wasm_binary" "$output_dir/raylib-rs.wasm"

miniquad_js=("$HOME"/.cargo/registry/src/*/miniquad-*/js/gl.js)
if [[ ${#miniquad_js[@]} -eq 0 || ! -f "${miniquad_js[0]}" ]]; then
    printf '%s\n' "Miniquad's gl.js was not found in the Cargo registry" >&2
    exit 1
fi
cp "${miniquad_js[0]}" "$output_dir/gl.js"
cp web/index.html "$output_dir/index.html"

printf 'Web bundle created in %s\n' "$output_dir"
