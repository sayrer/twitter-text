load(
    "@bazelruby_rules_ruby//ruby:defs.bzl",
    "ruby_rspec",
    "ruby_test",
)

ruby_test(
    name = "twittertext-rspec-test",
    srcs = [
        ":spec_helper.rb",
        ":autolinker_spec.rb",
    ],
    main = "@bundle//:bin/rspec",
    args = [
        "rust_bindings/ruby/spec",
    ],
    deps = [
        "@bundle//:bin",
        "@bundle//:rspec",
    ],
    data = [
        "//rust_bindings/ruby:twitter_text_shared_library",
        "//rust/conformance:yaml",
    ],
)
