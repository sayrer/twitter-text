#!/bin/bash
# Run the Ruby benchmark with proper setup

set -e

# CWD is already the runfiles/_main directory when run via bazel run
export RUNFILES_DIR="$(pwd)"

# Run the benchmark
exec ruby "benchmark/ruby/benchmark.rb" "$@"
