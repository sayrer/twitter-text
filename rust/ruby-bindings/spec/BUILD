load(
    "@rules_ruby//ruby:defs.bzl",
    "rb_test",
)

rb_test(
    name = "twittertext-rspec-test",
    srcs = [
        ":spec_helper.rb",
        ":autolinker_spec.rb",
        ":configuration_spec.rb",
        ":extractor_spec.rb",
        ":hithighlighter_spec.rb",
        ":validator_spec.rb",
    ],
    main = "@bundle//bin:rspec",
    args = [
        "rust/ruby-bindings/spec",
    ],
    data = [
        "@bundle//bin:rspec",
        "@bundle//:bundle",
        "//rust/ruby-bindings:twittertext",
        "//rust/cpp-bindings:config_test_data",
        "//rust/conformance:yaml",
    ],
    tags = ["no-remote", "manual"],  # Ruby hermetic toolchain doesn't support RBE
)
