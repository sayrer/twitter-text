genrule(
    name = "cxxbridge",
    outs = ["cxx-bridge.h"],
    cmd = "$(location //rust_bindings/cargo:cargo_bin_cxxbridge) --header $(location cxx-bridge.h)",
    tools = ["//rust_bindings/cargo:cargo_bin_cxxbridge"],
)

genrule(
    name = "twitter_text_h",
    srcs = [
    	"//rust:twitter_text_main",
    ],
    outs = ["twitter-text.h"],
    cmd = "$(location //rust_bindings/cargo:cargo_bin_cxxbridge) --header $(location //rust:twitter_text_main) > $(location twitter-text.h)",
    tools = ["//rust_bindings/cargo:cargo_bin_cxxbridge"],
)

genrule(
    name = "twitter_text_cpp",
    srcs = [
    	"//rust:twitter_text_main",
    ],
    outs = ["twitter-text.cpp"],
    cmd = "$(location //rust_bindings/cargo:cargo_bin_cxxbridge) $(location //rust:twitter_text_main) > $(location twitter-text.cpp)",
    tools = ["//rust_bindings/cargo:cargo_bin_cxxbridge"],
)