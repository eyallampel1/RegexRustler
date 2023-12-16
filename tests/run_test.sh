#!/bin/bash
cargo build 
cargo build --release
cargo test --test integration_test -- --nocapture
