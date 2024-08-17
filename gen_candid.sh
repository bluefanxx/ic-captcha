#!/bin/bash

cargo build --release --target wasm32-unknown-unknown --package backend
candid-extractor ./target/wasm32-unknown-unknown/release/backend.wasm > ./src/backend/backend.did
