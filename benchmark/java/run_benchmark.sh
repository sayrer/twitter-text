#!/bin/bash
# Wrapper script that makes the native library accessible for dlopen()
# On macOS, DYLD_LIBRARY_PATH doesn't work due to SIP, so we create a symlink in CWD

set -e

# CWD is already the runfiles/_main directory
# The library is at rust/java-bindings/ relative to CWD
LIB_DIR="$(pwd)/rust/java-bindings"

if [[ ! -d "$LIB_DIR" ]]; then
    echo "ERROR: Library directory not found: $LIB_DIR" >&2
    exit 1
fi

# Determine the library filename
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_FILE="libtwitter_text_java_ffm.dylib"
else
    LIB_FILE="libtwitter_text_java_ffm.so"
fi

# Create a symlink in CWD so dlopen() can find it
# (dlopen searches CWD on both macOS and Linux)
if [[ -e "$LIB_DIR/$LIB_FILE" ]] && [[ ! -e "./$LIB_FILE" ]]; then
    ln -sf "$LIB_DIR/$LIB_FILE" "./$LIB_FILE"
fi

# Also set LD_LIBRARY_PATH for Linux (and try DYLD_LIBRARY_PATH for macOS, just in case)
if [[ "$OSTYPE" == "darwin"* ]]; then
    export DYLD_LIBRARY_PATH="$LIB_DIR:$(pwd):${DYLD_LIBRARY_PATH:-}"
else
    export LD_LIBRARY_PATH="$LIB_DIR:$(pwd):${LD_LIBRARY_PATH:-}"
fi

# The java binary is at benchmark/java/benchmark_binary relative to runfiles
JAVA_BIN="$(pwd)/benchmark/java/benchmark_binary"

exec "$JAVA_BIN" "$@"
