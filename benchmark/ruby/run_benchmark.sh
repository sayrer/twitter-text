#!/bin/bash
# Run the Ruby benchmark with proper setup
#
# Prerequisites: The old Ruby twitter-text library requires native gems.
# Install them before running:
#
#   brew install libidn
#   gem install idn-ruby -- --with-idn-dir=/opt/homebrew
#   gem install unf
#

set -e

# CWD is already the runfiles/_main directory when run via bazel run
export RUNFILES_DIR="$(pwd)"

# Run the benchmark
exec ruby "benchmark/ruby/benchmark.rb" "$@"
