load("@rules_rust//rust:defs.bzl", "rust_library")

def rust_cxx_bridge(name, src, deps = [], visibility = None):
    native.genrule(
        name = name + "_codegen",
        srcs = [src],
        outs = [
            name + ".h",
            name + ".cc",
        ],
        tools = [
            "@cxxbridge-cmd//:cxxbridge-cmd",
        ],
        cmd = """
            SRC=$(location {src})
            $(location @cxxbridge-cmd//:cxxbridge-cmd) $$SRC --header > $(@D)/{name}.h
            $(location @cxxbridge-cmd//:cxxbridge-cmd) $$SRC > $(@D)/{name}.cc
        """.format(src=src, name=name),
        visibility = visibility,
    )

    rust_library(
        name = name + "_rlib",
        srcs = [src],
        deps = deps,
        edition = "2021",
        data = [":" + name + "_codegen"],
        visibility = visibility,
    )

    native.cc_library(
        name = name,
        hdrs = [ name + ".h" ],
        srcs = [ name + ".cc" ],
        deps = deps,
        include_prefix = "",
        strip_include_prefix = ".",
        visibility = visibility,
    )
