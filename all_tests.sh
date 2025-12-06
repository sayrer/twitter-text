#!/bin/bash

bazel test //rust/... && \
bazel test //rust_bindings/cpp/... && \
bazel test //rust_bindings/cpp_sanitizers/... && \
bazel test //rust/python-bindings/test/... && \
#bazel test //rust/ruby-bindings/spec/... && \
echo "Done."
