load("@rules_python//python:defs.bzl", "py_binary")

py_binary(
  name = "demo",
  srcs = ["demo.py"],
  data = [":twitter_text.so"],
  imports=['.'],
)

cc_binary(
  name = "twitter_text.so",
  srcs = [":twitter_text"],
  linkstatic = True, 
  linkshared = True,
)

cc_library(
  name = "twitter_text",
  srcs = [
    ":twitter-text-py.cpp",
    "//rust_bindings/cpp:twitter_text_h",
    "//rust_bindings/cpp:twitter_text_cpp",
  ],
  copts = ["-Iexternal/python3Full/include/python3.7/"],
  deps = [
      ":python_headers",
      "//rust/twitter-text:twitter_text_wrapper",
  ],
  linkstatic = True,
)

cc_library(
  name = "python_headers",
  hdrs = ["@python3Full//:include"],
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
    srcs = [
      "//rust_bindings:swig_input", 
      "//rust_bindings/cpp:twitter_text_h",
    ],
    outs = ["twitter-text-py.cpp"],
    cmd = "$(location @swig//:bin/swig) -v -c++ -Dfinal= -python -o $(location twitter-text-py.cpp) -I$(BINDIR) $(location //rust_bindings:swig_input)",
    tools = ["@swig//:bin/swig"],
)