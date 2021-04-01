package(default_visibility = ["//visibility:public"])

cc_binary(
  name = "libtwittertext.dylib",
  srcs = [
    ":twitter-text-java.cpp",
    "//rust_bindings/cpp:twitter_text_h",
    "//rust_bindings/cpp:twitter_text_cpp",
  ],
  deps = [
    "//rust_bindings/cpp:twitter_text",
    "//rust_bindings/java/jni:jniheaders",
  ],
  linkstatic = True,
  linkshared = True,
)

SWIG_JAVA_FILES = [
  "Autolinker.java",
  "Entities.java",
  "Entity.java",
  "Extractor.java",
  #"ExtractorString.java",
  #"ExtractorStrings.java",
  "ExtractResult.java",
  "Hit.java",
  "HitHighlighter.java",
  "Hits.java",
  "MentionResult.java",
  "Range.java",
  "RustString.java",
  #"SWIGTYPE_p_ExtractorString.java",
  #"SWIGTYPE_p_std__vectorT_twitter_text__ExtractorString_t.java",
  #"SWIGTYPE_p_rust__Str.java",
  #"SWIGTYPE_p_rust__String.java",
  #"SWIGTYPE_p_rust__cxxbridge04__unsafe_bitcopy_t.java",
  "SWIGTYPE_p_std__shared_ptrT_twitter_text__Configuration_t.java",
  #"SWIGTYPE_p_std__string.java",
  #"SWIGTYPE_p_std__vectorT_std__string_t.java",
  #"SWIGTYPE_p_twitter_text__AutolinkerConfig.java",
  #"SWIGTYPE_p_twitter_text__Configuration.java",
  #"SWIGTYPE_p_twitter_text__RustExtractor.java",
  #"SWIGTYPE_p_twitter_text__RustHitHighlighter.java",
  #"SWIGTYPE_p_twitter_text__RustValidatingExtractor.java",
  #"SWIGTYPE_p_twitter_text__RustValidator.java",
  "Str.java",
  "StringList.java",
  "TwitterText.java",
  "TwitterTextConfiguration.java",
  "TwitterTextJNI.java",
  "TwitterTextParser.java",
  "TwitterTextParseResults.java",
  "ValidatingExtractor.java",
  "Validator.java",
  "WeightedRange.java",
  "WeightedRangeList.java",
]

java_library(
  name = "twittertext_jni",
  srcs = [
    "src/main/java/app/grafica/twittertext/TwitterTextJNILib.java",
  ] + SWIG_JAVA_FILES,
  deps = [
    ":libtwittertext.dylib",
  ],
)

java_library(
    name = "test_deps",
    exports = [
      "@maven//:junit_junit",
      "@maven//:com_fasterxml_jackson_dataformat_jackson_dataformat_yaml",
      "@maven//:com_fasterxml_jackson_core_jackson_databind",
    ],
    data = [
      "//rust_bindings/cpp:config_test_data",
      "//rust/conformance:yaml",      
    ],
)

genrule(
  name = "swig_output",
  srcs = [
      "//rust_bindings:swig_input", 
      "//rust_bindings/cpp:twitter_text_h",
      "//rust_bindings:twitter_text_h",
  ],
  outs = ["twitter-text-java.cpp"] + SWIG_JAVA_FILES,
  cmd = "$(location @swig4//:bin/swig) -v -c++ -Dfinal= -java -package app.grafica.twittertext -o $(location twitter-text-java.cpp) -I$(BINDIR) $(location //rust_bindings:swig_input)",
  tools = ["@swig4//:bin/swig"],
)
