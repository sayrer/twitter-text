#!/bin/bash

bazel test //rust/... && \
bazel test //rust_bindings/cpp/... && \
bazel test //rust_bindings/cpp_sanitizers/... && \
bazel build //rust/python-bindings:twitter_text_cdylib && \
rm -f twitter_text.so && \
cp bazel-bin/rust/python-bindings/libtwitter_text_cdylib.dylib twitter_text.so && \
PYTHONPATH=. python3 -m pytest rust_bindings/python/test/ -v && \
bazel build //rust/ruby-bindings:twittertext && \
ln -sf ../../bazel-bin/rust/ruby-bindings/twittertext.bundle rust_bindings/ruby/twittertext.bundle && \
/opt/homebrew/opt/ruby@3.2/bin/ruby -I . -e "require 'rspec/autorun'; Dir['rust_bindings/ruby/spec/*_spec.rb'].sort.each { |f| load f }"
