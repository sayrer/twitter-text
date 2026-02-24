"""Repository rule to create a Swift toolchain with the correct repo path."""

def _swift_toolchain_repo_impl(repository_ctx):
    """Generate a swift_toolchain BUILD file with the correct root path."""
    # Resolve the canonical path to the Swift toolchain repo
    swift_repo_path = str(repository_ctx.path(Label("@swift_linux_x86_64//:BUILD.bazel")).dirname)

    # Extract the canonical repo directory name (e.g., "+http_archive+swift_linux_x86_64")
    # by finding the "external/" segment in the absolute path
    parts = swift_repo_path.split("/external/")
    repo_dir = parts[-1]

    root = "external/{}/usr".format(repo_dir)

    repository_ctx.file("BUILD.bazel", """
load("@rules_swift//swift/toolchains:swift_toolchain.bzl", "swift_toolchain")

package(default_visibility = ["//visibility:public"])

swift_toolchain(
    name = "hermetic_swift_linux_x86_64",
    arch = "x86_64",
    features = [
        "swift.use_module_wrap",
    ],
    os = "linux",
    root = "{root}",
    toolchain_files = "@swift_linux_x86_64//:usr",
    version_file = "@swift_linux_x86_64//:version_file",
)

toolchain(
    name = "hermetic_swift_linux_x86_64_toolchain",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
    toolchain = ":hermetic_swift_linux_x86_64",
    toolchain_type = "@rules_swift//toolchains:toolchain_type",
)
""".format(root = root))

swift_toolchain_repo = repository_rule(
    implementation = _swift_toolchain_repo_impl,
)
