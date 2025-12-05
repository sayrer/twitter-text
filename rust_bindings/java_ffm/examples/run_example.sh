#!/usr/bin/env bash

set -euo pipefail

# Get the runfiles directory
RUNFILES="${BASH_SOURCE[0]}.runfiles"
if [[ ! -d "$RUNFILES" ]]; then
  RUNFILES="${BASH_SOURCE[0]}.runfiles"
fi

# Copy the dylib to the current directory so Java can find it
cp -f "${RUNFILES}/_main/rust_bindings/java_ffm/libtwitter_text_java_ffm.dylib" .

# Run the Java example
exec "${RUNFILES}/_main/rust_bindings/java_ffm/examples/example_java" "$@"
