load("@rust_python_test_deps//:requirements.bzl", "requirement")

TEST_DEPS = [
    requirement("pytest"),
    requirement("attrs"),
    requirement("iniconfig"),
    requirement("more-itertools"),
    requirement("packaging"),
    requirement("pluggy"),
    requirement("py"),
    requirement("toml"),
    requirement("importlib-metadata"),
    requirement("wcwidth"),
    requirement("PyYaml"),
    "//rust_bindings/python:twitter_text",
]

py_test(
    name = "test_autolinker",
    srcs = ["test_autolinker.py"],
    deps = TEST_DEPS,
    imports =[".."],
    python_version = "PY3",
    srcs_version = "PY3",
    data = [
        "//rust/conformance:yaml"
    ],
)

py_test(
    name = "test_configuration",
    srcs = ["test_configuration.py"],
    deps = TEST_DEPS,
    imports =[".."],
    python_version = "PY3",
    srcs_version = "PY3",
    data = [
        "//rust_bindings/cpp:config_test_data",
        "//rust/conformance:yaml",
    ],
)

py_test(
    name = "test_extractor",
    srcs = ["test_extractor.py"],
    deps = TEST_DEPS,
    imports =[".."],
    python_version = "PY3",
    srcs_version = "PY3",
    data = [
        "//rust/conformance:yaml",
    ],
)

py_test(
    name = "test_hithighlighter",
    srcs = ["test_hithighlighter.py"],
    deps = TEST_DEPS,
    imports =[".."],
    python_version = "PY3",
    srcs_version = "PY3",
    data = [
        "//rust/conformance:yaml",
    ],
)

py_test(
    name = "test_validator",
    srcs = ["test_validator.py"],
    deps = TEST_DEPS,
    imports =[".."],
    python_version = "PY3",
    srcs_version = "PY3",
    data = [
        "//rust/conformance:yaml",
    ],
)