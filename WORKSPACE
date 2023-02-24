workspace(name = "twitter_text")

#
# Bazel infrastructure
#
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
http_archive(
    name = "bazel_skylib",
    sha256 = "b8a1527901774180afc798aeb28c4634bdccf19c4d98e7bdd1ce79d1fe9aaad7",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.1/bazel-skylib-1.4.1.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.1/bazel-skylib-1.4.1.tar.gz",
    ],
)
load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
bazel_skylib_workspace()

#
# Rust
#
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_rust",
    sha256 = "2466e5b2514772e84f9009010797b9cd4b51c1e6445bbd5b5e24848d90e6fb2e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.18.0/rules_rust-v0.18.0.tar.gz"],
)
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains()

#
# SWIG
#
http_archive(
    name = "io_tweag_rules_nixpkgs",
    #sha256 = "4fe458cb27708b7319b04a420c9b39022eb994624325dfb0f8b903ed3456c491",
    strip_prefix = "rules_nixpkgs-dc24090573d74adcf38730422941fd69b87682c7",
    urls = ["https://github.com/tweag/rules_nixpkgs/archive/dc24090573d74adcf38730422941fd69b87682c7.tar.gz"],
)
load("@io_tweag_rules_nixpkgs//nixpkgs:repositories.bzl", "rules_nixpkgs_dependencies")
rules_nixpkgs_dependencies()

load("@io_tweag_rules_nixpkgs//nixpkgs:nixpkgs.bzl", "nixpkgs_git_repository", "nixpkgs_package", "nixpkgs_cc_configure")

nixpkgs_git_repository(
    name = "nixpkgs",
    revision = "20.03",
    sha256 = "f21ca8bc4c8f848a351232e09f3a58d280c05323173a78a5a6013937fb05c6fe"
)

nixpkgs_package(
    name = "swig4",
    repositories = { "nixpkgs": "@nixpkgs//:default.nix" },
)

#
# Python
# 
http_archive(
    name = "rules_python",
    sha256 = "8821ca29f45fcd25fa7424118c5cad0abc9b57b2c0a2b482f815b70d09871f5c",
    strip_prefix = "rules_python-0f8183b1cfa7e8afebfeeec5bcad75deb846613a",
    urls = [
        "https://github.com/bazelbuild/rules_python/archive/0f8183b1cfa7e8afebfeeec5bcad75deb846613a.tar.gz",
    ],
)
load("@rules_python//python:repositories.bzl", "py_repositories")
py_repositories()

#
# Use pip for test dependencies
#
load("@rules_python//python:pip.bzl", "pip3_import", "pip_repositories")
pip_repositories()

pip3_import(
    name = "rust_python_test_deps",
    requirements = "//rust_bindings/python/test:requirements.txt",
)
load("@rust_python_test_deps//:requirements.bzl", _rust_python_test_deps_install = "pip_install")
_rust_python_test_deps_install()

#
# SWIG needs development headers
#
nixpkgs_package(
    name = "python3Full",
    repositories = { "nixpkgs": "@nixpkgs//:default.nix" },
)

#
# Ruby
# 
#———————————————————————————————————————————————————————————————————————
# To get the latest ruby rules, grab the 'master' branch.
#———————————————————————————————————————————————————————————————————————

git_repository(
    name = "bazelruby_rules_ruby",
    remote = "https://github.com/bazelruby/rules_ruby.git",
    branch = "0a3275b235dd4093a2a44e2f08d96a9f07ecbe0a"
)

load(
    "@bazelruby_rules_ruby//ruby:deps.bzl",
    "rules_ruby_dependencies",
    "rules_ruby_select_sdk",
)

rules_ruby_dependencies()
rules_ruby_select_sdk(version = "host") #fixme

load(
    "@bazelruby_rules_ruby//ruby:defs.bzl",
    "ruby_bundle",
)

# Run 'bundle lock --update' to update the lock file without installing anything.
ruby_bundle(
    name = "bundle",
    excludes = {
        "mini_portile": ["test/**/*"],
    },
    gemfile = "//:Gemfile",
    gemfile_lock = "//:Gemfile.lock",
)

#
# C++
#
http_archive(
    name = "gtest",
    url = "https://github.com/google/googletest/archive/release-1.7.0.zip",
    sha256 = "b58cb7547a28b2c718d1e38aee18a3659c9e3ff52440297e965f5edffe34b6d0",
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

#
# Java
#
RULES_JVM_EXTERNAL_TAG = "3.3"
RULES_JVM_EXTERNAL_SHA = "d85951a92c0908c80bd8551002d66cb23c3434409c814179c0ff026b53544dab"

http_archive(
    name = "rules_jvm_external",
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    sha256 = RULES_JVM_EXTERNAL_SHA,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    artifacts = [
        "junit:junit:4.12",
        "com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.9.1",
        "com.fasterxml.jackson.core:jackson-databind:2.8.7",
        "com.google.code.findbugs:jsr305:2.0.1",
    ],
    repositories = [
        "https://jcenter.bintray.com/",
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)

#
# Android
#
# Checks the standard environment variables by default
#android_sdk_repository(name = "androidsdk")
#android_ndk_repository(name = "androidndk")
