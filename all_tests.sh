#!/bin/bash

bazel test //rust/... && \
bazel test //rust_bindings/cpp/... && \
bazel test //rust_bindings/cpp_sanitizers/... && \
bazel test //rust/python-bindings/test/... && \
#bazel build //rust/ruby-bindings:twittertext && \
#ln -sf ../../bazel-bin/rust/ruby-bindings/twittertext.bundle rust_bindings/ruby/twittertext.bundle && \
#/opt/homebrew/opt/ruby@3.2/bin/ruby -I . -e "require 'rspec/autorun'; Dir['rust_bindings/ruby/spec/*_spec.rb'].sort.each { |f| load f }"
echo "Done."
