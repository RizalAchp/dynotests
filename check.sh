#!/usr/bin/env bash

set -xe

cargo check --all-targets --all-features $@
cargo fmt --all  $@ -- --check
cargo clippy --all-targets --all-features $@ -- -D warnings -W clippy::all
