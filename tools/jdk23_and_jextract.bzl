load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Tags used from MODULE.bazel:
#   jdk23_ext.add_jdk(...)
#   jdk23_ext.add_jextract(...)

JdkTag = tag_class(
    attrs = {
        "name": attr.string(),
        "urls": attr.string_list(),
        "sha256": attr.string(),
        "strip_prefix": attr.string(),
        "os": attr.string(),
        "cpu": attr.string(),
    },
)

JextractTag = tag_class(
    attrs = {
        "name": attr.string(),
        "urls": attr.string_list(),
        "sha256": attr.string(),
        "strip_prefix": attr.string(),
        "os": attr.string(),
        "cpu": attr.string(),
    },
)

def _create_toolchains_repo_impl(repo_ctx):
    # This repo will just declare toolchain_type + toolchain objects that
    # point at the external JDK/jextract repos we created with http_archive.
    repo_ctx.file("WORKSPACE", "workspace(name = \"jdk23_toolchains\")\n")
    repo_ctx.file("BUILD.bazel", """
load("@rules_java//java:defs.bzl", "java_runtime")
load("@rules_java//toolchains:default_java_toolchain.bzl", "default_java_toolchain")
load("@twitter_text_rs//tools/jextract:defs.bzl", "jextract_toolchain")

# Types
toolchain_type(name = "jextract_toolchain_type")

# ---------- macOS arm64 ----------

default_java_toolchain(
    name = "macos_arm64_java_toolchain_definition",
    source_version = "23",
    target_version = "23",
)

toolchain(
    name = "macos_arm64_java_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:toolchain_type",
    toolchain = ":macos_arm64_java_toolchain_definition",
    exec_compatible_with = [
        "@platforms//os:macos",
        "@platforms//cpu:arm64",
    ],
)

toolchain(
    name = "macos_arm64_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:runtime_toolchain_type",
    toolchain = "@jdk23_macos_arm64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:macos",
        "@platforms//cpu:arm64",
    ],
)

toolchain(
    name = "macos_arm64_bootstrap_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:bootstrap_runtime_toolchain_type",
    toolchain = "@jdk23_macos_arm64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:macos",
        "@platforms//cpu:arm64",
    ],
)

jextract_toolchain(
    name = "macos_arm64_jextract_impl",
    jextract = "@jextract_macos_arm64_repo//:jextract_bin",
)

toolchain(
    name = "macos_arm64_jextract",
    toolchain_type = ":jextract_toolchain_type",
    toolchain = ":macos_arm64_jextract_impl",
    exec_compatible_with = [
        "@platforms//os:macos",
        "@platforms//cpu:arm64",
    ],
)

# ---------- Linux x86_64 ----------

default_java_toolchain(
    name = "linux_x86_64_java_toolchain_definition",
    source_version = "23",
    target_version = "23",
)

toolchain(
    name = "linux_x86_64_java_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:toolchain_type",
    toolchain = ":linux_x86_64_java_toolchain_definition",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

toolchain(
    name = "linux_x86_64_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:runtime_toolchain_type",
    toolchain = "@jdk23_linux_x86_64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

toolchain(
    name = "linux_x86_64_bootstrap_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:bootstrap_runtime_toolchain_type",
    toolchain = "@jdk23_linux_x86_64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

jextract_toolchain(
    name = "linux_x86_64_jextract_impl",
    jextract = "@jextract_linux_x86_64_repo//:jextract_bin",
)

toolchain(
    name = "linux_x86_64_jextract",
    toolchain_type = ":jextract_toolchain_type",
    toolchain = ":linux_x86_64_jextract_impl",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

# ---------- Linux arm64 ----------

default_java_toolchain(
    name = "linux_arm64_java_toolchain_definition",
    source_version = "23",
    target_version = "23",
)

toolchain(
    name = "linux_arm64_java_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:toolchain_type",
    toolchain = ":linux_arm64_java_toolchain_definition",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:arm64",
    ],
)

toolchain(
    name = "linux_arm64_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:runtime_toolchain_type",
    toolchain = "@jdk23_linux_arm64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:arm64",
    ],
)

toolchain(
    name = "linux_arm64_bootstrap_jdk_toolchain",
    toolchain_type = "@@bazel_tools//tools/jdk:bootstrap_runtime_toolchain_type",
    toolchain = "@jdk23_linux_arm64_repo//:runtime",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:arm64",
    ],
)

jextract_toolchain(
    name = "linux_arm64_jextract_impl",
    jextract = "@jextract_linux_arm64_repo//:jextract_bin",
)

toolchain(
    name = "linux_arm64_jextract",
    toolchain_type = ":jextract_toolchain_type",
    toolchain = ":linux_arm64_jextract_impl",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:arm64",
    ],
)
""")

_create_toolchains_repo = repository_rule(
    implementation = _create_toolchains_repo_impl,
)

def _jdk23_impl(ctx):
    # Create one repo per JDK
    for mod in ctx.modules:
        for t in mod.tags.add_jdk:
            # macOS JDKs have a Contents/Home subdirectory
            java_home = "Contents/Home" if t.os == "macos" else "."
            java_binary = "Contents/Home/bin/java" if t.os == "macos" else "bin/java"
            http_archive(
                name = t.name + "_repo",
                urls = t.urls,
                sha256 = t.sha256,
                strip_prefix = t.strip_prefix,
                build_file_content = """
load("@rules_java//java:defs.bzl", "java_runtime")

filegroup(
    name = "jdk_files",
    srcs = glob(
        ["**"],
        exclude = ["**/*src.zip"],
    ),
)

java_runtime(
    name = "runtime",
    java_home = "{java_home}",
    java = "{java_binary}",
    srcs = [":jdk_files"],
    visibility = ["//visibility:public"],
)
""".format(java_home = java_home, java_binary = java_binary),
            )

        # Create one repo per jextract
        for t in mod.tags.add_jextract:
            http_archive(
                name = t.name + "_repo",
                urls = t.urls,
                sha256 = t.sha256,
                strip_prefix = t.strip_prefix,
                build_file_content = """
filegroup(
    name = "jextract_bin",
    srcs = ["bin/jextract"],
    visibility = ["//visibility:public"],
)
""",
            )

    # Create the central toolchains repo
    _create_toolchains_repo(name = "jdk23_toolchains")

jdk23_ext = module_extension(
    implementation = _jdk23_impl,
    tag_classes = {
        "add_jdk": JdkTag,
        "add_jextract": JextractTag,
    },
)
