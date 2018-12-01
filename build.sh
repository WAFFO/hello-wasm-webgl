#!/bin/sh

set -ex
cd "$(dirname $0)"

cargo build --target wasm32-unknown-unknown

wasm-bindgen ./target/wasm32-unknown-unknown/debug/hello_webgl.wasm --out-dir ./www/wasm
