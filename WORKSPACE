
#
# Bazel infrastructure
#
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

http_archive(
    name = "bazel_skylib",
    sha256 = "2c62d8cd4ab1e65c08647eb4afe38f51591f43f7f0885e7769832fa137633dcb",
    strip_prefix = "bazel-skylib-0.7.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.7.0.tar.gz",
)

#
# Rust
#
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "299108772020c103eefacb4de30873d45224e8e0e6c11df7b56ffd11d959e212",
    strip_prefix = "rules_rust-3fac9fe0001d2a829d8ddaf3033b5171c049abdb",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/3fac9fe0001d2a829d8ddaf3033b5171c049abdb.tar.gz",
    ],
)

#git_repository(
#    name = "io_bazel_rules_rust",
#    remote = "https://github.com/sayrer/rules_rust",
#    branch = "rust_proc_macro"
#)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")


#
# Android
#

# Checks the standard environment variables by default
android_sdk_repository(name = "androidsdk")
android_ndk_repository(name = "androidndk")

# Google Maven Repository
GMAVEN_TAG = "20181212-2"
http_archive(
    name = "gmaven_rules",
    strip_prefix = "gmaven_rules-%s" % GMAVEN_TAG,
    url = "https://github.com/bazelbuild/gmaven_rules/archive/%s.tar.gz" % GMAVEN_TAG,
)
load("@gmaven_rules//:gmaven.bzl", "gmaven_rules")

gmaven_rules()