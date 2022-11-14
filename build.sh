#! /bin/bash

cargo build --target wasm32-unknown-unknown --release
#wasm-bindgen target/wasm32-unknown-unknown/release/snake.wasm --out-dir ./wasm_help --no-modules --no-typescript
wasm-bindgen target/wasm32-unknown-unknown/release/snake.wasm --out-dir ./wasm_help --no-typescript
