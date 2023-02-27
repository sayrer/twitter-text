package(default_visibility = ["//visibility:public"])

filegroup(
    name = "swig_input",
    srcs = ["twitter-text.i"],
)

#
# The cxx crate generates some modern C++ that SWIG can't parse.
# This code is not needed for the wrappers. This code creates
# a copy of the twitter.h file for parsing, with that stuff
# commented out.
#

genrule(
    name = "cleaned_header",
    srcs = ["//rust/twitter-text:twitter_text_h"],
    outs = ["twittertext-cleaned.h"],
    cmd = "$(location //rust_bindings/tools/cxxcleaner:cxxcleaner) $(location //rust/twitter-text:twitter_text_h) > \"$@\"",
    tools = ["//rust_bindings/tools/cxxcleaner:cxxcleaner"],
)