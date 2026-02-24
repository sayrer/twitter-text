"""Module extension for hermetic Swift toolchain."""

load("//tools/swift:swift_toolchain_wrapper.bzl", "swift_toolchain_repo")

def _swift_toolchain_impl(ctx):
    swift_toolchain_repo(name = "hermetic_swift_toolchain")

swift_toolchain_ext = module_extension(
    implementation = _swift_toolchain_impl,
)
