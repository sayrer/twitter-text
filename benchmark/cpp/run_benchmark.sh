#!/bin/bash
# Run the C++ benchmark

set -e

# Get the directory where this script is located (runfiles)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Data files (relative to script in runfiles)
DATA_DIR="$SCRIPT_DIR/../../benchmark/data"
AUTOLINK_YML="$DATA_DIR/autolink.yml"
EXTRACT_YML="$DATA_DIR/extract.yml"
VALIDATE_YML="$DATA_DIR/validate.yml"
PARSE_YML="$DATA_DIR/parse.yml"

# Binary path (in same directory as script)
BENCHMARK="$SCRIPT_DIR/cpp_benchmark"

exec "$BENCHMARK" "$AUTOLINK_YML" "$EXTRACT_YML" "$VALIDATE_YML" "$PARSE_YML"
