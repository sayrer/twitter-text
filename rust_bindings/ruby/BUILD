package(default_visibility = ["//visibility:public"])

load(
    "@rules_ruby//ruby:defs.bzl",
    "rb_binary",
)

# Ruby extension using Magnus
filegroup(
    name = "twitter_text_shared_library",
    srcs = ["//rust/ruby-bindings:twittertext"],
)

rb_binary(
    name = "demo",
    srcs = ["demo.rb"],
    data = [":twitter_text_shared_library"],
)
