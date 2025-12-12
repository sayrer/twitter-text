"""Configure hermetic Swift toolchain for rules_swift."""

def _hermetic_swift_impl(repository_ctx):
    """Create a repository that wraps the hermetic Swift toolchain."""

    # Get the Swift toolchain path from the downloaded archive
    swift_repo = repository_ctx.attr.swift_repo

    # Create a BUILD file that exports the Swift binaries
    repository_ctx.file("BUILD.bazel", """
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "swift_bin",
    srcs = ["@{swift_repo}//:usr/bin/swift"],
)

filegroup(
    name = "swiftc_bin",
    srcs = ["@{swift_repo}//:usr/bin/swiftc"],
)
""".format(swift_repo = swift_repo))

    # Create a script that adds Swift to PATH
    repository_ctx.file("setup_path.sh", """#!/bin/bash
export PATH="$(dirname $(realpath $(readlink -f @{swift_repo}//:usr/bin/swift))):$PATH"
""".format(swift_repo = swift_repo), executable = True)

hermetic_swift = repository_rule(
    implementation = _hermetic_swift_impl,
    attrs = {
        "swift_repo": attr.string(
            mandatory = True,
            doc = "The name of the Swift toolchain repository",
        ),
    },
)
