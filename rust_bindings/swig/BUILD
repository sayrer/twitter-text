load("@rules_foreign_cc//tools/build_defs:configure.bzl", "configure_make")

configure_make(
    name = "pcre",
    lib_source = "@pcre//:all",
    static_libraries = ["libpcre.a"],
    shared_libraries = ["libpcre.1.dylib"],
)

configure_make(
    name = "swig_cmd",
    lib_source = "@swig//:all",
    binaries = [
        "swig",
    ],
    deps = [
        ":pcre",
    ],
    visibility = [
        "//visibility:public"
    ],
)
