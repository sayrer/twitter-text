JextractToolchainInfo = provider(fields = ["jextract"])

def _jextract_toolchain_impl(ctx):
    return [JextractToolchainInfo(jextract = ctx.executable.jextract)]

jextract_toolchain = rule(
    implementation = _jextract_toolchain_impl,
    attrs = {
        "jextract": attr.label(
            executable = True,
            cfg = "exec",
            allow_single_file = True,
            mandatory = True,
        ),
    },
)

def _jextract_impl(ctx):
    tc = ctx.toolchains["@jdk23_toolchains//:jextract_toolchain_type"][JextractToolchainInfo]
    out = ctx.outputs.out
    args = list(ctx.attr.args)
    args.extend(["-o", out.path])
    for hdr in ctx.files.hdrs:
        args.append(hdr.path)

    ctx.actions.run(
        executable = tc.jextract,
        arguments = args,
        inputs = ctx.files.hdrs,
        tools = [tc.jextract],
        outputs = [out],
        progress_message = "jextract %s" % out.path,
    )

jextract = rule(
    implementation = _jextract_impl,
    attrs = {
        "hdrs": attr.label_list(allow_files = [".h"]),
        "args": attr.string_list(),
        "out": attr.output(mandatory = True),
    },
    toolchains = ["@jdk23_toolchains//:jextract_toolchain_type"],
)
