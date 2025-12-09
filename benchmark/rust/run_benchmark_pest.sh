#!/bin/bash
# Run benchmark with Pest backend only (for profiling)

set -e

exec benchmark/rust/benchmark_bin \
    benchmark/data/autolink.yml \
    benchmark/data/extract.yml \
    benchmark/data/validate.yml \
    benchmark/data/parse.yml \
    --backend=pest
