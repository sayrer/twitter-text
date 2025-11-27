def _rust_toolchain_repo_impl(rctx):
    # write the rustflags config file
    rctx.file("rustflags", """
[env]
RUSTFLAGS = "-C linker=%s/clang_lld"
""" % rctx.path(Label("@%s//:clang_lld" % rctx.attr.llvm_repo)))

    # generate BUILD that instantiates a rust toolchain using this config
    rctx.file("BUILD.bazel", """
load("@rules_rust//rust:defs.bzl", "rust_toolchain_repository")

rust_toolchain_repository(
    name = "toolchain",
    version = "%s",
    patches = [],
    patch_tool = "@//:false",
    extra_rustflags = ["-C", "linker=%s"],
)
""" % (rctx.attr.rust_version, "clang_lld"))

rust_toolchain_repo = repository_rule(
    implementation = _rust_toolchain_repo_impl,
    attrs = {
        "rust_version": attr.string(mandatory = True),
        "llvm_repo": attr.string(mandatory = True),
    },
)
