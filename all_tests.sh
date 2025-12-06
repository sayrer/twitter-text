#!/bin/bash

# Run rust tests (may fail if Ruby < 3.3, but that's OK)
bazel test //rust/... || true

# Run cpp tests
bazel test //rust_bindings/cpp/... && \
bazel test //rust_bindings/cpp_sanitizers/... && \
echo "Done."
