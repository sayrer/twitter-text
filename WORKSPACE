
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
    sha256 = "618e791454692b58004fcfc96bb48470eaf29304d8268b26ce0e16e87869a76b",
    strip_prefix = "rules_rust-50f45841dc68f7355113ada4d61fecabb528b38f",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/50f45841dc68f7355113ada4d61fecabb528b38f.tar.gz",
    ],
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories(edition = "2018")
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

#
# SWIG
#
http_archive(
    name = "io_tweag_rules_nixpkgs",
    sha256 = "aca86baa64174478c57f74ed09d5c2313113abe94aa3af030486d1b14032d3ed",
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

#nixpkgs_cc_configure(repository = "@nixpkgs//:default.nix")

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
git_repository(
    name = "bazelruby_rules_ruby",
    remote = "https://github.com/bazelruby/rules_ruby.git",
    commit = "79da596d0d26597dcb147b3b4159d4a4d76f2401"
)

load(
    "@bazelruby_rules_ruby//ruby:deps.bzl",
    "rules_ruby_dependencies",
    "rules_ruby_select_sdk",
)

rules_ruby_dependencies()
rules_ruby_select_sdk(version = "2.6.6")


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
