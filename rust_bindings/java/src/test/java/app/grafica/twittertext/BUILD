load("@rules_java//java:defs.bzl", "java_test")

java_test(
    name = "configuration",
    srcs = ["TestConfiguration.java"],
    test_class = "app.grafica.twittertext.TestConfiguration",
    deps = [
        "//rust_bindings/java:twittertext_jni",
        "//rust_bindings/java:test_deps",
    ],
)

java_test(
    name = "autolinker",
    srcs = ["TestAutolinker.java"],
    test_class = "app.grafica.twittertext.TestAutolinker",
    deps = [
        ":yaml",
        "//rust_bindings/java:twittertext_jni",
        "//rust_bindings/java:test_deps",
    ],
)

java_test(
    name = "extractor",
    srcs = ["TestExtractor.java"],
    test_class = "app.grafica.twittertext.TestExtractor",
    deps = [
        ":yaml",
        "//rust_bindings/java:twittertext_jni",
        "//rust_bindings/java:test_deps",
    ],
)

java_test(
    name = "highlighter",
    srcs = ["TestHitHighlighter.java"],
    test_class = "app.grafica.twittertext.TestHitHighlighter",
    deps = [
        ":yaml",
        "//rust_bindings/java:twittertext_jni",
        "//rust_bindings/java:test_deps",
    ],
)

java_test(
    name = "validator",
    srcs = ["TestValidator.java"],
    test_class = "app.grafica.twittertext.TestValidator",
    deps = [
        ":yaml",
        "//rust_bindings/java:twittertext_jni",
        "//rust_bindings/java:test_deps",
    ],
)

java_library(
    name = "yaml",
    srcs = ["Yaml.java"],
    deps = [
      "//rust_bindings/java:twittertext_jni",
      "//rust_bindings/java:test_deps",
    ]
)