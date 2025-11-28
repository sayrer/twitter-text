###############################################
# Hermetic C/C++ Toolchain Provider (Bazel 7+)
###############################################

HermeticCppToolchainInfo = provider(fields = {
    "compiler": "clang binary",
    "linker": "lld or wrapper",
    "ar": "ar tool",
    "nm": "nm tool",
    "strip": "strip tool",
    "objcopy": "objcopy tool",
    "all_files": "runtime filegroup",
})

def _impl(ctx):
    return [
        platform_common.ToolchainInfo(
            cpp = HermeticCppToolchainInfo(
                compiler = ctx.attr.compiler,
                linker = ctx.attr.linker,
                ar = ctx.attr.ar,
                nm = ctx.attr.nm,
                strip = ctx.attr.strip,
                objcopy = ctx.attr.objcopy,
                all_files = ctx.attr.all_files,
            )
        )
    ]

hermetic_cc_toolchain = rule(
    implementation = _impl,
    attrs = {
        "compiler": attr.label(allow_single_file = True),
        "linker": attr.label(allow_single_file = True),
        "ar": attr.label(allow_single_file = True),
        "nm": attr.label(allow_single_file = True),
        "strip": attr.label(allow_single_file = True),
        "objcopy": attr.label(allow_single_file = True),
        "all_files": attr.label(),
    },
)
