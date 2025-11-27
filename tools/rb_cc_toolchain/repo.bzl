def _llvm_toolchain_impl(rctx):
    # Symlink linux/bin from the linux_repo's clang_file
    linux_clang = rctx.path(
        Label("@%s//:clang_file" % rctx.attr.linux_repo)
    )
    rctx.symlink(linux_clang.dirname, "linux/bin")

    # Symlink macos/bin from the macos_repo's clang_file
    macos_clang = rctx.path(
        Label("@%s//:clang_file" % rctx.attr.macos_repo)
    )
    rctx.symlink(macos_clang.dirname, "macos/bin")

    # Write a simple wrapper that calls ld.lld (Linux) or ld64.lld (macOS)
    rctx.file(
        "clang_lld",
        """#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"

if [[ -x "${DIR}/linux/bin/ld.lld" ]]; then
    exec "${DIR}/linux/bin/ld.lld" "$@"
elif [[ -x "${DIR}/macos/bin/ld64.lld" ]]; then
    exec "${DIR}/macos/bin/ld64.lld" "$@"
else
    echo "clang_lld: no ld.lld or ld64.lld found" >&2
    exit 1
fi
""",
        executable = True,
    )

    # Minimal BUILD file. allow_empty=True so missing dirs won't hard-fail globs.
    rctx.file(
        "BUILD.bazel",
        """
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "linux_all_files",
    srcs = glob(["linux/bin/**"], allow_empty = True)
)

filegroup(
    name = "macos_all_files",
    srcs = glob(["macos/bin/**"], allow_empty = True)
)

exports_files(["clang_lld"])
""",
    )


llvm_toolchain = repository_rule(
    implementation = _llvm_toolchain_impl,
    attrs = {
        "linux_repo": attr.string(mandatory = True),
        "macos_repo": attr.string(mandatory = True),
    },
)
