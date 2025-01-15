#! /bin/bash

cargo build --target x86_64-apple-darwin -r
cargo build --target aarch64-apple-darwin -r
lipo -create -output brainz_to_sqlite target/x86_64-apple-darwin/release/brainz_to_sqlite target/aarch64-apple-darwin/release/brainz_to_sqlite