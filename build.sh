#!/bin/sh

set -ex
cd "$(dirname $0)"

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build --target wasm32-unknown-unknown

wasm-bindgen ./target/wasm32-unknown-unknown/debug/hello_webgl.wasm --out-dir ./www/wasm
