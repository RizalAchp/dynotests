#!/usr/bin/env bash

set -xe

cargo test --all-targets --all-features $@
cargo test --doc %@
