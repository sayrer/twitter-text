
#
# Bazel infrastructure
#
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
http_archive(
    name = "bazel_skylib",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
)
load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
bazel_skylib_workspace()

#
# Rust
#
http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "f33bffd6b779ae5a4f57944e86307f876872b9dbfc07b3d10d0e7f0041f29d5f",
    strip_prefix = "rules_rust-959ba5692cc4e6b803b20018c9eeeadedaa4b637",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/959ba5692cc4e6b803b20018c9eeeadedaa4b637.tar.gz",
    ],
)
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
    sha256 = "33027de68db6a49a352f83808fa9898c4930d39aa6fb0edc6bb3d3eec6e2bc7d",
)
load("@gmaven_rules//:gmaven.bzl", "gmaven_rules")

gmaven_rules()

#
# SWIG
#