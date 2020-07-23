package(default_visibility = ["//visibility:public"])

load("@rules_python//python:defs.bzl", "py_binary")

py_binary(
  name = "demo",
  srcs = ["demo.py"],
  deps = [
    ":twitter_text",
  ],
  imports =["."]
)

py_library(
  name = "twitter_text",
  srcs = [":twitter_text.py"],
  data = [":_twitter_text.so"],
)

cc_binary(
  name = "_twitter_text.so",
  srcs = [
    ":twitter-text-py.cpp",
    "//rust_bindings/cpp:twitter_text_h",
    "//rust_bindings/cpp:twitter_text_cpp",
  ],
  copts = ["-Iexternal/python3Full/include/python3.7/"],
  deps = [
      ":python_headers",
      "//rust_bindings/cpp:twitter_text",
  ],
  linkstatic = True,
  linkshared = True,
)

cc_library(
  name = "python_headers",
  hdrs = ["@python3Full//:include"],
)

genrule(
    name = "swig_output",
    srcs = [
      "//rust_bindings:swig_input", 
      "//rust_bindings/cpp:twitter_text_h",
    ],
    outs = ["twitter-text-py.cpp", "twitter_text.py"],
    cmd = "$(location @swig4//:bin/swig) -v -c++ -Dfinal= -python -o $(location twitter-text-py.cpp) -I$(BINDIR) $(location //rust_bindings:swig_input)",
    tools = ["@swig4//:bin/swig"],
)
