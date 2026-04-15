#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt

deploy:
    cargo run --release
