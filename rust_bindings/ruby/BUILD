package(default_visibility = ["//visibility:public"])

load(
    "@bazelruby_rules_ruby//ruby:defs.bzl",
    "ruby_binary",
)

# Ruby extension using Magnus
filegroup(
    name = "twitter_text_shared_library",
    srcs = ["//rust/ruby-bindings:twittertext"],
)

ruby_binary(
    name = "demo",
    srcs = ["demo.rb"],
    data = [":twitter_text_shared_library"],
)
