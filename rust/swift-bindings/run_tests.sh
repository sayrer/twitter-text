#!/bin/bash
# Wrapper script to run Swift tests on Linux via bazel test
# Dynamically find the Swift runtime libs directory in the runfiles tree
SWIFT_RUNTIME_DIR=$(dirname "$(find . -path '*/usr/lib/swift/linux/libswiftCore.so' -print -quit 2>/dev/null)")
FFI_DIR=$(dirname "$(find . -name 'libtwitter_text_ffi.so' -print -quit 2>/dev/null)")
export LD_LIBRARY_PATH="${SWIFT_RUNTIME_DIR}:${FFI_DIR}:${LD_LIBRARY_PATH}"
exec "$1"
