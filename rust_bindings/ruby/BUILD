package(default_visibility = ["//visibility:public"])

load(
    "@bazelruby_rules_ruby//ruby:defs.bzl",
    "ruby_binary",
)

ruby_binary(
    name = "demo",
    srcs = ["demo.rb"],
    data = [":twitter_text_shared_library"],
)

config_setting(
    name = "requires_bundle",
    constraint_values = ["@platforms//os:osx"],
)

filegroup(
    name = "twitter_text_shared_library",
    srcs = select({
        ":requires_bundle": ["twittertext.bundle"],
        "//conditions:default": ["twittertext.so"],
    }),
)

genrule(
    name = "gen_twitter_text",
    testonly = True,
    srcs = [":twitter_text_loadable"],
    outs = ["twittertext.bundle"],
    cmd = "cp $< $@",
    tags = ["manual"],
)

apple_binary(
    name = "twitter_text_loadable",
    testonly = True,
    binary_type = "loadable_bundle",
    linkopts = [
        "-undefined,dynamic_lookup",
        "-multiply_defined,suppress",
    ],
    platform_type = "macos",
    tags = ["manual"],
    deps = [
        ":twitter_text_lib",
    ],
)

cc_library(
    name = "twitter_text_lib",
    testonly = True,
    srcs = [
        ":twitter-text-ruby.cpp",
        "//rust_bindings/cpp:twitter_text_h",
        "//rust_bindings/cpp:twitter_text_cpp",
    ],
    deps = [
        "@org_ruby_lang_ruby_toolchain//:headers",
        "//rust_bindings/cpp:twitter_text",
    ],
    tags = ["manual"],
    linkstatic = True,
    alwayslink = True,
)

cc_binary(
    name = "twittertext.so",
    srcs = [
        ":twitter-text-ruby.cpp",
        "//rust_bindings/cpp:twitter_text_h",
        "//rust_bindings/cpp:twitter_text_cpp",
    ],
    deps = [
        "@org_ruby_lang_ruby_toolchain//:headers",
        "//rust_bindings/cpp:twitter_text",
    ],
    linkstatic = True,
    linkshared = True,
)

genrule(
    name = "swig_output",
    srcs = [
        "//rust_bindings:swig_input", 
        "//rust_bindings/cpp:twitter_text_h",
        "//rust/twitter-text:twitter_text_h",
    ],
    outs = ["twitter-text-ruby.cpp"],
    cmd = "$(location @swig4//:bin/swig) -v -c++ -Dfinal= -ruby -o $(location twitter-text-ruby.cpp) -I$(BINDIR) $(location //rust_bindings:swig_input)",
    tools = ["@swig4//:bin/swig"],
)
