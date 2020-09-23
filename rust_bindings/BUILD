package(default_visibility = ["//visibility:public"])

load("@bazel_skylib//rules:run_binary.bzl", "run_binary")

filegroup(
    name = "swig_input",
    srcs = ["twitter-text.i"],
)

py_binary(
    name = "swig_clean",
    srcs = ["swig_clean.py"],
)

run_binary(
    name = "twitter_text_h",
    args = [
        "$(location //rust/twitter-text:twitter_text_h)",
        "$(location :twitter-text-swig.h)",
    ],
    outs = [
        "twitter-text-swig.h"
    ],
    srcs = [
        "//rust/twitter-text:twitter_text_h",
    ],
    tool = ":swig_clean"
)