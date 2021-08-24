#!/bin/bash

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen ./target/wasm32-unknown-unknown/release/client.wasm --out-dir webpage/ --no-modules --no-typescript

http-server ./webpage/
