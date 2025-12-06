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
            # Determine the JDK repo name based on platform
            jdk_repo_map = {
                ("macos", "arm64"): ("@jdk23_macos_arm64_repo//:runtime", "jdk23_macos_arm64_repo"),
                ("linux", "x86_64"): ("@jdk23_linux_x86_64_repo//:runtime", "jdk23_linux_x86_64_repo"),
                ("linux", "arm64"): ("@jdk23_linux_arm64_repo//:runtime", "jdk23_linux_arm64_repo"),
            }
            jdk_target, jdk_target_short = jdk_repo_map.get((t.os, t.cpu), ("@jdk23_macos_arm64_repo//:runtime", "jdk23_macos_arm64_repo"))

            # Determine the jextract repo name based on platform
            jextract_repo_name = "jextract_%s_%s" % (t.os, t.cpu)

            build_content = """
genrule(
    name = "jextract_wrapper_gen",
    outs = ["jextract_wrapper.sh"],
    cmd = \"\"\"cat > $@ << 'EOF'
#!/bin/bash
set -e
# Find the runfiles directory and JDK
SCRIPT_DIR="$$(cd "$$(dirname "$$0")" && pwd)"
RUNFILES="$$SCRIPT_DIR/../.."
echo "DEBUG: SCRIPT_DIR=$$SCRIPT_DIR" >&2
echo "DEBUG: RUNFILES=$$RUNFILES" >&2
echo "DEBUG: Looking for JDK in $$RUNFILES" >&2
JDK_BIN="$$(find "$$RUNFILES" -path "*%s/bin/java" -o -path "*%s/Contents/Home/bin/java" 2>/dev/null | head -1)"
if [[ -z "$$JDK_BIN" ]]; then
  echo "Error: Could not find JDK java binary" >&2
  echo "DEBUG: Contents of RUNFILES:" >&2
  ls -la "$$RUNFILES" >&2 || true
  exit 1
fi
export JAVA_HOME="$$(dirname "$$(dirname "$$JDK_BIN")")"
echo "DEBUG: JAVA_HOME=$$JAVA_HOME" >&2
# Look for jextract in the runfiles directory (platform-specific)
JEXTRACT_BIN="$$SCRIPT_DIR/jextract_bin.runfiles/+jdk23_ext+%s_repo/bin/jextract"
echo "DEBUG: Looking for JEXTRACT_BIN at $$JEXTRACT_BIN" >&2
if [[ ! -f "$$JEXTRACT_BIN" ]]; then
  echo "Error: Could not find jextract binary at $$JEXTRACT_BIN" >&2
  echo "DEBUG: Contents of jextract_bin.runfiles:" >&2
  ls -la "$$SCRIPT_DIR/jextract_bin.runfiles/" 2>&1 | head -20 >&2 || true
  echo "DEBUG: Contents of %s_repo:" >&2
  ls -la "$$SCRIPT_DIR/jextract_bin.runfiles/+jdk23_ext+%s_repo/" 2>&1 | head -20 >&2 || true
  exit 1
fi
exec "$$JEXTRACT_BIN" "$$@"
EOF
chmod +x $@
\"\"\",
)

sh_binary(
    name = "jextract_bin",
    srcs = [":jextract_wrapper.sh"],
    data = ["%s"] + glob(["**"]),
    visibility = ["//visibility:public"],
)
""" % (jdk_target_short, jdk_target_short, jextract_repo_name, jextract_repo_name, jextract_repo_name, jdk_target)

            http_archive(
                name = t.name + "_repo",
                urls = t.urls,
                sha256 = t.sha256,
                strip_prefix = t.strip_prefix,
                build_file_content = build_content,
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
