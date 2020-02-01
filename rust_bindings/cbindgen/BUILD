package(default_visibility = ["//visibility:public"])

genrule(
	name = "swig_input",
	srcs = [
		"twitter-text-template.i",
		":twitter-text.h"
	],
	outs = ["twitter-text.i"],
	cmd = "cp $(location twitter-text-template.i) $$(dirname $(location :twitter-text.h))/twitter-text.i"
)

genrule(
    name = "twitter_text_header",
    srcs = [
    	"//rust:twitter_text",
    	":cbindgen.toml"
    ],
    outs = ["twitter-text.h"],
    cmd = "$(location //rust_bindings/cbindgen/bin:cbindgen) $(location cbindgen.toml) -v --output $@ --files $(locations //rust:twitter_text)",
    tools = ["//rust_bindings/cbindgen/bin:cbindgen"],
)

# TODO why did I want to concat this?
genrule(
    name = "concat_config_files",
    srcs = [
        "config_templates/prefix.toml",  # a filegroup with multiple files in it ==> $(locations)
        "config_templates/suffix.toml",   # a genrule with a single output ==> $(location)
    ],
    outs = ["cbindgen.toml"],
    cmd = "cat $(location config_templates/prefix.toml) $(location config_templates/suffix.toml) > $@",
)
