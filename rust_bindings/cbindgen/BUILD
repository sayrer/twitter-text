genrule(
    name = "make_cbindgen_config",
    srcs = [
    	"//rust:twitter_text",
    	"//rust:twitter_text_crate",
 	    	"//rust:highlighter",
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
