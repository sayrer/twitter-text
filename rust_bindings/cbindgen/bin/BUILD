package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_test")

rust_binary(
    name = "cbindgen",
    srcs = ["src/main.rs"],
    deps = [
    	"//rust_bindings/cargo:cbindgen",
    	"//rust_bindings/cargo:clap",
	    "//rust_bindings/cargo:log",
    ],
    crate_features = ["std"],
    edition = "2018",
)

rust_test(
	name = "cbindgen_test",
    crate = ":cbindgen",
    edition = "2018",
)