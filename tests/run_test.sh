#!/bin/bash
cargo build 
cargo build --release
cargo test --test integration_test -- --nocapture

cargo run -- -p ./file.txt -r "\w+"