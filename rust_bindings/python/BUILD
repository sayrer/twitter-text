load("@rules_python//python:defs.bzl", "py_binary")

py_binary(
  	name = "demo",
  	srcs = ["demo.py"],
  	deps = [":twitter_text"]
)

cc_library(
	name = "twitter_text",
    srcs = [":twitter-text-py.cpp"],
    hdrs = [":twitter-text-py.h"],
)

#genrule(
#    name = "swig_output",
#    srcs = ["//rust_bindings/cbindgen:swig_input", "//rust_bindings/cbindgen:twitter_text_header"],
#    outs = ["twitter-text-py.cpp"],
#    cmd = "$(location @swig//:bin/swig) -v -python -o $(location twitter-text-py.cpp) -I$$(dirname $(location //rust_bindings/cbindgen:swig_input)) $(location //rust_bindings/cbindgen:swig_input)",
#    tools = ["@swig//:bin/swig"],
#)

genrule(
    name = "swig_output",
    srcs = ["//rust_bindings/cbindgen:swig_input", "//rust_bindings/cbindgen:twitter_text_header"],
    outs = ["twitter-text-py.cpp"],
    cmd = "$(location @swig//:bin/swig) -v -python -o $(location twitter-text-py.cpp) -I$$(dirname $(location //rust_bindings/cbindgen:swig_input)) $(location //rust_bindings/cbindgen:swig_input)",
    tools = ["@swig//:bin/swig"],
)