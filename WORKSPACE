
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
#http_archive(
#    name = "io_bazel_rules_rust",
#    sha256 = "f33bffd6b779ae5a4f57944e86307f876872b9dbfc07b3d10d0e7f0041f29d5f",
#    strip_prefix = "rules_rust-959ba5692cc4e6b803b20018c9eeeadedaa4b637",
#    urls = [
#        "https://github.com/bazelbuild/rules_rust/archive/959ba5692cc4e6b803b20018c9eeeadedaa4b637.tar.gz",
#    ],
#)
local_repository(
    name = "io_bazel_rules_rust",
    path = "/Users/sayrer/github/sayrer/rules_rust_master/rules_rust",
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories(edition = "2018")
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
http_archive(
    name = "io_tweag_rules_nixpkgs",
    #sha256 = "f5af641e16fcff5b24f1a9ba5d93cab5ad26500271df59ede344f1a56fc3b17d",
    strip_prefix = "rules_nixpkgs-adfe991ad7fd41fcdbeaeabf33ea061d9b435c97",
    urls = ["https://github.com/tweag/rules_nixpkgs/archive/adfe991ad7fd41fcdbeaeabf33ea061d9b435c97.tar.gz"],
)
load("@io_tweag_rules_nixpkgs//nixpkgs:repositories.bzl", "rules_nixpkgs_dependencies")
rules_nixpkgs_dependencies()

load("@io_tweag_rules_nixpkgs//nixpkgs:nixpkgs.bzl", "nixpkgs_git_repository", "nixpkgs_package", "nixpkgs_cc_configure")

nixpkgs_git_repository(
    name = "nixpkgs",
    revision = "19.09",
    sha256 = "ac23e3fad10035dc9fc2882997cd12e316785438c15cf103db0fe0e267ab7f84"
)

#nixpkgs_cc_configure(repository = "@nixpkgs//:default.nix")

nixpkgs_package(
    name = "swig",
    repositories = { "nixpkgs": "@nixpkgs//:default.nix" },
)

#
# Python
#
http_archive(
    name = "rules_python",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.0.1/rules_python-0.0.1.tar.gz",
    sha256 = "aa96a691d3a8177f3215b14b0edc9641787abaaa30363a080165d06ab65e1161",
)
load("@rules_python//python:repositories.bzl", "py_repositories")
py_repositories()

nixpkgs_package(
    name = "python3Full",
    repositories = { "nixpkgs": "@nixpkgs//:default.nix" },
)

#
# C++
#
http_archive(
    name = "gtest",
    url = "https://github.com/google/googletest/archive/release-1.7.0.zip",
    #sha256 = "b58cb7547a28b2c718d1e38aee18a3659c9e3ff52440297e965f5edffe34b6d0",
    build_file = "@//:gtest.BUILD",
    strip_prefix = "googletest-release-1.7.0",
)

http_archive(
    name = "com_github_jbeder_yaml_cpp",
    sha256 = "bf991eb70c1005cf34d38e60f66392f4461c5519d594a9790263eb62518f133c",
    strip_prefix = "yaml-cpp-33316d531bd9032d66f5bcc3ba1fd114a4ab0e1c",
    urls = [
        "https://github.com/jbeder/yaml-cpp/archive/33316d531bd9032d66f5bcc3ba1fd114a4ab0e1c.tar.gz",
    ],
)
