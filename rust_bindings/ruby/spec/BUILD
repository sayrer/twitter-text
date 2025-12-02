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
        "rust_bindings/ruby/spec",
    ],
    data = [
        "@bundle//bin:rspec",
        "@bundle//:bundle",
        "//rust_bindings/ruby:twitter_text_shared_library",
        "//rust_bindings/cpp:config_test_data",
        "//rust/conformance:yaml",
    ],
)
