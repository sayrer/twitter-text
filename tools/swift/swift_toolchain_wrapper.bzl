"""Repository rule to create a Swift toolchain with absolute paths."""

def _swift_toolchain_path_impl(repository_ctx):
    """Generate the absolute path to the Swift toolchain."""
    # Get the path to the Swift toolchain
    swift_path = str(repository_ctx.path(Label("@swift_linux_x86_64//:usr")).dirname.dirname)

    repository_ctx.file("BUILD.bazel", """
package(default_visibility = ["//visibility:public"])

exports_files(["swift_root.txt"])
""")

    repository_ctx.file("swift_root.txt", swift_path + "/usr")

swift_toolchain_path = repository_rule(
    implementation = _swift_toolchain_path_impl,
)
