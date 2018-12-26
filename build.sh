#!/bin/sh

set -ex
cd "$(dirname $0)"

proj_name=`perl -ne '/name = "(.+)"/ && print $1' Cargo.toml | sed 's/\-/\_/g'`

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build --target wasm32-unknown-unknown

wasm-bindgen ./target/wasm32-unknown-unknown/debug/$proj_name.wasm --out-dir ./www/wasm
