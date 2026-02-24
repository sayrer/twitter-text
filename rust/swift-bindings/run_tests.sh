#!/bin/bash
# Wrapper script to run Swift tests on Linux via bazel test
# Dynamically find the Swift runtime libs directory in the runfiles tree.
# -L is required because Bazel runfiles uses symlinks for external repo directories.
RUNFILES="${TEST_SRCDIR:-.}"
SWIFT_RUNTIME_DIR=$(dirname "$(find -L "$RUNFILES" -path '*/usr/lib/swift/linux/libswiftCore.so' -print -quit 2>/dev/null)")
FFI_DIR=$(dirname "$(find -L "$RUNFILES" -name 'libtwitter_text_ffi.so' -print -quit 2>/dev/null)")
export LD_LIBRARY_PATH="${SWIFT_RUNTIME_DIR}:${FFI_DIR}:${LD_LIBRARY_PATH}"
exec "$1"
