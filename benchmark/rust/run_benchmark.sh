#!/bin/bash
# Run the Rust benchmark

set -e

exec benchmark/rust/benchmark_bin \
    benchmark/data/autolink.yml \
    benchmark/data/extract.yml \
    benchmark/data/validate.yml \
    benchmark/data/parse.yml
